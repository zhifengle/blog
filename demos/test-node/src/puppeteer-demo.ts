/*
import puppeteer from 'puppeteer-core';
import path from 'path';

// https://www.npmjs.com/package/puppeteer-extra-plugin-stealth
// puppeteer.launch({ headless: true }).then(async (browser) => {
// });
// const proxy = 'http://127.0.0.1:1080';
(async () => {
  const browser = await puppeteer.launch({
    devtools: true,
    headless: false,
    // product: 'firefox',
    // executablePath: String.raw`C:\Program Files\Mozilla Firefox\firefox.exe`,
    // 不行
    // args: ['-P', 'default-release'],
    // Windows
    executablePath: path.resolve(
      process.env.USERPROFILE,
      String.raw`AppData\Local\Google\Chrome\Application\chrome.exe`
    ),
    // dumpio: true,
    ignoreDefaultArgs: ['--enable-automation'],
    args: [
      // `--proxy-server=${proxy}`,
      // 不能设置时尝试这个
      // '--disable-blink-features=AutomationControlled',
      `--user-data-dir=${path.resolve(
        process.env.USERPROFILE,
        String.raw`AppData\Local\Google\Chrome\profile1`
      )}`,
    ],
  });
  // const page = (await browser.pages())[0];
  const page = await browser.newPage();
  page.evaluateOnNewDocument(() => {
    Object.defineProperty(navigator, 'webdriver', { get: () => false });
  });
  // page.setUserAgent('')
  await page.goto('https://httpbin.org/ip');
  await page.close();
  await browser.close();
})();

*/
