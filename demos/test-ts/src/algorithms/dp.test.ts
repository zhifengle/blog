import { minDistance } from "./dp";

test("minDistance", () => {
  expect(minDistance("horse", "ros")).toEqual(3);
  expect(minDistance("intention", "execution")).toEqual(5);
});
