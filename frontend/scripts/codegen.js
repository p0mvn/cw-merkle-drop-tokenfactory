const { join, resolve }  = require('path');
const codegen = require('@cosmwasm/ts-codegen').default;

const contractsDir = resolve(join(__dirname, '/../../contracts'));
const contracts = [
  {
    name: 'MerkleDrop',
    dir: join(contractsDir, 'merkle-drop/schema')
  }
];

codegen({
  contracts,
  outPath: join(__dirname, '../codegen'),
  options: {
    bundle: {
      enabled: true,
      bundleFile: 'index.ts',
      scope: 'contracts'
    },
    types: {
      enabled: true
    },
    client: {
      enabled: true
    },
    messageComposer: {
      enabled: false
    }
  }
}).then(() => {
  console.log('✨ all done!');
}).catch(e=>{
  console.error(e);
  process.exit(1)
});
