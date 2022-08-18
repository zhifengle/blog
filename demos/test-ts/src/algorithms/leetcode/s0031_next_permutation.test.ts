function nextPermutation(nums: number[]): void {
  let i = nums.length - 1;
  let j = nums.length - 1;
  // 找到递增的序列 比如   2  3.   i - 1 是索引
  // [1,2,3,4,5]   4,5 就是递增
  while (i > 0 && nums[i - 1] >= nums[i]) {
    i -= 1;
  }
  if (i > 0) {
    // i..len-1 里面比 i-1大的数字
    while (j >= i && nums[i - 1] >= nums[j]) {
      j -= 1;
    }
    // 交换两者
    [nums[i - 1], nums[j]] = [nums[j], nums[i - 1]];
  }
  // 反转
  j = nums.length - 1;
  while (i < j) {
    [nums[i], nums[j]] = [nums[j], nums[i]];
    i += 1;
    j -= 1;
  }
}

function nextPermutation2(nums: number[]): void {
  let i = nums.length - 2;
  let j = nums.length - 1;
  while (i >= 0 && nums[i] >= nums[i + 1]) {
    i -= 1;
  }
  if (i >= 0) {
    while (j > i && nums[i] >= nums[j]) {
      j -= 1;
    }
    // 这种交换方式和 [ ] 交换性能上面好像差别不大
    const temp = nums[i];
    nums[i] = nums[j];
    nums[j] = temp;
  }
  i = i + 1;
  j = nums.length - 1;
  while (i < j) {
    const temp = nums[i];
    nums[i] = nums[j];
    nums[j] = temp;
    i += 1;
    j -= 1;
  }
}

test('s0031', () => {
  const nums = [1, 2, 3, 4, 5];
  nextPermutation2(nums);
  expect(nums).toEqual([1, 2, 3, 5, 4]);
  const vec2 = [5, 4, 3, 2, 1];
  nextPermutation2(vec2);
  expect(vec2).toEqual([1, 2, 3, 4, 5]);
});
