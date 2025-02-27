export = RIPEMD160;
declare class RIPEMD160 {
    /** @private */
    private _a;
    /** @private */
    private _b;
    /** @private */
    private _c;
    /** @private */
    private _d;
    /** @private */
    private _e;
    /** @private */
    private _block;
    /** @private */
    private _blockSize;
    /** @private */
    private _blockOffset;
    /** @private */
    private _length;
    /** @private */
    private _finalized;
    /**
     * @param {Uint8Array|string} data
     * @returns {RIPEMD160}
     */
    update(data: Uint8Array | string): RIPEMD160;
    /** @private */
    private _update;
    /**
     * @param {'hex'} [encoding]
     * @returns {Uint8Array|string}
     */
    digest(encoding?: "hex" | undefined): Uint8Array | string;
    /**
     * @returns {Uint8Array}
     */
    _digest(): Uint8Array;
}
//# sourceMappingURL=index.d.ts.map