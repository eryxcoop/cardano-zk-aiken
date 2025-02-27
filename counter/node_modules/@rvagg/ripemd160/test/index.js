/* globals it */

/** @type {{input:string, ripemd160:string}[]} */
// @ts-ignore
const vectors = require('hash-test-vectors')
const { base64 } = require('multiformats/bases/base64')
const { assert } = require('chai')
const RIPEMD160 = require('../')

vectors.forEach((vector, i) => {
  const input = base64.baseDecode(vector.input)

  it('vector #' + (i + 1) + '', () => {
    assert.strictEqual(new RIPEMD160().update(input).digest('hex'), vector.ripemd160)
  })
})

it('42', () => {
  assert.strictEqual(new RIPEMD160().update('42').digest('hex'), '0df020ba32aa9b8b904471ff582ce6b579bf8bc8')
})
