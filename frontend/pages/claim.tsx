import { useEffect, useState, useMemo } from 'react';
import { useWallet } from '@cosmos-kit/react';
import { assets } from 'chain-registry';
import { AssetList, Asset } from '@chain-registry/types';

// import cosmwasm client generated with cosmwasm-ts-codegen
import { MerkleDropQueryClient } from '../codegen/MerkleDrop.client';

import {
  Box,
  Divider,
  Grid,
  Heading,
  Text,
  Stack,
  Container,
  Link,
  Button,
  Flex,
  Icon,
  useColorMode,
  useColorModeValue,
  GridItem
} from '@chakra-ui/react';
import { BsFillMoonStarsFill, BsFillSunFill } from 'react-icons/bs';
import { dependencies, products } from '../config';

import { WalletStatus } from '@cosmos-kit/core';
import { Product, Dependency, WalletSection, ChainCard } from '../components';
import Head from 'next/head';
import Navbar from '../components/navbar';
import Layout from '../components/navbar';
import { chainName, contractAddress } from '../config/contract';

const library = {
  title: 'OsmoJS',
  text: 'OsmoJS',
  href: 'https://github.com/osmosis-labs/osmojs'
};

const chainassets: AssetList = assets.find(
  // N.B. we do not provide a separate asset list for LocalOsmosis
  (chain) => chain.chain_name === "osmosis"
) as AssetList;
const coin: Asset = chainassets.assets.find(
  (asset) => asset.base === 'uosmo'
) as Asset;

export default function Claim() {
  const { colorMode, toggleColorMode } = useColorMode();

  const {
    getStargateClient,
    getCosmWasmClient,
    address,
    setCurrentChain,
    currentWallet,
    walletStatus,
    connect,
    chains
  } = useWallet();

  useEffect(() => {
    setCurrentChain(chainName);

    const fn = async () => {
        await connect();
        console.log("connected")
    }

    fn()
  }, [chainName]);

  const chainOptions = useMemo(
    () =>
      chains.map((chainRecord) => {

        return {
          chainName: chainRecord.name,
          label: chainRecord.chain.pretty_name,
          value: chainRecord.name,
          icon: chainassets
            ? chainassets.assets[0]?.logo_URIs?.svg || chainassets.assets[0]?.logo_URIs?.png
            : undefined,
          disabled: false
        };
      }),
    [chains]
  );

  const chain = chainOptions.find((c) => c.chainName === chainName);

  

  // get cw20 balance
  const [merkleDropClient, setMerkleDropClient] = useState<MerkleDropQueryClient | null>(
    null
  );
  useEffect(() => {
    getCosmWasmClient().then((cosmwasmClient) => {
      if (!cosmwasmClient) {
        console.error('cosmwasmClient undefined');
        return;
      }

      if (!address) {
        console.error('address undefined');
        return;
      }

      setMerkleDropClient(
        new MerkleDropQueryClient(
          cosmwasmClient,
          contractAddress,
        )
      );
    });
  }, [address, getCosmWasmClient]);
  const [root, setRoot] = useState<string | null>(null);
  useEffect(() => {
    async function getRoot() {
        if (merkleDropClient && address) {
            let response = await merkleDropClient.getRoot();

            setRoot(response.root);
        }
    }

    getRoot()
  }, [merkleDropClient, address]);

  return (
	<Layout>
		<Container maxW="5xl" py={10}>
		<Head>
			<title>Create Cosmos App</title>
			<meta name="description" content="Generated by create cosmos app" />
			<link rel="icon" href="/favicon.ico" />
		</Head>
		<Flex justifyContent="end" mb={4}>
		</Flex>
		<Box textAlign="center">
			<Heading
			as="h1"
			fontSize={{ base: '3xl', sm: '4xl', md: '5xl' }}
			fontWeight="extrabold"
			mb={3}
			>
			Claim your Airdrop
			</Heading>
			<Heading
			as="h1"
			fontWeight="bold"
			fontSize={{ base: '2xl', sm: '3xl', md: '4xl' }}
			>
			</Heading>
		</Box>

        <Box textAlign="center">
        {chainName && (
          <GridItem marginBottom={'20px'}>
            <ChainCard
              prettyName={chain?.label || chainName}
              icon={chain?.icon}
            />
          </GridItem>
        )}
        </Box>

        <Grid templateColumns='repeat(2, 1fr)'>
            <GridItem>
                <WalletSection chainName={chainName} />
            </GridItem>
            <GridItem>
                Test
            </GridItem>
        </Grid>

		
		<Grid textAlign="center">
		<GridItem>
			{walletStatus === WalletStatus.Disconnected
				? ''
				: <Button 
				mb={8} 
				bgImage="linear-gradient(109.6deg, rgba(157,75,199,1) 11.2%, rgba(119,81,204,1) 83.1%)"
				color="white"
				opacity={1}
				transition="all .5s ease-in-out"
				_hover={{
					bgImage:
						'linear-gradient(109.6deg, rgba(157,75,199,1) 11.2%, rgba(119,81,204,1) 83.1%)',
					opacity: 0.75
				}}
				_active={{
					bgImage:
					'linear-gradient(109.6deg, rgba(157,75,199,1) 11.2%, rgba(119,81,204,1) 83.1%)',
					opacity: 0.9
				}}
				>Claim</Button>}
		</GridItem>
		<GridItem>
			Merkle Root:{' '}
				{root ? root :'loading...'}
		</GridItem>
		</Grid>
		</Container>
	</Layout>
  );
}
