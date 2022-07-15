#!/usr/bin/env node

import path from 'path';
import fs from 'fs';
import { Command, Option } from 'commander';
import { nyaa, mikanme, GetItemsFn, yiyiwu } from '../site';
import { randomSleep } from '../utils/utils';
import { fetchInstance, initDefaultOption } from '../utils/fetchData';
import { loggerFactory } from '../utils/logger';

type RssName = 'mikanme' | 'nyaa';

type Rss = {
  name: string;
  url: string;
  filter?: string;
};

type RssConfig = {
  [key in RssName]?: Rss[];
};

const rssFnDict: Record<RssName, GetItemsFn> = {
  nyaa: nyaa.getItems,
  mikanme: mikanme.getItems,
};

async function executeRssTask(name: RssName, rssList: Rss[]) {
  for (const rss of rssList) {
    const contents = await fetchInstance(rss.url);
    const items = await rssFnDict[name](contents, rss.filter);
    // @TODO 分段; 以及新的离线
    try {
      await yiyiwu.addOfflineTask(items.map((item) => item.magnet));
      items.forEach((item) => {
        logger.info(`[115] [${name}] ${rss.name} ${item.title} ${item.magnet}`);
      });
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
}

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
  .action(async (options) => {
    initDefaultOption();
    let rssConfig: RssConfig = {};
    try {
      rssConfig = JSON.parse(fs.readFileSync(options.rss, 'utf-8'));
    } catch (error) {
      console.error(error);
    }
    // var currentPath = process.cwd();
    await Promise.all(
      Object.entries(rssConfig).map(([key, list]) =>
        executeRssTask(key as RssName, list)
      )
    );

    // fs.writeFileSync(options.rss, JSON.stringify(rssConfig, null, 2));
  });

program.parse();
