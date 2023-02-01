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
        Yes ) near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/toss_coin.wasm --initFunction new --initArgs '{"accountid_last_deposit":"alan_test.testnet","ft_token_id":"bb-strw.testnet","owner_id":"alan_test.testnetS"}'; break;;
        No ) near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/toss_coin.wasm;break;;
    esac
done
