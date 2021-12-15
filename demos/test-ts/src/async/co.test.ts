import { co } from './co';
const ctx = {
  name: 'xxx',
};
test('test this', (done) => {
  co.call(ctx, function* () {
    expect(this).toEqual(ctx);
    done();
  });
});

// https://github.com/tj/co/blob/master/test/generators.js
describe('co(*)', function () {
  describe('with a generator function', function () {
    it('should wrap with co()', function () {
      const res = co(function* () {
        // ts 4.2 必须设置 type
        // 或者返回值 Generator<number, void, string>
        var a: number = yield 1;
        var b: number = yield 1;
        return [a, b];
      });
      expect(res).resolves.toEqual([1, 1]);
    });
  });
});
