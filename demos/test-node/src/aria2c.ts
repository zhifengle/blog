import { Command } from 'commander';
import path from 'path';
import { fetchInfo } from './utils/fetchData';

// https://github.com/sonnyp/aria2.js
const homedir = require('os').homedir();
const downloadPath = path.join(homedir, 'Downloads');
const program = new Command('tingshu');
program.version('0.0.1').description('A downloader for lrts');
program
  .requiredOption('-u, --url <type>', 'lrts book url')
  .option('-p, --path <type>', 'the path for saving audio', downloadPath)
  // .argument('<start>', 'start chapter')
  // .argument('[end]', 'last chapter', 0)
  .action(async (options) => {
    try {
      console.log(options);
      // console.log(start, end);
      // await run(reStr, dir, options);
    } catch (error) {
      console.error(error.message);
    }
  });
program.parse();

type InfoJson = {
  name: string;
  section: string;
} & any;

async function getAudioUrl(
  info: InfoJson,
  entityId: string,
  entityType: number
): Promise<string> {
  const res = await fetchInfo(
    `http://m.lrts.me/ajax/getPlayPath?entityId=${entityId}&entityType=${entityType}&opType=1&sections=[${info.section}]&type=0`,
    'json'
  );
  console.log(res.msg);
  return res?.list[0]?.path || '';
}

async function getBookMenu(entityId: string) {
  const res = await fetchInfo(
    `http://m.lrts.me/ajax/getBookMenu?bookId=${entityId}&pageNum=1&pageSize=5000&sortType=0`,
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

function genAudioExt(url: string): string {
  if (url.includes('m4a')) {
    return 'm4a';
  }
  return 'mp3';
}

function getBookList(list: InfoJson[], start: number, end: number) {
  return list.slice(start, end === 0 ? undefined : end);
}
