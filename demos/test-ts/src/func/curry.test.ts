import { curry } from "./curry";
test("test", () => {
  const mockFn = jest.fn((a, b, c) => [a, b, c]);
  const curryedFn = curry(mockFn);
  // curryedFn("a", "b", "c");
  curryedFn("a")("b")("c");
  expect(mockFn.mock.calls.length).toBe(1);
  expect(mockFn.mock.results[0].value).toEqual(["a", "b", "c"]);
});
