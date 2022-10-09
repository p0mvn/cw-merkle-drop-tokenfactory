import '../styles/globals.css';
import type { AppProps } from 'next/app';
import { WalletProvider } from '@cosmos-kit/react';
import { ChakraProvider } from '@chakra-ui/react';
import { defaultTheme } from '../config';
import { wallets } from '@cosmos-kit/keplr';
import { assets, chains } from 'chain-registry';
import { getSigningCosmosClientOptions } from 'osmojs';
import { GasPrice } from '@cosmjs/stargate';

import { SignerOptions } from '@cosmos-kit/core';
import { Chain, AssetList } from '@chain-registry/types';

function CreateCosmosApp({ Component, pageProps }: AppProps) {
  const signerOptions: SignerOptions = {
    stargate: (_chain: Chain) => {
      return getSigningCosmosClientOptions();
    },
    cosmwasm: (chain: Chain) => {
      switch (chain.chain_name) {
        case 'osmosis':
        case 'osmosistestnet':
        case 'Local Osmosis':
          return {
            gasPrice: GasPrice.fromString('0.0025uosmo')
          };
      }
    }
  };

  let localosmosis: Chain ={
    chain_name: 'Local Osmosis',
    status: 'live',
    chain_id: 'Local Osmosis',
    apis: {
        "rpc": [{
            address: 'http://localhost:26657'
        }],
        "rest": [{
            address: 'http://localhost:1317'
        }],
        "grpc": [{
            address: 'http://localhost:9090'
        }]
    },
    network_type: 'local',
    pretty_name: 'Osmosis Local',
    bech32_prefix: 'osmo',
    slip44: 118,
    "logo_URIs": {
        "png": "https://raw.githubusercontent.com/cosmos/chain-registry/master/osmosis/images/osmosis-chain-logo.png"
    },
    codebase: {
        "cosmwasm_version": "0.28",
        "cosmwasm_enabled": true,
    }
  };

  let localosmosisAssetList: AssetList = {
    chain_name: 'Local Osmosis',
    "assets": [
        {
          "description": "The native token of Osmosis",
          "denom_units": [
            {
              "denom": "uosmo",
              "exponent": 0,
              "aliases": []
            },
            {
              "denom": "osmo",
              "exponent": 6,
              "aliases": []
            }
          ],
          "base": "uosmo",
          "name": "Osmosis",
          "display": "osmo",
          "symbol": "OSMO",
          "logo_URIs": {
            "png": "https://raw.githubusercontent.com/cosmos/chain-registry/master/osmosis/images/osmo.png",
            "svg": "https://raw.githubusercontent.com/cosmos/chain-registry/master/osmosis/images/osmo.svg"
          },
          "coingecko_id": "osmosis",
          "keywords": [
              "dex", "staking"
          ]
        },
        {
          "denom_units": [
            {
              "denom": "uion",
              "exponent": 0
            },
            {
              "denom": "ion",
              "exponent": 6
            }
          ],
          "base": "uion",
          "name": "Ion",
          "display": "ion",
          "symbol": "ION",
          "logo_URIs": {
            "png": "https://raw.githubusercontent.com/cosmos/chain-registry/master/osmosis/images/ion.png",
            "svg": "https://raw.githubusercontent.com/cosmos/chain-registry/master/osmosis/images/ion.svg"
          },
          "coingecko_id": "ion",
          "keywords": [
              "memecoin"
          ]
        }
      ]
  };

  return (
    <ChakraProvider theme={defaultTheme}>
      <WalletProvider
        chains={[localosmosis, ...chains]}
        assetLists={[localosmosisAssetList, ...assets]}
        wallets={wallets}
        signerOptions={signerOptions}
      >
        <Component {...pageProps} />
      </WalletProvider>
    </ChakraProvider>
  );
}

export default CreateCosmosApp;
