# Arcmind Vector DB

Arcmind Vector DB is a high-performance, flexible, and ergonomic vector similarity search database for the [Internet Computer](https://internetcomputer.org). It is designed to be a general-purpose vector similarity search database that can be used for a wide range of AI-powered applications, including recommendation systems, search engines, [Retrieval Augmented Generation](https://arxiv.org/abs/2005.11401) (RAG), and long-term memory of Autonomous AI agents like [ArcMind AI](https://github.com/arcmindai/arcmindai).

## Architecture

Sequence Flow Diagram
![ArcMind Vector DB](/diagram/architecture.png)

## Prerequisites

- Install Rust Toolchain using Rustup  
  Follows https://www.rust-lang.org/tools/install
- Install cargo-audit

```
cargo install cargo-audit
```

- Install dfx sdk  
  Follow https://github.com/dfinity/sdk

## Quick Start

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys controller and brain canisters to the local replica
# Setup the environment variable: CONTROLLER_PRINCIPAL using using > dfx identity get-principal

./scripts/provision.sh
```

The provision script will deploy a `arcmindvectordb` canister.

## API

See [Candid](/src/arcmindvectordb/arcmindvectordb.did) for the full API.

## Interacting with the canisters

Sample shell scripts are provided to interact with the canisters in the [interact](/interact/) directory.
Sample embeddings content and their embedding vectors are provided in the [embeddings](/embeddings/) directory.

### Add a vector to the VectorStore

Open and Edit:

```bash
./interact/add_vector.sh
```

Try adding multiple vectors of different topics to the VectorStore.

### Search the VectorStore

Then search for similar vectors by using one of the vectors you added as input.
It should return the same vector as the most similar vector and other similar vectors of the same topic.
See how it can understand the semantic meanings of the vectors with many dimensions.

Open and Edit:

```bash
./interact/search_vector.sh
```

Note that the same embedding model must be used for adding and searching vectors.
It is recommended that you use the same embedding model in a single VectorStore for consistent results.

The embeddings in /embeddings/ are generated using the [OpenAI text-embedding-ada-002](https://platform.openai.com/docs/guides/embeddings/embedding-models) model with its [Embedding API](https://platform.openai.com/docs/api-reference/embeddings)

## Setting up Github Action CI / CD

Get the string using commands below then put it into Github Secrets.
Note: Replace default by the identity name you need.

### DFX_IDENTITY

```
awk 'NF {sub(/\r/, ""); printf "%s\\r\\n",$0;}' ~/.config/dfx/identity/default/identity.pem
```

### DFX_WALLETS

```
cat ~/.config/dfx/identity/default/wallets.json
```

## Roadmap

- [x] Backend - Research and implement primary canister as long-term VectorStore with Nearest Neighbours distance metric, embedding API and indexing
- [x] Backend - Integrate with ArcMind AI Autonomous Agent for long-term memory
- [ ] Doc - Add documentation for the VectorStore API
- [ ] Backend - Self-hosted machine learning models for generating text (NLP), image and audio embeddings
- [ ] Backend - Scalable storage buckets for large-scale vector data beyond the canister storage limit

## License

See the [License](LICENSE) file for license rights and limitations (MIT).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details about how to contribute to this project.

## Authors

Code & Architecture: Henry Chan, [henry@arcmindai.app](mailto:henry@arcmindai.app), Twitter: [@kinwo](https://twitter.com/kinwo)

## References

- [Internet Computer](https://internetcomputer.org)
- [Cloudflare - What is a Vector Database?](https://developers.cloudflare.com/vectorize/reference/what-is-a-vector-database/)
- [RAG](https://arxiv.org/abs/2005.11401)
- [Open-source vector similarity search for Postgres](https://github.com/pgvector/pgvector)
- [Spotify Annoy Library - Approximate Nearest Neighbors in C++/Python](https://github.com/spotify/annoy)
- [What is similarity Search](https://www.pinecone.io/learn/what-is-similarity-search/)
- [Semantic Search: Measuring Meaning From Jaccard to Bert](https://www.pinecone.io/learn/semantic-search/)
- [A high-performance, flexible, ergonomic k-d tree Rust library](https://github.com/sdd/kiddo)
- [K-d tree](https://en.wikipedia.org/wiki/K-d_tree)
- [Depplearing.ai course - Building Applications with Vector Databases](https://www.deeplearning.ai/short-courses/building-applications-vector-databases/)
