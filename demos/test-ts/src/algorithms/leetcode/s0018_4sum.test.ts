// https://leetcode.com/problems/4sum/

function fourSum(nums: number[], target: number): number[][] {
  const res: number[][] = [];
  if (nums.length < 4) {
    return [];
  }
  nums.sort((a, b) => a - b);

  for (let i = 0; i < nums.length; i++) {
    if (i > 0 && nums[i] === nums[i - 1]) {
      continue;
    }
    const a = nums[i];
    for (let j = i + 1; j < nums.length; j++) {
      if (j > i + 1 && nums[j] === nums[j - 1]) {
        continue;
      }
      let lo = j + 1;
      let hi = nums.length - 1;
      while (lo < hi) {
        const sum = a + nums[j] + nums[lo] + nums[hi];
        if (sum < target) {
          lo++;
        } else if (sum > target) {
          hi--;
        } else {
          res.push([a, nums[j], nums[lo], nums[hi]]);
          lo++;
          while (lo < hi && nums[lo] === nums[lo - 1]) {
            lo++;
          }
        }
      }
    }
  }

  return res;
}

test('s0018_4sum', () => {
  expect(fourSum([2, 2, 2, 2, 2], 8)).toEqual([[2, 2, 2, 2]]);
});
