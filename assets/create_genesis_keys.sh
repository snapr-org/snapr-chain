#!/usr/bin/env bash

# this script was used to generate genesis keys for snapr mainnet and testnet chains.
# it exists in this repository purely for the purpose of scrutiny and transparency.

genesis_key_mount_point=${1}
assets_dir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

if test -z "${1}" || [ ! -d ${genesis_key_mount_point} ]; then
  echo "usage: ${assets_dir}/create-genesis-keys.sh /path/to/key/mount/point"
  exit 1
fi

for chain_name in testnet mainnet; do

  echo --- > ${assets_dir}/public_keys_${chain_name}.yml
  echo "authority:" >> ${assets_dir}/public_keys_${chain_name}.yml

  for key_name in trillian ford arthur; do

    echo "  ${key_name}:" >> ${assets_dir}/public_keys_${chain_name}.yml

    # create key folder
    if [ ! -d ${genesis_key_mount_point}/${chain_name}/authority/${key_name} ]; then
      mkdir -p ${genesis_key_mount_point}/${chain_name}/authority/${key_name}
    fi

    # create babe (sr25519) key

    if [ ! -s ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/babe/sr25519 ]; then
      subkey generate --scheme sr25519 --words 24 > ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/babe/sr25519
    fi

    echo "    babe:" >> ${assets_dir}/public_keys_${chain_name}.yml

    public_key_sr25519_hex=$(grep -Po '(?<=Public key \(hex\):  0x)[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/babe/sr25519 | cat)
    echo "      hex: ${public_key_sr25519_hex}" >> ${assets_dir}/public_keys_${chain_name}.yml
    public_key_sr25519_ss58=$(grep -Po '(?<=Public key \(SS58\): )[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/babe/sr25519 | cat)
    echo "      ss58: ${public_key_sr25519_ss58}" >> ${assets_dir}/public_keys_${chain_name}.yml

    # create grandpa (ed25519) key

    if [ ! -s ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/grandpa/ed25519 ]; then
      key_mnemonic=$(grep -Po '(?<=`)[^`]*(?=`)' ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/babe/sr25519 | cat)
      subkey inspect --scheme ed25519 "${key_mnemonic}" > ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/grandpa/ed25519
    fi

    echo "    grandpa:" >> ${assets_dir}/public_keys_${chain_name}.yml

    public_key_ed25519_hex=$(grep -Po '(?<=Public key \(hex\):  0x)[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/grandpa/ed25519 | cat)
    echo "      hex: ${public_key_ed25519_hex}" >> ${assets_dir}/public_keys_${chain_name}.yml
    public_key_ed25519_ss58=$(grep -Po '(?<=Public key \(SS58\): )[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/grandpa/ed25519 | cat)
    echo "      ss58: ${public_key_ed25519_ss58}" >> ${assets_dir}/public_keys_${chain_name}.yml

    for key_variant in controller stash im-online authority-discovery; do

      # create variant key folder
      if [ ! -d ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/${key_variant} ]; then
        mkdir -p ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/${key_variant}
      fi

      # create sr25519 variant key
      if [ ! -s ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/${key_variant}/sr25519 ]; then
        subkey generate --scheme sr25519 --words 24 > ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/${key_variant}/sr25519
      fi

      echo "    ${key_variant}:" >> ${assets_dir}/public_keys_${chain_name}.yml

      public_key_sr25519_hex=$(grep -Po '(?<=Public key \(hex\):  0x)[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/${key_variant}/sr25519 | cat)
      echo "      hex: ${public_key_sr25519_hex}" >> ${assets_dir}/public_keys_${chain_name}.yml
      public_key_sr25519_ss58=$(grep -Po '(?<=Public key \(SS58\): )[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/authority/${key_name}/${key_variant}/sr25519 | cat)
      echo "      ss58: ${public_key_sr25519_ss58}" >> ${assets_dir}/public_keys_${chain_name}.yml

    done

  done

  echo "endowed:" >> ${assets_dir}/public_keys_${chain_name}.yml

  for key_name in liquidity core contributor sudo; do

    echo "  ${key_name}:" >> ${assets_dir}/public_keys_${chain_name}.yml

    # create key folder
    if [ ! -d ${genesis_key_mount_point}/${chain_name}/endowed/${key_name} ]; then
      mkdir -p ${genesis_key_mount_point}/${chain_name}/endowed/${key_name}
    fi

    # create sr25519 key
    if [ ! -s ${genesis_key_mount_point}/${chain_name}/endowed/${key_name}/sr25519 ]; then
      subkey generate --scheme sr25519 --words 24 > ${genesis_key_mount_point}/${chain_name}/endowed/${key_name}/sr25519
    fi

    public_key_sr25519_hex=$(grep -Po '(?<=Public key \(hex\):  0x)[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/endowed/${key_name}/sr25519 | cat)
    echo "    hex: ${public_key_sr25519_hex}" >> ${assets_dir}/public_keys_${chain_name}.yml
    public_key_sr25519_ss58=$(grep -Po '(?<=Public key \(SS58\): )[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/endowed/${key_name}/sr25519 | cat)
    echo "    ss58: ${public_key_sr25519_ss58}" >> ${assets_dir}/public_keys_${chain_name}.yml

  done

  echo "faucet:" >> ${assets_dir}/public_keys_${chain_name}.yml

  for key_name in stash 2021 2022 2023 2024 2025 2026 2027 2028 2029 2030; do

    echo "  ${key_name}:" >> ${assets_dir}/public_keys_${chain_name}.yml

    # create key folder
    if [ ! -d ${genesis_key_mount_point}/${chain_name}/faucet/${key_name} ]; then
      mkdir -p ${genesis_key_mount_point}/${chain_name}/faucet/${key_name}
    fi

    # create sr25519 key
    if [ ! -s ${genesis_key_mount_point}/${chain_name}/faucet/${key_name}/sr25519 ]; then
      subkey generate --scheme sr25519 --words 24 > ${genesis_key_mount_point}/${chain_name}/faucet/${key_name}/sr25519
    fi

    public_key_sr25519_hex=$(grep -Po '(?<=Public key \(hex\):  0x)[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/faucet/${key_name}/sr25519 | cat)
    echo "    hex: ${public_key_sr25519_hex}" >> ${assets_dir}/public_keys_${chain_name}.yml
    public_key_sr25519_ss58=$(grep -Po '(?<=Public key \(SS58\): )[^`]*(?=$)' ${genesis_key_mount_point}/${chain_name}/faucet/${key_name}/sr25519 | cat)
    echo "    ss58: ${public_key_sr25519_ss58}" >> ${assets_dir}/public_keys_${chain_name}.yml

  done
  yq . ${assets_dir}/public_keys_${chain_name}.yml > ${assets_dir}/public_keys_${chain_name}.json
done
