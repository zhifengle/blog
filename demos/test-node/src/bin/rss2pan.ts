#!/usr/bin/env node

import fs from 'fs';
import { Command, Option } from 'commander';
import { nyaa, mikanme, GetItemsFn, yiyiwu, dmhy } from '../site';
import { randomSleep } from '../utils/utils';
import {
  fetchInstance,
  initDefaultOption,
  initDefaultOptionFirefox,
} from '../utils/fetchData';
import { loggerFactory } from '../utils/logger';
import { RssService, SiteStatusService } from '../rss';
import { JsonEngine, KvExpiration } from 'kv-expiration';

type RssName = 'mikanani.me' | 'nyaa.si' | 'share.dmhy.org' | 'sukebei.nyaa.si';

type Rss = {
  name: string;
  url: string;
  cid?: string;
  filter?: string;
};

type RssConfig = {
  [key in RssName]?: Rss[];
};

function getRssConfigByURL(rssConfig: RssConfig, url: string): RssConfig {
  const urlObj = new URL(url);
  const config: RssConfig = {
    [urlObj.host]: [
      {
        name: url,
        url,
      },
    ],
  };
  if (rssConfig.hasOwnProperty(urlObj.host)) {
    let rss = rssConfig[urlObj.host as RssName].find((c) => c.url === url);
    if (rss) {
      return {
        [urlObj.host]: [rss],
      };
    }
  }
  return config;
}

const rssFnDict: Record<RssName, GetItemsFn> = {
  'nyaa.si': nyaa.getItems,
  'sukebei.nyaa.si': nyaa.getItems,
  'mikanani.me': mikanme.getItems,
  'share.dmhy.org': dmhy.getItems,
};

async function getRssItems(name: RssName, rss: Rss) {
  const contents = await fetchInstance(rss.url);
  const items = await rssFnDict[name](contents, rss.filter);
  const arr = await Promise.all(
    items.map((item) => rssSerivce.isItemExist(item.magnet))
  );
  return items.filter((item, idx) => arr[idx] === false);
}

async function executeRssTask(
  name: RssName,
  rssList: Rss[],
  options: { url?: string } & Record<string, any>
) {
  // const saveTasks: Promise<any>[] = [];
  for (const rss of rssList) {
    if (!options.url && rssUrlExpiration.get(rss.url)) {
      continue;
    }
    const items = await getRssItems(name, rss);
    if (!items || items.length === 0) {
      await randomSleep();
      continue;
    }
    // 三天不请求
    rssUrlExpiration.set(rss.url, true, 3);
    try {
      // @TODO 分段; 以及新的离线
      await yiyiwu.addOfflineTask(
        items.map((item) => item.magnet),
        rss.cid
      );
      items.forEach((item) => {
        logger.info(`[115] [${name}] ${rss.name} ${item.title} ${item.magnet}`);
      });
      await rssSerivce.saveItems(
        items.map((item) => {
          item.done = true;
          return item;
        })
      );
    } catch (error) {
      let msg = '';
      if (typeof error === 'string') {
        msg = error;
      } else {
        msg = error?.message;
      }
      if (msg.includes('abnormal operation')) {
        await siteStatusService.updateStatus('115.com', {
          abnormalOp: true,
        });
      } else if (msg.includes('need login')) {
        await siteStatusService.updateStatus('115.com', {
          needLogin: true,
        });
      }
      msg && logger.error(msg);
    }
    await randomSleep(1000, 500);
  }
}

const engine = new JsonEngine('expiration.json');
const rssUrlExpiration = new KvExpiration(engine, 'RSS_URL_');

const rssSerivce = new RssService();
const siteStatusService = new SiteStatusService();
const logger = loggerFactory('rss2pan', './');
const program = new Command('rss2pan');
program.version('0.0.1').description('Add magnet to wangpan');
program
  .addOption(
    new Option('-c, --chrome', 'use chrome cookies').conflicts('firefox')
  )
  .addOption(
    new Option('-f, --firefox <type>', 'firefox cookies path').conflicts(
      'chrome'
    )
  )
  .option('-y, --yiyiwu', '115')
  .option('-r --rss [rss]', 'rss config path', './rss.json')
  .option('-u --url [ur]', 'rss url')
  .action(async (options) => {
    const ready = await siteStatusService.isReady('115.com');
    if (!ready) {
      console.error('115.com 需要浏览器操作');
      return;
    }
    if (options.firefox) {
      await initDefaultOptionFirefox(options.firefox);
    } else {
      initDefaultOption();
    }
    let rssConfig: RssConfig = {};
    try {
      rssConfig = JSON.parse(fs.readFileSync(options.rss, 'utf-8'));
    } catch (error) {
      console.error(error);
    }
    let config: RssConfig = rssConfig;
    if (options.url) {
      config = getRssConfigByURL(rssConfig, options.url);
    }
    // var currentPath = process.cwd();
    await Promise.all(
      Object.entries(config).map(([key, list]) =>
        executeRssTask(key as RssName, list, options)
      )
    );

    // fs.writeFileSync(options.rss, JSON.stringify(rssConfig, null, 2));
  });

program
  .command('reset')
  .description('重置网站状态')
  .argument('<host>', 'target host')
  .action(async (host) => {
    await siteStatusService.resetStatus(host);
  });
program
  .command('fetch')
  .description('保存 rss 数据到数据库')
  .argument('<name>', 'rss name')
  .action(async (name) => {
    initDefaultOption();
    let rssConfig: RssConfig = {};
    try {
      rssConfig = JSON.parse(fs.readFileSync('rss.json', 'utf-8'));
    } catch (error) {
      console.error(error);
    }
    const list: Rss[] = rssConfig[name as RssName];
    if (!list || list.length === 0) {
      console.error('输入的 rss name 不对');
      return;
    }
    for (const rss of list) {
      const items = await getRssItems(name, rss);
      if (!items || items.length === 0) {
        await randomSleep();
        continue;
      }
      try {
        await rssSerivce.saveItems(
          items.map((item) => {
            // item.done = true;
            return item;
          })
        );
      } catch (error) {
        let msg = '';
        if (typeof error === 'string') {
          msg = error;
        } else {
          msg = error?.message;
        }
        msg && logger.error(msg);
      }
      await randomSleep(1000, 500);
    }
  });
program.parse();
