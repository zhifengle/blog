function threeSum2(nums: number[]): number[][] {
  const res: number[][] = [];
  // 注意:  nums.sort() 不是按照数字大小排序的
  nums.sort((a, b) => a - b);

  for (let i = 0; i < nums.length; i++) {
    if (i > 0 && nums[i] === nums[i - 1]) {
      continue;
    }
    let lo = i + 1;
    let hi = nums.length - 1;
    while (lo < hi) {
      const sum = nums[i] + nums[lo] + nums[hi];
      if (sum > 0) {
        hi--;
      } else if (sum < 0) {
        lo++;
      } else {
        res.push([nums[i], nums[lo], nums[hi]]);
        lo++;

        while (nums[lo] === nums[lo - 1] && lo < hi) {
          lo++;
        }
      }
    }
  }

  return res;
}

function threeSum(nums: number[]): number[][] {
  const result: number[][] = [];
  nums.sort((a, b) => a - b);
  for (let i = 0; i < nums.length; i++) {
    // 3数之和为 0. 第一个都大于0了。所以后面的不用比较了
    if (nums[i] > 0) {
      break;
    }
    // edge 去重
    if (i > 0 && nums[i - 1] == nums[i]) {
      continue;
    }
    let lo = i + 1;
    let hi = nums.length - 1;
    while (lo < hi) {
      const sum = nums[i] + nums[lo] + nums[hi];
      if (sum < 0) {
        lo++;
      } else if (sum > 0) {
        hi--;
      } else {
        result.push([nums[i], nums[lo], nums[hi]]);
        // edge 去重
        while (lo < hi && nums[lo + 1] == nums[lo]) {
          lo++;
        }
        // edge 去重
        while (lo < hi && nums[hi - 1] == nums[hi]) {
          hi--;
        }
        lo++;
        hi--;
      }
    }
  }

  return result;
}

test('s0015_3sum', () => {
  expect(threeSum([-1, 0])).toEqual([]);
  expect(threeSum([-1, 0, 1, 2, -1, -4])).toEqual([
    [-1, -1, 2],
    [-1, 0, 1],
  ]);
  expect(threeSum([-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4])).toEqual([
    [-4, 0, 4],
    [-4, 1, 3],
    [-3, -1, 4],
    [-3, 0, 3],
    [-3, 1, 2],
    [-2, -1, 3],
    [-2, 0, 2],
    [-1, -1, 2],
    [-1, 0, 1],
  ]);
});
