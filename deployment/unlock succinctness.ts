import { mConStr0, mStringToPlutusBSArray } from "@meshsdk/core";
import { BlockchainProvider } from "./blockchain_provider";
import { Contract } from "./contract";

async function main() {
  const compiledContractPath = process.argv[2];
  const validatorScriptIndex = parseInt(process.argv[3]);
  const txHashFromDeposit = process.argv[4];
  const blockchain_provider = new BlockchainProvider();
  const wallet = blockchain_provider.getWalletUsing('me.sk');
  
  const contract = new Contract(compiledContractPath, blockchain_provider, wallet);
  const factors = mConStr0([5, 7]);
  const proof1 = mConStr0([
    "949135c16bda90ad1633a73dc349e3bfa5791d81f66c3bc2de11c58230fff2b24a8c736d72f6e77ad466354c0245a3d9",
    "8cb4dfcf31b1a950656af753975bc46a76c7ff38f9f024e77e93478c92f9d65a5a6c36ecdb566be81b19a37f6240b9a70f295405af43470931b462ca27189ae51e4aceb38ab457e0c19392dccc1d35cc12f61fec33988f49447c1ddac36e46d2",
    "99a38d427161b538c189cbb7720aa1b6a84afb7e3824caad44a5e54bb91d4bd1b292ce8e08a7697c89aa2bf2ccc6e739"]);
  const proof2 = mConStr0([
    "87b2ffc6ee3e3fad918e629a122cecc7709d758c9d67568ac64e2546d420dedd964911d5ffafb0883381eb12d38e6c99",
    "84c05d2bc38a0b2229df9471ed92cfa13a2681a1824b4c67c78b3f518447754cd1ec011de7fbcc9f2e0cd3bec8c9b4a212f0b9bb5a29e3097c1de1a62e6aec5d8058b4c2651be334930d59c2ab74a8ffbcce2a625075c103cdf9927bf1e3f2ad",
    "b6377a604c9e218a004e29c54250e925c84715b762fe3b55e8a98f638f64d9934562f3175f86c021b3caf42ab04e55af"]);
  const proofs = [proof1, proof2];
  const redeemer = mConStr0([factors, proofs]);
  contract.spend(validatorScriptIndex, txHashFromDeposit, redeemer, {mem: 1301280, steps: 5031522698})
}

main();
