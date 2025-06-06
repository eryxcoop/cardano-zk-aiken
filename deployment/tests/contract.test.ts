import { Data, mConStr0 } from "@meshsdk/core";
import { Contract, mVoid } from "../contract";
import { BlockchainProvider } from "../blockchain_provider";

describe('Contract Deployment', () => {
  let scriptPath: string;
  beforeEach(() => {
    scriptPath = "./tests/plutus.json";
  })

  test('Happy path with datum and redeemer', async () => {
    const datum = 42;
    const redeemer = 42;
    await testDeploymentWith(scriptPath, 0, datum, redeemer);
  }, 100000);

  test.skip('Happy path without datum and redeemer', async () => {
    const datum = mVoid();
    const redeemer = mVoid();
    await testDeploymentWith(scriptPath, 2, datum, redeemer);
  }, 100000);

  test.skip('Happy path with manual budget', async () => {
    const datum = mVoid();
    const redeemer = mVoid();
    const redeemerBudget = { mem: 5000, steps: 25000000 };
    await testDeploymentWith(scriptPath, 2, datum, redeemer, redeemerBudget);
    /*
    const datum = 35;
    const redeemer = mConStr0([5, 7]);
    const redeemerBudget = { mem: 90742, steps: 4017641489 };
    await testDeploymentWith("./tests/two_prime_factors_number.json", 0, datum, redeemer, redeemerBudget);
    */
  }, 200000);
});

async function testDeploymentWith(scriptPath: string, validatorIndex: number, datum: Data, redeemer: Data, redeemerBudget?: {mem: number, steps: number}) {
  let deployed_successfully = false;
  const contract = new Contract(scriptPath);

  const deployTxHash = await contract.deploy(validatorIndex, datum);
  await waitUntilDeployed(deployTxHash);
  try {
    const spendTxHash = await contract.spend(validatorIndex, deployTxHash, redeemer, redeemerBudget);
    deployed_successfully = true;
    await waitUntilDeployed(spendTxHash)
  } catch {}

  expect(deployed_successfully).toBe(true);
}

async function waitUntilDeployed(deployTxHash: string) {
  const blockchainProvider = new BlockchainProvider();
  while (!await blockchainProvider.hasTxBeenImpactedOnBlockchain(deployTxHash)) {
    await new Promise(resolve => setTimeout(resolve, 1000));
  };
}
