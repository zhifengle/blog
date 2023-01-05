import { readFileSync, writeFileSync } from 'fs';
import { isIP } from 'net';
import path from 'path';
import { fetchInfo } from '../utils/fetchData';

async function asyncPool<T, U>(
  limit: number,
  arr: T[],
  iteratorFn: (item: T) => Promise<U>
): Promise<U[]> {
  const ret = [];
  const executing: Promise<U>[] = [];
  for (const item of arr) {
    const p = Promise.resolve(iteratorFn(item));
    ret.push(p);
    const e: Promise<any> = p.then(() =>
      executing.splice(executing.indexOf(e), 1)
    );
    executing.push(e);

    if (executing.length >= limit) {
      await Promise.race(executing);
    }
  }
  return Promise.all(ret);
}

async function checkIp(ip: string, host: string) {
  try {
    const res = await fetchInfo(
      `http://${ip}/cdn-cgi/trace`,
      'text',
      {
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
          Host: host,
        },
      },
      3 * 1000
    );
    if (res.includes(`h=${host}`)) {
      return true;
    }
  } catch (error) {
    // console.error(error);
  }
}
async function startCheck() {
  // txt file path
  const ipTxt = String.raw`xx`;
  const ipStr = readFileSync(ipTxt, 'utf-8');
  // host
  const host = 'cloudflare.com';
  const check = async (ip: string) => {
    return await checkIp(ip, host);
  };
  const ipArr = ipStr.split('\n').map((s) => s.trim()).filter(s => isIP(s) !== 0);

  const res = await asyncPool(
    100,
    ipArr,
    check
  );
  const results: string[] = ipArr.filter((v, idx) => res[idx]);
  writeFileSync(
    path.join(path.dirname(ipTxt), 'output.txt'),
    results.join('\n')
  );
}

startCheck();
