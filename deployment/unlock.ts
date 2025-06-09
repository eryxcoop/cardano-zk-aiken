import { BlockchainProvider } from "./blockchain_provider";
import { Contract, mVoid } from "./contract";

async function main() {
  const compiledContractPath = process.argv[2];
  const validatorIndex = parseInt(process.argv[3]);
  const txHashFromDeposit = process.argv[4];
  const blockchain_provider = new BlockchainProvider();
  const wallet = blockchain_provider.getWalletUsing('me.sk');
  
  const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
}
 
main();
