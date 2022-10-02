import { Box, Button, FormControl, FormHelperText, FormLabel, Input, Table, Tbody, Thead, Tr } from "@chakra-ui/react";
import { SetStateAction, useState } from "react";


export default function CsvProcessor() {
	const [file, setFile] = useState();
	const [array, setArray] = useState([]);

    const fileReader = new FileReader();

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
				<Button mx="4" mt="4">
					<FormLabel htmlFor='file-upload' alignContent='center' m="2">Pick a File</FormLabel>
				</Button>
				<Input onChange={handleOnChange} accept=".csv" id='file-upload' type='file' display="none"/>
				<Button onClick={(e) => {handleOnSubmit(e);}} mt="4">Process CSV</Button>
			</FormControl>
			<Table mt="6">
				<Thead>
				<Tr key={"header"}>
					{headerKeys.map((key) => (
					<th>{key}</th>
					))}
				</Tr>
				</Thead>

				<Tbody>
				{array.map((item) => (
					<Tr key={item}>
					{Object.values(item).map((val: any) => (
						<td>{val}</td>
					))}
					</Tr>
				))}
				</Tbody>
			</Table>
		</Box>
	);
}