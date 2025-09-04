import {MConStr} from "@meshsdk/common";
import {Data, mConStr0} from "@meshsdk/core";

type Proof = MConStr<any, string[]>;

type ZKRedeemer = MConStr<any, Data[] | Proof[]>;

function mProof(piA: string, piB: string, piC: string): Proof {
    if (piA.length != 96 || piB.length != 192 || piC.length != 96) {
        throw new Error("Wrong proof");
    }

    return mConStr0([piA, piB, piC]);
}

export function mZKRedeemer(redeemer: Data): ZKRedeemer {
    return mConStr0([redeemer, proofs()]);
}

function proofs(): Proof[] {
    return [
		mProof(
			"b2d9ba8ef640452abb56bf59572b7addabbd483461293b9ae94a7a345572e26f7108768867bc06cf1f4ab1570af1d1dc",
			"a851858665018bdcd7bc57ec9b7dfc9fe9f1ea6ba44e5661c78f03141fd0cc5fa3f4027f6c1184579f8bb5354a624fe209815bb745d533cb525ec21873f961954cb5dd5df06a84f602b025d1a571b6a2abf941a7b8c11f2c717c77fc4c5acd14",
			"b384d3ba52f31d2d50b7e4592e6cf79dcd56a8a88957dd470dc8086eece023f815de9c01b124b69c3422e7337c264427",
		),
    ];
}
