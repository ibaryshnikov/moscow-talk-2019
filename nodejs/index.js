const wasm = require('./pkg');

const array = [1, 2, 3];
const result = wasm.updateArray(array);

console.log('array', array);
console.log('result', result);
