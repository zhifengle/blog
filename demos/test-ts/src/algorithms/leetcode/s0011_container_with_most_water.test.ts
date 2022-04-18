import { maxArea } from './s0011_container_with_most_water';

test('s0011_container_with_most_water', () => {
  expect(maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7])).toBe(49);
  expect(maxArea([6, 9])).toBe(6);
  expect(maxArea([1, 1, 2, 1, 1, 1])).toBe(5);
});
