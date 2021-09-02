function minSubArrayLen(target: number, nums: number[]): number {
  let l = 0;
  let total = 0;

  let result = nums.length + 1;
  for (let i = 0; i < nums.length; i++) {
    total += nums[i];
    while (total >= target) {
      result = Math.min(result, i - l + 1);
      total -= nums[l];
      l += 1;
    }
  }
  if (result === nums.length + 1) {
    return 0;
  }
  return result;
}
