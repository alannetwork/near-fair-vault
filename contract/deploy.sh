#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
echo ">> Initi contract?"
select yn in "Yes" "No"; do
    case $yn in
        Si ) near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/toss_coin.wasm --initFunction new --initArgs '{"owner_id":"alan_test.testnet"}'; break;;
        No ) near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/toss_coin.wasm;break;;
    esac
done
