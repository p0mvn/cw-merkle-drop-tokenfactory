# Merkle Drop App

## Introduction

TODO

## Prerequisites

1. Have osmosisd running as a systemctl service named osmosisd.service

  * This can be done automatically with the following command: `source <(curl -sL https://get.osmosis.zone/run)`. Choose the full node option and at the end choose to run a osmosisd service (not a cosmovisor service for this to work out of the box)

2. Ensure you have enough state to export the account balances at the desired height

## Instructions

Clone the repo and step inside

```bash
cd $HOME
git clone https://github.com/p0mvn/merkle-drop-app.git
cd $HOME/merkle-drop-app
```

Give the script execution permissions

```bash
chmod +x ./helpers/create-csv.sh
```

Run the script. Without any inputs, this will give you a csv of all accounts that have staked greater than 0 OSMO at the latest block height you have on your full node.

```bash
./helpers/create-csv.sh
```

Optionally, you can provide two inputs: the minimum staked amount and the export height. In the following example, this provides a csv of all accounts have staked 1000 OSMO or more on block height 4000000.

```bash
./helpers/create-csv.sh 1000000000 4000000
```

This provides you with the final airdrop file in the following location:

```bash
cat ~/.osmosisd/airdrop.csv
```

## Interacting with The Contract

### Deploy and Instantiate

```bash
beaker wasm deploy merkle-drop --signer-account test1 --no-wasm-opt --raw '{ "merkle_root": "1V0YcwzXWtB+iuOTob6juiNliUmB278xZIKMnzwjqOU=" }' --label 1
```

### Create TokenFactory Denom For Testing

- create
```bash
osmosisd tx tokenfactory create-denom mydenom --from lo-test1 --keyring-backend test --chain-id=localosmosis -b=block
```

- verify admin
```bash
osmosisd q tokenfactory denom-authority-metadata factory/osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks/mydenom
```

- change admin
```bash
osmosisd tx tokenfactory change-admin "factory/osmo1cyyzpxplxdzkeea7kwsydadg87357qnahakaks/mydenom" "osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9" --from lo-test1 --keyring-backend test -b=block --chain-id=localosmosis
```

### SetSubDenom

```bash
beaker wasm execute merkle-drop --raw '{ "set_sub_denom": { "subdenom": "mydenom" } }' --signer-account test1 --label 1
```

### Claim

```bash
beaker wasm execute merkle-drop --raw '{ "claim": { "claimer_addr": "osmo1hqslwuc8ukaaaxfmahgnquyqx3w0tmrluwxmxj", "amount": "1421901", "proof": "[{\"is_left_sibling\":true,\"hash\":[89,79,106,114,49,69,77,102,68,119,114,48,69,84,73,103,82,71,97,108,48,79,108,53,105,56,82,103,111,57,85,51,76,70,82,90,115,66,97,78,89,51,73,61]},{\"is_left_sibling\":false,\"hash\":[80,54,110,55,43,55,72,72,111,52,109,104,79,104,102,105,108,83,43,118,87,54,88,85,88,113,48,115,105,99,83,116,116,52,112,54,119,114,68,48,113,47,73,61]},{\"is_left_sibling\":true,\"hash\":[79,79,110,66,86,100,72,56,121,84,70,57,115,78,65,56,80,85,81,97,111,71,89,119,81,89,87,83,109,71,116,89,56,79,118,85,118,98,73,83,122,74,77,61]},{\"is_left_sibling\":false,\"hash\":[102,65,68,121,57,69,49,118,56,70,78,78,81,53,109,47,50,120,78,55,103,110,119,89,78,82,104,80,83,53,69,105,79,53,115,79,77,43,118,106,50,98,56,61]}]" } }' --signer-account test1 --label 1
```
