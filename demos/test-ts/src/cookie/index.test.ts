import { fakeCookie } from "./index";

test("fake cookie", () => {
  fakeCookie.cookie = "a=22";
  fakeCookie.cookie = "b=123";
  expect(fakeCookie.cookie).toEqual("a=22;b=123");
});
