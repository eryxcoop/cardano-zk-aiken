import { Contract } from "./utils";

async function main() {
  const compiledContractPath = process.argv[2];
  const contract = new Contract(compiledContractPath);
  // get utxo, collateral and address from wallet
  await contract.spend(parseInt(process.argv[3]), process.argv[4]);
}
 
main();
