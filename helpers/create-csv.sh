#!/bin/bash
# $1 is minStakeAmount. If min stake amount not provided, GT 0 is used
# $2 is height. If Height not provided, latest height is used

if [ -z $1 ];
then
    minStake=0
else
    minStake=$1
fi

COUNTER=0
while [ $COUNTER -lt 10 ]; do
    status=$(osmosisd status 2>&1)
    if [[ "$status" == *"Error"* ]];
    then
        systemctl start osmosisd
        sleep 1
        let COUNTER=COUNTER+1
    elif [[ $COUNTER -eq 9 ]];
    then
        echo "osmosis daemon unable to start"
        exit 1
    else
        let COUNTER=10
    fi
done

earliestHeight=$(osmosisd status 2>&1 | jq ".SyncInfo.earliest_block_height")
echo "earliest height is $earliestHeight"
latestHeight=$(osmosisd status 2>&1 | jq ".SyncInfo.latest_block_height")
echo "latest height is $latestHeight"

cd $HOME/.osmosisd
echo "stopping osmosisd service"
systemctl stop osmosisd

if [ -z $2 ];
then
    echo "no height provided, exporting latest height"
    osmosisd export --modules-to-export="lockup,bank,gamm,staking" 2> export.json

elif [ $2 -gt $latestHeight ];
then
    echo "cannot export height greater than latest height"
    exit 1

elif [ $2 -lt $earliestHeight ];
then
    echo "cannot export height less than earliest height"
    exit 1

else
    echo "height $2 provided, exporting height $2"
    osmosisd export --modules-to-export="lockup,bank,gamm,staking" --height $2 2> export.json
fi

echo "starting osmosisd"
systemctl start osmosisd
echo "exporting balances"
osmosisd export-derive-balances export.json balances.json
echo "exporting csv"
osmosisd staked-to-csv --minimum-stake-amount=$minStake balances.json airdrop.csv
echo "done"
