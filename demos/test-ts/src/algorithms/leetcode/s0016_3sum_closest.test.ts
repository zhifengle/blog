function threeSumClosest(nums: number[], target: number): number {
  // 可以改成 closet; ref: Rust 版本
  // 保存绝对值状态 .通过比较绝对值.如果绝对值更小，那么更新 closet 和 绝对值
  let minDistance = Number.MAX_SAFE_INTEGER;
  nums.sort((a, b) => a - b);
  for (let i = 0; i < nums.length; i++) {
    if (i > 0 && nums[i] === nums[i - 1]) {
      continue;
    }
    let lo = i + 1;
    let hi = nums.length - 1;
    while (lo < hi) {
      const sum = nums[i] + nums[lo] + nums[hi];
      const subVal = sum - target;
      if (subVal > 0 && minDistance > 0) {
        minDistance = Math.min(subVal, minDistance);
      } else if (subVal < 0 && minDistance < 0) {
        minDistance = Math.max(subVal, minDistance);
      } else {
        const minVal = Math.min(Math.abs(subVal), Math.abs(minDistance));
        minDistance = Math.abs(subVal) === minVal ? subVal : minDistance;
      }
      if (sum < target) {
        lo++;
      } else if (sum > target) {
        hi--;
      } else if (sum === target) {
        return sum;
      }
    }
  }

  return target + minDistance;
}

test('s0016_3sum_closest', () => {
  expect(threeSumClosest([-1, 2, 1, -4], 2)).toBe(2);
  expect(threeSumClosest([-1, 2, 1, -4], 1)).toBe(2);
  expect(threeSumClosest([0, 0, 0], 1)).toBe(0);
});
