import CryptoJS from 'crypto-js';

export function decodeBase64(str: string): string {
  return CryptoJS.enc.Base64.parse(str).toString(CryptoJS.enc.Utf8);
}

export function encodeBase64(str: string): string {
  return CryptoJS.enc.Utf8.parse(str).toString(CryptoJS.enc.Base64);
}

export function encryptAesEcb(content: string, key: string): string {
  var parsedKey = CryptoJS.enc.Utf8.parse(key);
  var parsedContent = CryptoJS.enc.Utf8.parse(content);
  var encrypted = CryptoJS.AES.encrypt(parsedContent, parsedKey, {
    mode: CryptoJS.mode.ECB,
    padding: CryptoJS.pad.Pkcs7,
  });
  return encrypted.toString();
}

export function decryptAesEcb(content: string, key: string): string {
  var parsedKey = CryptoJS.enc.Utf8.parse(key);
  var decrypt = CryptoJS.AES.decrypt(content, parsedKey, {
    mode: CryptoJS.mode.ECB,
    padding: CryptoJS.pad.Pkcs7,
  });
  return CryptoJS.enc.Utf8.stringify(decrypt).toString();
}
