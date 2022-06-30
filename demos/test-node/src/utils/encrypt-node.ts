import crypto from 'crypto';

// https://gist.github.com/siwalikm/8311cf0a287b98ef67c73c1b03b47154
// 32
const ENC_KEY = 'bf3c199c2470cb477d907b1e0917c17b';
// 16
const IV = '5183666c72eec9e4';

export const encrypt = (val: string) => {
  let cipher = crypto.createCipheriv('aes-256-cbc', ENC_KEY, IV);
  let encrypted = cipher.update(val, 'utf8', 'base64');
  encrypted += cipher.final('base64');
  return encrypted;
};

export const decrypt = (encrypted: string) => {
  let decipher = crypto.createDecipheriv('aes-256-cbc', ENC_KEY, IV);
  let decrypted = decipher.update(encrypted, 'base64', 'utf8');
  return decrypted + decipher.final('utf8');
};
