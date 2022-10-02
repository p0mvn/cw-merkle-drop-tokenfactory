# Merkle Drop App

## Introduction

TODO

## Prerequisits

* Have osmosisd running as a systemctl service named osmosisd.service

* Ensure you have enough state to export the account balances at the desired height

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

Run the script

```bash
./helpers/create-csv.sh
```

Optionally, you can provide two inputs: the minimum staked amount and the export height. In the following example, this provides a csv of all accounts have staked 1000 OSMO or more on block height 4000000.

```bash
./helpers/create-csv.sh 1000000000 4000000
```

This provides you with the final airdrop file in the following location:

* ~/.osmosisd/airdrop.csv
