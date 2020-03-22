let a = require('./a');

console.log('B: before logging a');
console.log(a);
console.log('B: after logging a');

module.exports = {
    B: 'this is b Object'
};
