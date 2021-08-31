const mergeTwoArray = (arr1: number[], arr2: number[]): number[] => {
  const results: number[] = [];
  while (arr1.length || arr2.length) {
    let b = arr1;
    if (arr1 < arr2) {
      b = arr2;
    }
    results.push(b[0]);
    b.shift();
  }
  return results;
};
const compareTwoArray = (arr1: number[], arr2: number[]): boolean => {
  const len1 = arr1.length;
  const len2 = arr2.length;
  let i = 0;
  let j = 0;
  for (; i < len1 && j < len2; i++, j++) {
    if (arr1[i] > arr2[j]) return true;
    if (arr1[i] < arr2[j]) return false;
  }
  return i < len1;
};

const findMaxSubArray = (nums: number[], k: number): number[] => {
  if (k >= nums.length) return [...nums];
  const result = Array.from(Array(k), () => 0);
  let idx = 0;
  for (let i = 0; i < nums.length; i++) {
    while (idx > 0 && k - idx < nums.length - i && result[idx - 1] < nums[i]) {
      idx--;
    }
    if (idx < k) {
      result[idx++] = nums[i];
    }
  }
  return result;
};

function maxNumber(nums1: number[], nums2: number[], k: number): number[] {
  let result: number[] = [];

  for (let i = 0; i <= k; i++) {
    if (nums1.length < i || nums2.length < k - i) continue;
    const sub1 = findMaxSubArray(nums1, i);
    const sub2 = findMaxSubArray(nums2, k - i);
    const merge = mergeTwoArray(sub1, sub2);
    if (compareTwoArray(merge, result)) {
      result = merge;
    }
  }

  return result;
}

// console.log(mergeTwoArray([6], [9, 5, 8, 3]));

const findMaxSubArray2 = (nums: number[], k: number): number[] => {
  if (k >= nums.length) return [...nums];
  const stack: number[] = [];
  let drop = nums.length - k;
  for (const n of nums) {
    while (drop && stack.length && stack[stack.length - 1] < n) {
      stack.pop();
      drop--;
    }
    stack.push(n);
  }

  return stack.slice(0, k);
};

// console.log(maxNumber([3, 4, 6, 5], [9, 1, 2, 5, 8, 3], 5));
// console.log(maxNumber([6, 7], [6, 0, 4], 5));
// console.log(maxNumber([3, 9], [8, 9], 3));
console.log(maxNumber([8, 6, 9], [1, 7, 5], 3));
