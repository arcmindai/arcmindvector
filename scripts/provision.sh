# To deplopy locally, update IC_NETWORK to local. To deploy to ic, update IC_NETWORK to ic.
IC_NETWORK=${IC_NETWORK:-local}
echo Provisioning on $IC_NETWORK

OWENR_PRINCIPAL=$(dfx identity --network $IC_NETWORK get-principal)
# CONTROLLER_PRINCIPAL=$(dfx canister --network $IC_NETWORK id arcmindai_controller)
CONTROLLER_PRINCIPAL=br5f7-7uaaa-aaaaa-qaaca-cai

# Deploy vectordb canister 
echo Deploying vectordb canister with owner $OWENR_PRINCIPAL, controller $CONTROLLER_PRINCIPAL
dfx deploy --network $IC_NETWORK arcmindvectordb --argument "(opt principal \"$OWENR_PRINCIPAL\", opt principal \"$CONTROLLER_PRINCIPAL\")"
