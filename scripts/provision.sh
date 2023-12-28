# Validate required env vars
if [[ -z "${CONTROLLER_PRINCIPAL}" ]]; then
  echo "CONTROLLER_PRINCIPAL is unset."
  exit 1
fi

# To deplopy locally, update IC_NETWORK to local. To deploy to ic, update IC_NETWORK to ic.
IC_NETWORK=${IC_NETWORK:-local}
echo Provisioning on $IC_NETWORK

# Deploy vectordb canister 
echo Deploying vectordb canister with owner $CONTROLLER_PRINCIPAL
dfx deploy --network $IC_NETWORK arcmindvectordb --argument "(opt principal \"$CONTROLLER_PRINCIPAL\")"
