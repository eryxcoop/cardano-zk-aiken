export = Signature;
/**
 * @param {EDDSA} eddsa - eddsa instance
 * @param {Array<Bytes>|Object} sig -
 * @param {Array<Bytes>|Point} [sig.R] - R point as Point or bytes
 * @param {Array<Bytes>|bn} [sig.S] - S scalar as bn or bytes
 * @param {Array<Bytes>} [sig.Rencoded] - R point encoded
 * @param {Array<Bytes>} [sig.Sencoded] - S scalar encoded
 */
declare function Signature(eddsa: any, sig: Array<any> | Object): void;
declare class Signature {
    /**
     * @param {EDDSA} eddsa - eddsa instance
     * @param {Array<Bytes>|Object} sig -
     * @param {Array<Bytes>|Point} [sig.R] - R point as Point or bytes
     * @param {Array<Bytes>|bn} [sig.S] - S scalar as bn or bytes
     * @param {Array<Bytes>} [sig.Rencoded] - R point encoded
     * @param {Array<Bytes>} [sig.Sencoded] - S scalar encoded
     */
    constructor(eddsa: any, sig: Array<any> | Object);
    eddsa: any;
    _R: any;
    _S: any;
    _Rencoded: any;
    _Sencoded: any;
    toBytes(): any;
    toHex(): any;
}
