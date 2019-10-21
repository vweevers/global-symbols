const binding = require('./native')

function syms (fp) {
  return binding.syms(fp).filter(nodeps)
}

function nodeps (sym) {
  return !/leveldb|snappy/.test(sym)
}

module.exports = syms

// console.log(syms('./prebuilds/darwin-x64/node.napi.node'))
// console.log(syms('./prebuilds/linux-x64/node.napi.node'))
