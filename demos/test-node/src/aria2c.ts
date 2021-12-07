import { Command } from 'commander';
import path from 'path';
import { fetchInfo } from './utils/fetchData';
import { request } from './utils/request';
import { USER_SITE_CONFIG } from './utils/site-config';
import { randomSleep } from './utils/utils';

type InfoJson = {
  name: string;
  section: string;
} & any;

// aria2 task id
let taskId = 1;

// https://github.com/sonnyp/aria2.js
const homedir = require('os').homedir();
const downloadPath = path.join(homedir, 'Downloads');
const program = new Command('tingshu');
program.version('0.0.1').description('A downloader for lrts');
program
  .requiredOption('-u, --url <type>', 'lrts book url')
  .option('-p, --path <type>', 'the path for saving audio', downloadPath)
  .option('-t, --token <type>', 'aria2 secret token', '')
  .argument('<start>', 'start chapter')
  .argument('[end]', 'last chapter', 0)
  .action(async (start, end, options) => {
    try {
      // 在 site-config.ts 里面配置 Cookie 值
      const loginFlag = await isLogin();
      if (!loginFlag) {
        // 登录失效返回
        console.log('登录 Cookie 失效');
        return;
      }
      console.log('下载路径: ', options.path);
      console.log('=======================================');
      const url = options.url;
      const entityId = url.split('/').slice(-1)[0];
      let entityType;
      let getListFn: (...args: any) => Promise<InfoJson[]>;
      if (url.includes('album')) {
        entityType = 2;
        getListFn = getAlbumAudios;
      } else if (url.includes('book')) {
        entityType = 3;
        getListFn = getBookMenu;
      }
      let lst = await getListFn(entityId);
      lst = getBookList(lst, start, end);
      for (const info of lst) {
        const audioUrl = await getAudioUrl(info, entityId, entityType);
        const name = info.name + '.' + getAudioExt(audioUrl);
        if (audioUrl) {
          console.log('获取音频链接成功: ', name);
          sendToAria2(audioUrl, name, options.path, options.token);
        } else {
          console.log('获取音频链接失败: ', name);
        }
        // 随机等待 3-5 秒
        await randomSleep(5000, 3000);
      }
      taskId = 1;
      // console.log(start, end);
      // await run(reStr, dir, options);
    } catch (error) {
      console.error(error.message);
    }
  });
program.parse();

async function isLogin() {
  const res = await fetchInfo(`https://m.lrts.me/user`, 'text');
  // if (res.includes(''))
  if (res.includes('马上登录')) {
    return false;
  }
  return true;
}

async function getAudioUrl(
  info: InfoJson,
  entityId: string,
  entityType: number
): Promise<string> {
  const res = await fetchInfo(
    `https://m.lrts.me/ajax/getPlayPath?entityId=${entityId}&entityType=${entityType}&opType=1&sections=[${info.section}]&type=0`,
    'json'
  );
  console.log(res.msg);
  return res?.list[0]?.path || '';
}

async function getBookMenu(entityId: string, pageSize = 5000) {
  const res = await fetchInfo(
    `https://m.lrts.me/ajax/getBookMenu?bookId=${entityId}&pageNum=1&pageSize=${pageSize}&sortType=0`,
    'json'
  );
  return res?.list;
}

async function getAlbumAudios(entityId: string) {
  const res = await fetchInfo(
    `https://m.lrts.me/ajax/getAlbumAudios?ablumnId=${entityId}&sortType=0`,
    'json'
  );
  return res?.list;
}

function getAudioExt(url: string): string {
  // abc.mp3?token=null
  if (url.includes('.m4a')) {
    return 'm4a';
  }
  if (url.includes('.aac')) {
    return 'aac';
  }
  return 'mp3';
}

function getBookList(list: InfoJson[], start: number, end: number): InfoJson[] {
  return list.slice(start - 1, end === 0 ? undefined : end);
}

async function sendToAria2(
  url: string,
  filename: string,
  savePath: string,
  token = ''
) {
  let params: any[] = [
    [url],
    {
      out: filename,
      dir: savePath,
      split: '32',
      'max-connection-per-server': '5',
      'seed-ratio': '0.1',
      'user-agent': USER_SITE_CONFIG['m.lrts.me'].headers['user-agent'],
    },
  ];
  if (token) {
    params.unshift(`token:${token}`);
  }
  const res = await request.post('http://localhost:6800/jsonrpc', {
    jsonrpc: '2.0',
    method: 'aria2.addUri',
    id: taskId,
    params,
  });
  if (res.status !== 200) {
    console.log(`添加任务: ${filename} 失败`);
  } else if (res.status === 200) {
    console.log(`添加任务: ${filename} 成功`);
  }
  taskId++;
  return res;
}
