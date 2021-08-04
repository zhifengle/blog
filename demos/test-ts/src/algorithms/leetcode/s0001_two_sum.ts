export const twoSum = function (nums: number[], target: number): number[] {
  const m = new Map();
  const r: number[] = [];
  for (let i = 0; i < nums.length; i++) {
    const num = nums[i];
    if (m.has(target - num)) {
      return [m.get(target - num), i];
    }
    m.set(num, i);
  }

  return r;
};
