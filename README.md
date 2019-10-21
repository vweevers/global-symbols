# global-symbols

> **Node.js addon to get global symbols of an ELF (linux) or Mach-O (mac) binary.**\
> Requires Rust.

Nothing to see here. Dipping my toes into Rust.

```js
const syms = require('global-symbols')

// Returns an array of symbol names
console.log(syms('./prebuilds/darwin-x64/node.napi.node'))
```
