import { quickSort } from './quickSort'

test('quick', () => {
  const a1 = [3, 7, 8, 5, 2, 1, 9, 5, 4]
  quickSort<number>(a1, 0, a1.length - 1)
  expect(a1).toEqual([1, 2, 3, 4, 5, 5, 7, 8, 9])
})
