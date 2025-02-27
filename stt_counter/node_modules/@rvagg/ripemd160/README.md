# @rvagg/ripemd160

Browser-safe `ripemd160` for JavaScript.

This project is a fork of https://github.com/crypto-browserify/ripemd160 (and its parent, https://github.com/crypto-browserify/hash-base) but it:

* Adds TypeScript types exports
* Operates natively on `Uint8Array`s
* Has no dependencies, even in the browser (i.e. no `Buffer`)
* Does not handle streaming operations (i.e. just use `update()` and `digest()`)

## Example

```js
var RIPEMD160 = require('ripemd160')

console.log(new RIPEMD160().update('42').digest('hex'))
// => 0df020ba32aa9b8b904471ff582ce6b579bf8bc8
```

## License & Copyright

MIT

Copyright (c) 2016 crypto-browserify
Copyright (c) 2022 Rod Vagg
