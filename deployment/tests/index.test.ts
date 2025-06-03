import { Contract } from "../utils";

describe('Contract Deployment', () => {
  test('Happy path', async () => {
    let deployed_successfully = false;
    const contract = new Contract("./two_prime_factors.json");

    const txHashPromise = contract.deployWithDatum(0);
    txHashPromise.then((_) => deployed_successfully = true);
    await txHashPromise;

    expect(deployed_successfully).toBe(true);
  });
});