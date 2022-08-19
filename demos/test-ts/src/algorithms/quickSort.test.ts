import { quickSort, quickSortHoare, quickSortArr } from './quickSort';

describe('quick sort', () => {
  test('quick', () => {
    const a1 = [3, 7, 8, 5, 2, 1, 9, 5, 4];
    quickSort<number>(a1, 0, a1.length - 1);
    expect(a1).toEqual([1, 2, 3, 4, 5, 5, 7, 8, 9]);
  });

  test('quick Hoare', () => {
    const a1 = [3, 7, 8, 5, 2, 1, 9, 5, 4];
    quickSortHoare(a1);
    expect(a1).toEqual([1, 2, 3, 4, 5, 5, 7, 8, 9]);
  });
  test('quick divide and conquer', () => {
    const a1 = [3, 7, 8, 5, 2, 1, 9, 5, 4];
    const arr = quickSortArr(a1);
    expect(arr).toEqual([1, 2, 3, 4, 5, 5, 7, 8, 9]);
  });
});
