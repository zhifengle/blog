import {
  decodeBase64,
  decryptAesEcb,
  encodeBase64,
  encryptAesEcb,
} from './crypto';
import { genRandomStr } from './utils';

describe('crypto', () => {
  test('base64', () => {
    var encryptStr = 'aHR0cHM6Ly93d3cuYmFpZHUuY29tLw==';
    var content = 'https://www.baidu.com/';

    expect(encodeBase64(content)).toBe(encryptStr);
    expect(decodeBase64(encryptStr)).toBe(content);
  });
  test('aes ecb', () => {
    // 注释随机生成
    const key = genRandomStr(32);
    const content = '1';
    const encryptStr = encryptAesEcb(content, key);
    expect(encryptAesEcb('1', key)).toEqual(encryptStr);
    expect(decryptAesEcb(encryptStr, key)).toBe('1');
  });
});
