#!/bin/bash

# Validate required env vars
if [[ -z "${CONTROLLER_PRINCIPAL}" ]]; then
  echo "CONTROLLER_PRINCIPAL is unset."
  exit 1
fi

if [[ -z "${BATTERY_PRINCIPAL}" ]]; then
  echo "BATTERY_PRINCIPAL is unset."
  exit 1
fi

if [[ -z "${BATTERY_API_KEY}" ]]; then
  echo "BATTERY_API_KEY is unset."
  exit 1
fi

# To deplopy locally, update IC_NETWORK to local. To deploy to ic, update IC_NETWORK to ic.
IC_NETWORK=${IC_NETWORK:-local}

# Deploy vectordb canister 

echo Deploying arcmindvectordb canister with owner=$CONTROLLER_PRINCIPAL, BATTERY_PRINCIPAL=$BATTERY_PRINCIPAL, BATTERY_API_KEY=$BATTERY_API_KEY on $IC_NETWORK
dfx deploy --network $IC_NETWORK arcmindvectordb --argument "(opt principal \"$CONTROLLER_PRINCIPAL\", opt \"$BATTERY_API_KEY\", opt principal \"$BATTERY_PRINCIPAL\")" > arcmindvectordb_deploy.txt

VECTOR_PRINCIPAL=$(dfx canister --network $IC_NETWORK id arcmindvectordb)
echo $VECTOR_PRINCIPAL
