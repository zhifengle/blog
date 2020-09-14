import { flatten } from '.'
import { uniqueByKey } from './unique'

describe('test array utils', () => {
  test('unique by key', () => {
    expect(
      uniqueByKey(
        [
          {
            name: 'ok',
            value: 123,
          },
          {
            name: 'sa',
            value: 123,
          },
        ],
        'value'
      )
    ).toEqual([{ name: 'ok', value: 123 }])
  })
  test('flatten', () => {
    expect(flatten([1, [2, 3, [4]]])).toEqual([1, 2, 3, 4])
  })
})
