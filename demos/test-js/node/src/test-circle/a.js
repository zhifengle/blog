let b = require('./b');

console.log('A: before logging b');
console.log(b);
console.log('A: after logging b');

module.exports = {
    A: 'this is a Object'
};
