#!/usr/bin/env bash

# this script was used to generate chain spec for snapr mainnet and testnet chains.
# it exists in this repository purely for the purpose of scrutiny and transparency.

genesis_key_mount_point=${1}
assets_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

for chain_name in testnet mainnet; do
  cargo run --release -- build-spec --disable-default-bootnode --chain ${chain_name} > ${assets_dir}/chain_spec_${chain_name}_human_readable.json
  cargo run --release -- build-spec --disable-default-bootnode --chain ${assets_dir}/chain_spec_${chain_name}_human_readable.json > ${assets_dir}/chain_spec_${chain_name}.json
done
