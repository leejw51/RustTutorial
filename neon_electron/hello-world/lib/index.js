const addon = require('../native/index.node');

console.log(addon.helloWorld());

module.exports = addon;
