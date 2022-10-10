import { useEffect, useState, useMemo } from 'react';
import { useWallet } from '@cosmos-kit/react';
import { osmosis } from 'osmojs';
import { MerkleDropClient, MerkleDropQueryClient } from '../../codegen/MerkleDrop.client';
import {
  Grid,
  Button,
  GridItem,
  Input,
  List,
  ListItem
} from '@chakra-ui/react';
import { chainName, contractAddress } from '../../config/contract';
import { SigningStargateClient} from '@cosmjs/stargate';

export default function SubdenomSetter() {
  const [subdenom, setSubdenom] = useState<string | null>(null);

  const [contractSubdenom, setContractSubdenom] = useState<string | null>(null);

  const [tokenfactoryDenoms, setTokenFactoryDenoms] = useState<string[]>([]);

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

  let chain = chains.find((chain) => chain.name === chainName)?.chain;

  useEffect(() => {
    setCurrentChain(chainName);

    const fn = async () => {
        await connect();
        console.log("connected")
    }

    fn()
  }, [chainName]);

  const [merkleDropClient, setMerkleDropClient] = useState<MerkleDropClient | null>(
    null
  );

  const [stargateClient, setStargateClient] = useState<SigningStargateClient | null>(
    null
  );

  useEffect(() => {
    async function getTokenfactoryDenoms() {
        try {
            if (!address) {
                return;
            }

            console.log("getting tf denoms");
            let rpcEndpoint = chain.apis?.rpc?.at(0)?.address as string;
            console.log(rpcEndpoint);
            let osmoClient = await osmosis.ClientFactory.createRPCQueryClient({rpcEndpoint: rpcEndpoint});
            let response = await osmoClient.osmosis.tokenfactory.v1beta1.denomsFromCreator(osmosis.tokenfactory.v1beta1.QueryDenomsFromCreatorRequest.fromPartial({creator: address}))
            setTokenFactoryDenoms(response.denoms);
        } catch(err) {
            console.error("error getting tokenfactory denoms ", err);
        }
    }

    getTokenfactoryDenoms()

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
        new MerkleDropClient(
          cosmwasmClient,
          address,
          contractAddress
        )
      );
    });
  }, [address, getCosmWasmClient, getStargateClient]);
  
  useEffect(() => {
    async function getSubdenom() {
        if (merkleDropClient && address) {
            try {
                let response = await merkleDropClient.getSubdenom();
                setContractSubdenom(response.subdenom);
            } catch (e) {
                console.log(e);
            }
        }
    }

    getSubdenom()
  }, [merkleDropClient, address]);

  const submitSubdenom = async () => {
    console.log("submitting subdenom")

    const toSubmit: string = subdenom as string;

    console.log("toSubmit ", toSubmit)

    try {
        await merkleDropClient?.setSubDenom({ subdenom: toSubmit});
    } catch(err) {
        alert("failed to submit subdenom: " + err);
    }
  }

  return (
		<Grid textAlign="center">
            <Grid>
                <GridItem mb="6">
                    Contract Subdenom:{' '}
                        {contractSubdenom ? contractSubdenom :''}
                </GridItem>
                <GridItem mb="6">
                    <List>
                        <ListItem>
                            Signer Tokenfactory Denoms:
                        </ListItem>
                        {tokenfactoryDenoms.map((denom) => (
                            <ListItem>
                                {denom}
                            </ListItem>
                        ))}
                    </List>
                </GridItem>
            </Grid>

            <Grid templateColumns='repeat(2, 1fr)'>
                <GridItem mr="5">
                    <Input onChange={(e) => setSubdenom(e.target.value)} placeholder='small size' size='sm' />
                </GridItem>
                <GridItem>
                <Button onClick={submitSubdenom}>Submit</Button>
                </GridItem>
            </Grid>
		</Grid>
  );
}
