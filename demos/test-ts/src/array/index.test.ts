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
})
