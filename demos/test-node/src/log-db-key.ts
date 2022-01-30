import CryptoJS from 'crypto-js';
import { genRandomStr } from './utils/utils';

/**
 * AES-256-ECB对称加密
 * @param text {string} 要加密的明文
 * @param secretKey {string} 密钥，32位随机大小写与数字
 * @returns {string} 加密后的密文，Base64格式
 */
function AES_ECB_ENCRYPT(text: string, secretKey: string) {
  var keyHex = CryptoJS.enc.Base64.parse(secretKey);
  var messageHex = CryptoJS.enc.Utf8.parse(text);
  var encrypted = CryptoJS.AES.encrypt(messageHex, keyHex, {
    mode: CryptoJS.mode.ECB,
    padding: CryptoJS.pad.Pkcs7,
  });
  return encrypted.toString();
}

// 注释随机生成
const key = genRandomStr(32);

/*
 {
    "115.com": {
        "name": "xxx",
        "pw": "sss"
    },
}
 */
var message = JSON.stringify(require('./site.json'));

var ecbEncrypt = AES_ECB_ENCRYPT(message, key);

const fs = require('fs');
fs.writeFileSync('db.txt', `"${key}"\n\n"${ecbEncrypt}"`, 'utf-8');
