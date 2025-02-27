"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
class CborMap extends Map {
    constructor() {
        super(...arguments);
        this.byteSpan = [0, 0];
    }
    setByteSpan(byteSpan) {
        this.byteSpan = byteSpan;
    }
    getByteSpan() {
        return this.byteSpan;
    }
}
exports.default = CborMap;
