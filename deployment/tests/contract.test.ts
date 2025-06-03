import { blockchainProvider, Contract, getUtxoByTxHash } from "../utils";

describe('Contract Deployment', () => {
  test('Happy path', async () => {
    const contract = new Contract("./two_prime_factors.json");

    const deployTxHash = await contract.deployWithDatum(0);
    await waitUntilDeployed(deployTxHash);

    let deployed_successfully = false;
    try {
      const spendTxHash = await contract.spend(0, deployTxHash);
      deployed_successfully = true;
    } catch {}

    expect(deployed_successfully).toBe(true);
  }, 100000);
});

async function waitUntilDeployed(deployTxHash: string) {
  let is_deployed = false;
  while (!is_deployed) {
    try {
      await blockchainProvider.fetchUTxOs(deployTxHash);
      is_deployed = true;
    } catch {
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  };
}
