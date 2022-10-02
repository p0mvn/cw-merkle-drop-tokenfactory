# Merkle Drop App

## Introduction

TODO

## Prerequisits

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
