import { Database } from 'sqlite3';
import os from 'os';
import path from 'path';

// 只支持 Windows
function getChromeCookiesPath(userDataPath?: string) {
  if (os.platform() !== 'win32') {
    throw new Error('Only Windows are supported');
  }
  const homedir = os.homedir();
  if (!userDataPath) {
    userDataPath = path.join(
      homedir,
      String.raw`AppData\Local\Google\Chrome\User Data`
    );
  }
  const subPath = 'Default/Network/Cookies';
  return path.join(userDataPath, subPath);
}

export class ChromeCookies {
  db: Database;
  constructor(storage: string = '') {
    if (!storage) {
      storage = getChromeCookiesPath();
    }
    this.db = new Database(storage);
  }
  getSiteCookie(url: string) {
    return new Promise((resolve, reject) => {
      const urlObj = new URL(url);
      this.db.all(
        `SELECT * FROM cookies WHERE host_key = ".${urlObj.host}" or host_key = "${urlObj.host}"`,
        (err, result) => {
          if (err) {
            reject(err);
            return;
          }
          // const fd = new URLSearchParams();
          // result.forEach((obj) => {
          //   fd.append(obj.name, obj.value);
          // });
          // resolve(fd.toString());
          resolve(result);
        }
      );
    });
  }
}

export class FirefoxCookies {
  db: Database;
  constructor(storage: string) {
    this.db = new Database(storage);
  }
  getSiteCookie(url: string) {
    return new Promise((resolve, reject) => {
      const urlObj = new URL(url);
      this.db.all(
        `SELECT * FROM moz_cookies WHERE host = ".${urlObj.host}" or host = "${urlObj.host}"`,
        (err, result) => {
          if (err) {
            reject(err);
            return;
          }
          // @TODO 是否需要 encodeURIComponent
          const res = result
            .map((obj) => `${obj.name}=${obj.value}`)
            .join('; ');
          resolve(res);
        }
      );
    });
  }
}
