use candid::Deserialize;
use std::cell::RefCell;

use serde::Serialize;

mod guards;
use guards::assert_owner;

// Candid
use candid::{candid_method, Principal};

use ic_cdk::{
    api::{self},
    init, post_upgrade, pre_upgrade, query, update,
};

// KDTree
use kiddo::float::{distance::squared_euclidean, kdtree::KdTree};
mod datatype;
use datatype::{PlainDoc, VecDoc, VecQuery};
mod hash;
use hash::hash;

const BUCKET_SIZE: usize = 32;
const EMBEDDING_SIZE: usize = 768;
pub type Tree = KdTree<f32, u64, EMBEDDING_SIZE, BUCKET_SIZE, u16>;
use std::collections::HashMap;
pub type PlainMap = HashMap<u64, PlainDoc>;

// Stable Structure
use ic_stable_structures::{writer::Writer, Memory as _, StableVec};
mod memory;
use memory::Memory;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub owner: Option<Principal>,
    pub controller_canister: Option<Principal>,

    #[serde(skip, default = "init_stable_vec_content")]
    stable_vec_content: StableVec<VecDoc, Memory>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            owner: None,
            controller_canister: None,
            stable_vec_content: init_stable_vec_content(),
        }
    }
}

// Mutable global state
thread_local! {
    // persistent state
    static STATE: RefCell<State> = RefCell::default();

    // volatile state
    static TREE: RefCell<Tree> = RefCell::new(KdTree::new());
    static PLAIN_MAP: RefCell<PlainMap> = RefCell::new(HashMap::new());
}

fn init_stable_vec_content() -> StableVec<VecDoc, Memory> {
    StableVec::init(memory::get_stable_vec_content_memory())
        .expect("call to init_stable_vec_content fails")
}

// Vector DB main functions
#[update]
#[candid_method(update)]
pub fn add(doc: VecDoc) {
    let mut embeddings = doc.embeddings.clone();
    embeddings.resize(EMBEDDING_SIZE, 0.0);

    let query: &[f32; EMBEDDING_SIZE] = &embeddings.try_into().unwrap();
    let plain_doc = PlainDoc {
        content: doc.content.to_owned(),
    };

    let id = hash(&plain_doc);

    // Update KDTree and PlainMap
    PLAIN_MAP.with(|plain_map| plain_map.borrow_mut().insert(id, plain_doc));
    TREE.with(|tree| tree.borrow_mut().add(query, id));

    // Add to stable vector content
    STATE.with(|state| {
        state
            .borrow_mut()
            .stable_vec_content
            .push(&doc)
            .expect("call to stable_vec_content.push fails")
    });
}

#[query]
#[candid_method(query)]
pub fn search(query: VecQuery, k: usize) -> Option<Vec<PlainDoc>> {
    let mut query: Vec<f32> = match query {
        VecQuery::Embeddings(q) => q.to_owned(),
    };
    query.resize(EMBEDDING_SIZE, 0.0);

    let query: &[f32; EMBEDDING_SIZE] = &query.try_into().unwrap();
    let neighbors = TREE.with(|tree| tree.borrow().nearest_n(query, k, &squared_euclidean));
    let plain_map = PLAIN_MAP.with(|plain_map| plain_map.borrow().clone());

    let mut result: Vec<PlainDoc> = vec![];
    for neighbor in &neighbors {
        let doc = plain_map.get(&neighbor.item);
        if let Some(document) = doc {
            result.push(document.to_owned());
        }
    }

    Some(result)
}

#[update]
#[candid_method(update)]
pub fn delete(doc: VecDoc) {
    let mut embeddings = doc.embeddings.clone();
    embeddings.resize(EMBEDDING_SIZE, 0.0);

    let query: &[f32; EMBEDDING_SIZE] = &embeddings.try_into().unwrap();
    let id = hash(&PlainDoc {
        content: doc.content.to_owned(),
    });

    TREE.with(|tree| tree.borrow_mut().remove(query, id));
    PLAIN_MAP.with(|plain_map| plain_map.borrow_mut().remove(&id));
}

pub fn init_index() {
    // let data_vec: Vec<(u64, PlainDoc)> = resource
    //     .docs
    //     .iter()
    //     .map(|resource| PlainDoc {
    //         id: resource.id.to_owned(),
    //         title: resource.title.to_owned(),
    //         url: resource.url.to_owned(),
    //     })
    //     .map(|document| (hash(&document), document))
    //     .collect();

    // let plain_map: HashMap<u64, PlainDoc> = data_vec.clone().into_iter().collect();
    // let mut tree: Tree = KdTree::new();

    // resource
    //     .docs
    //     .iter()
    //     .zip(data_vec.iter())
    //     .for_each(|(resource, data)| {
    //         let mut embeddings = resource.embeddings.clone();
    //         embeddings.resize(EMBEDDING_SIZE, 0.0);

    //         let query: &[f32; EMBEDDING_SIZE] = &embeddings.try_into().unwrap();
    //         // "item" holds the position of the document in "data"
    //         tree.add(query, data.0);
    //     });
}

#[query]
#[candid_method(query)]
pub fn size() -> usize {
    return PLAIN_MAP.with(|plain_map| plain_map.borrow().len());
}

// ---------------------- Supporting Functions ----------------------
// Controller canister must be created with principal
#[init]
#[candid_method(init)]
fn init(owner: Option<Principal>, controller_canister: Option<Principal>) {
    let my_owner: Principal = owner.unwrap_or_else(|| api::caller());
    STATE.with(|state| {
        *state.borrow_mut() = State {
            owner: Some(my_owner),
            controller_canister: controller_canister,
            stable_vec_content: init_stable_vec_content(),
        };
    });
}

#[query]
#[candid_method(query)]
pub fn get_owner() -> Option<Principal> {
    STATE.with(|state| (*state.borrow()).owner)
}

#[query]
#[candid_method(query)]
pub fn get_controller_canister() -> Option<Principal> {
    STATE.with(|state| (*state.borrow()).controller_canister)
}

#[update(guard = "assert_owner")]
#[candid_method(update)]
pub fn update_owner(new_owner: Principal) {
    STATE.with(|state| {
        state.borrow_mut().owner = Some(new_owner);
    });
}

// ---------------------- Canister upgrade process ----------------------
#[pre_upgrade]
fn pre_upgrade() {
    // Serialize the state.
    // This example is using CBOR, but you can use any data format you like.
    let mut state_bytes = vec![];
    STATE
        .with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    // Write the length of the serialized bytes to memory, followed by the
    // by the bytes themselves.
    let len = state_bytes.len() as u32;
    let mut memory = memory::get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    let memory = memory::get_upgrades_memory();

    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    // Deserialize and set the state.
    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    STATE.with(|s| *s.borrow_mut() = state);
}

// ---------------------- Custom getrandom ----------------------
use getrandom::register_custom_getrandom;

fn custom_getrandom(_buf: &mut [u8]) -> Result<(), getrandom::Error> {
    // TODO get some randomness
    return Ok(());
}

register_custom_getrandom!(custom_getrandom);

// ---------------------- Candid declarations did file generator ----------------------
#[cfg(test)]
mod tests {
    use crate::datatype::{PlainDoc, VecDoc, VecQuery};
    use candid::{export_service, Principal};

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::current_dir().unwrap());
        export_service!();
        write(dir.join("arcmindvectordb.did"), __export_service()).expect("Write failed.");
    }
}
