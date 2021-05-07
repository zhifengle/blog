// Math.min(...numbers)
function minArray(numbers: number[]): number {
  let left = 0;
  let right = numbers.length - 1;
  while (left < right) {
    // 对于 C++ 之类的减法不容易出现溢出的情况
    let pivot = left + ((right - left) >> 1);
    if (numbers[pivot] < numbers[right]) {
      right = pivot;
    } else if (numbers[pivot] > numbers[right]) {
      left = pivot + 1;
    } else {
      right -= 1;
    }
  }
  return numbers[left];
}
