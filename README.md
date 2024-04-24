# How to deploy/instantiate/query:

Based off of:

https://docs.osmosis.zone/cosmwasm/testnet/cosmwasm-deployment/

On WSL Ubuntu 20.04

```
Set up Rust:

# 1. Set 'stable' as the default release channel:
rustup default stable
cargo version
# If this is lower than 1.50.0+, update
rustup update stable

# 2. Add WASM as the compilation target:
rustup target list --installed
rustup target add wasm32-unknown-unknown

# 3. Install the following packages to generate the contract:
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script


```

```

Set up osmosis testnet:
curl -sL https://get.osmosis.zone/install > i.py && python3 i.py

# add wallets for testing
osmosisd keys add wallet

# Make sure to save your passphrase, wallet info, address somewhere. Get uosmo from faucet or from other people. Check balance with:

osmosisd query bank balances $(osmosisd keys show -a wallet)


```

```
rustup default stable

RUSTFLAGS='-C link-arg=-s' cargo build --release --target wasm32-unknown-unknown
```

```
RES=$(osmosisd tx wasm store target/wasm32-unknown-unknown/release/muh_contract.wasm --from wallet --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 -y --output json)
```

```
sudo apt-get install jq
```

```
#go to https://www.mintscan.io/osmosis-testnet/tx/{transaction hash} to get the code id
CODE_ID = code id from mintscan

osmosisd tx wasm instantiate $CODE_ID "{}" \
    --from wallet --label "my first contract" --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 --no-admin
```

```
CONTRACT_ADDR=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[0]')
```

```
# Can query something like:
QUERY='{"GetUpdate":{"model":"MODEL_A"}}'

osmosisd query wasm contract-state smart $CONTRACT_ADDR "$QUERY" --output json
```

Can take a look at https://osmosis-contracts.web.app/#/codes/{CODE ID} for some gui visualizations of stuff