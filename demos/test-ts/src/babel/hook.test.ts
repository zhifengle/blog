import { noopMethod } from './hook';

test('test hook', () => {
  const test = {
    log: function () {
      console.log('log');
    },
    log2: function () {
      console.log('log2');
    },
  };
  const obj = noopMethod(test, 'log');
  obj.log();
  obj.log2();
});
