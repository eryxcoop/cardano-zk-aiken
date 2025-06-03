import { Contract} from "./utils";

async function main() {
  const compiledContractPath = process.argv[2];
  const validatorIndex = parseInt(process.argv[3]);
  
  const contract = new Contract(compiledContractPath);
  // these are the assets we want to lock into the contract
  await contract.deployWithoutDatum(validatorIndex);
}
 
main();