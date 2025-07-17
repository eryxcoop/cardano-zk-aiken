import { BlockchainProvider } from "./blockchain_provider";
import { Contract } from "./contract";


async function main() {
  const compiledContractPath = process.argv[2];
  const validatorScriptIndex = parseInt(process.argv[3]);
  const blockchain_provider = new BlockchainProvider();
  const wallet = blockchain_provider.getWalletUsing('me.sk');

  const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
  await contract.deploy(validatorScriptIndex, 35);
}

main();