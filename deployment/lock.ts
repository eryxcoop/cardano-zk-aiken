import { Contract } from "./contract";


async function main() {
  const compiledContractPath = process.argv[2];
  const validatorScriptIndex = parseInt(process.argv[3]);
  
  const contract = new Contract(compiledContractPath);
  await contract.deploy(validatorScriptIndex, 42);
}

main();