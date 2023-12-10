# To deplopy locally, update IC_NETWORK to local. To deploy to ic, update IC_NETWORK to ic.
IC_NETWORK=${IC_NETWORK:-local}
echo Provisioning on $IC_NETWORK

# Owner is controller canister
OWENR_PRINCIPAL=${CONTROLLER_PRINCIPAL}

# Deploy vectordb canister 
echo Deploying vectordb canister with owner $OWENR_PRINCIPAL
dfx deploy --network $IC_NETWORK arcmindvectordb --argument "(opt principal \"$OWENR_PRINCIPAL\")"
