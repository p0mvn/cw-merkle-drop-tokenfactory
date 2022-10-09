import { Box, Button, FormControl, FormLabel, Input, Table, Tbody, Thead, Tr } from "@chakra-ui/react";
import { useState } from "react";
import { useWallet } from '@cosmos-kit/react';
import init, { wasm_gets, wasm_sends} from "stack"
import { cosmos } from "osmojs";
import { StdFee } from "@cosmjs/stargate";

export default function CsvProcessor() {
	const [file, setFile] = useState();
	const [array, setArray] = useState([]);

    const fileReader = new FileReader();

    const {
        getStargateClient,
        getCosmWasmClient,
        address,
        setCurrentChain,
        currentWallet,
        walletStatus
      } = useWallet();

      const sendTokens = async () => {
        const stargateClient = await getStargateClient();
        if (!stargateClient || !address) {
            console.error('stargateClient undefined or address undefined.')
            return;
        }
  
        const { send } = cosmos.bank.v1beta1.MessageComposer.withTypeUrl;
  
        const msg = send({
          amount: [ { denom: 'uatom', amount: '1000' } ],
          toAddress: address, fromAddress: address
        });
  
        const fee: StdFee = { amount: [ { denom: 'uatom', amount: '864' } ], gas: '86364' };

        const memo = "";
  
        await stargateClient.signAndBroadcast(address, [msg], fee, memo);
      }

    const handleOnChange = (e: any) => {
        setFile(e.target.files[0]);
    };

	const csvFileToArray = (string: any) => {
		const csvHeader = string.slice(0, string.indexOf("\n")).split(",");
		const csvRows = string.slice(string.indexOf("\n") + 1).split("\n");
	
		const array = csvRows.map((i: string) => {
			const values = i.split(",");
			const obj = csvHeader.reduce((object: { [x: string]: any; }, header: string | number, index: string | number | any) => {
			object[header] = values[index];
			return object;
			}, {});
			return obj;
		});
		let pass = array.filter((el: any) => {
			return el.staked !== undefined;
		});
		init().then(() => {
			let example = wasm_gets(pass);
			// wasm_sends(pass);
		})
		setArray(array);
	};


    const handleOnSubmit = (e: { preventDefault: () => void; }) => {
        e.preventDefault();

        if (file) {
            fileReader.onload = function (event) {
                const csvOutput = event.target?.result;
				csvFileToArray(csvOutput)
            };

            fileReader.readAsText(file);
        }
    };

	const headerKeys = Object.keys(Object.assign({}, ...array));

	return (
		<Box alignContent='center'>
            <FormControl>
                <Button mx="4" mt="4">Upload Root</Button>
            </FormControl>
			<FormControl>
				<Button mx="4" mt="4">
					<FormLabel htmlFor='file-upload' alignContent='center' m="2">Pick a File</FormLabel>
				</Button>
				<Input onChange={handleOnChange} accept=".csv" id='file-upload' type='file' display="none"/>
				<Button onClick={(e) => {handleOnSubmit(e);}} mt="4">Process CSV</Button>
			</FormControl>
			<Table mt="6">
				<Thead>
				<Tr key={"header"}>
					{headerKeys.map((key, idx) => (
					<th key={idx}>{key}</th>
					))}
				</Tr>
				</Thead>
				<Tbody>
				{array.map((item) => (
					<Tr key={item}>
					{Object.values(item).map((val: any, idx) => (
						<td key={idx}>{val}</td>
					))}
					</Tr>
				))}
				</Tbody>
			</Table>
		</Box>
	);
}