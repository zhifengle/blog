import { co } from './co'
const ctx = {
  name: 'xxx',
}
test('test this', (done) => {
  co.call(ctx, function* () {
    expect(this).toEqual(ctx)
    done()
  })
})

// https://github.com/tj/co/blob/master/test/generators.js
describe('co(*)', function () {
  describe('with a generator function', function () {
    it('should wrap with co()', function () {
      const res = co(function* () {
        var a = yield 1
        var b = yield 1
        return [a, b]
      })
      expect(res).resolves.toEqual([1, 1])
    })
  })
})
