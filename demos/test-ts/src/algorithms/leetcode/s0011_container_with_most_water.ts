export function maxArea(height: number[]): number {
  const size = height.length;
  let water = 0;
  let lo = 0;
  let hi = size - 1;

  while (lo < hi) {
    const h = Math.min(height[lo], height[hi]);
    water = Math.max(water, (hi - lo) * h);
    while (lo < hi && height[lo] <= h) {
      lo += 1;
    }
    while (lo < hi && height[hi] <= h) {
      hi -= 1;
    }
  }
  return water;
}
