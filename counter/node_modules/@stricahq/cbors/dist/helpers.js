"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.CborTag = exports.IndefiniteArray = exports.IndefiniteMap = void 0;
/* eslint-disable max-classes-per-file */
class IndefiniteMap extends Map {
}
exports.IndefiniteMap = IndefiniteMap;
class IndefiniteArray extends Array {
}
exports.IndefiniteArray = IndefiniteArray;
var CborTag_1 = require("./CborTag");
Object.defineProperty(exports, "CborTag", { enumerable: true, get: function () { return __importDefault(CborTag_1).default; } });
