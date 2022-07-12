import path from 'path';
import os from 'os';
import Storage from './storage';
import { randomNum } from './utils/utils';
import { fetchInfo, fetchText, initDefaultOption } from './utils/fetchData';
import { loggerFactory } from './utils/logger';
import { JSDOM } from 'jsdom';

// node start
const homedir = os.homedir();
const logsPath = path.join(homedir, 'Documents/test/logs');
const CONFIG_FILE = 'qiandao-config.json';
const CONFIG_FILE_PATH = path.join(logsPath, `${CONFIG_FILE}`);
const storage = new Storage(CONFIG_FILE_PATH);
const GM_getValue = storage.getValue.bind(storage);
const GM_setValue = storage.setValue.bind(storage);
const logger = loggerFactory('qiandao', logsPath);
// config
initDefaultOption();
// web 使用 DOMParser; parser.parseFromString(htmlString, "text/html");
function getDocObj(htmlStr: string): Document {
  const dom = new JSDOM(htmlStr);
  return dom.window.document;
}

// https://stackoverflow.com/questions/54903199/how-to-ignore-ssl-certificate-validation-in-node-requests/54903835
// 推荐使用 httpsAgent
// process.env.NODE_TLS_REJECT_UNAUTHORIZED = '0';

// node end

type SiteConfig = {
  name: string;
  href: string | string[];
  hostname?: string | string[];
  signFn: () => Promise<void>;
};

const USERJS_PREFIX = 'E_USERJS_SIGN_';
const UPDATE_INTERVAL = 24 * 60 * 60 * 1000;
const ALL_SITES = 'ALL_SITES';

async function signSouth() {
  const site_name = this.name;
  const sign = async (taskId: number) => {
    const res = await fetchText(
      genUrl(
        this.href,
        `plugin.php?H_name=tasks&action=ajax&actions=job&cid=${taskId}`
      )
    );
    // 未登录
    if (res.match('您还不是论坛会员,请先登录论坛')) {
      logger.error(`${this.name} 需要登录`);
      return;
    }
    if (res.includes('success')) {
      await fetchText(
        genUrl(
          this.href,
          `plugin.php?H_name=tasks&action=ajax&actions=job2&cid=${taskId}`
        )
      );
      setSignResult('south-plus' + taskId, true);
    } else if (res.includes('拒离上次申请')) {
      // 已经签到了
      setSignResult('south-plus' + taskId, true);
    }
  };
  if (!getSignResult(site_name + '14', 7)) {
    await sign(14);
  } else {
    logger.info(`${site_name}周任务 已签到`);
  }
  if (!getSignResult(site_name + '15')) {
    await sign(15);
  } else {
    logger.info(`${site_name}日任务 已签到`);
  }
}

function setSignResult(site: string, result: boolean) {
  logger.info(`${site} 已签到`);
  GM_setValue(USERJS_PREFIX + site.toUpperCase(), {
    result: Number(result),
    date: +new Date(),
  });
}
function getSignResult(site: string, numOfDays?: number): boolean {
  let obj = GM_getValue(USERJS_PREFIX + site.toUpperCase());
  if (obj) {
    const now = new Date();
    const preDate = new Date(obj.date as any);
    // 存在时间限制
    if (numOfDays) {
      // 小于时间限制
      if (+now - +preDate < UPDATE_INTERVAL * numOfDays) {
        return Number(obj.result) === 1 ? true : false;
      } else {
        return false;
      }
    } else {
      return now.getDate() === preDate.getDate();
    }
  }
  return false;
}

function genUrl(href: string, pathname: string) {
  const url = new URL(pathname, href);
  return url.href;
}

const siteDict: SiteConfig[] = [
  {
    name: 'south-plus',
    href: 'https://www.south-plus.net/',
    signFn: signSouth,
  },
  /*
  {
    name: '52pojie',
    href: 'https://www.52pojie.cn/forum.php',
    async signFn() {
      if (getSignResult(this.name)) {
        logger.info(`${this.name} 已签到`);
        return;
      }
      const pathname = 'home.php?mod=task&do=apply&id=2';
      if (globalThis?.location?.href === this.href) {
        const $btn = document.querySelector(`a[href="${pathname}"`);
        if (!$btn) return;
      } else {
        const content = await fetchText(this.href, { decode: 'gbk' });
        // 未登录
        if (content.includes('注册[Register]')) {
          logger.error(`${this.name} 需要登录`);
          return;
        } else if (!content.includes(pathname)) {
          setSignResult(this.name, true);
          return;
        }
      }
      const qdText = await fetchText(genUrl(this.href, pathname));
      setSignResult(this.name, true);
    },
  },
  */
  {
    name: 'v2ex',
    href: ['https://v2ex.com/', 'https://www.v2ex.com/'],
    async signFn() {
      if (getSignResult(this.name)) {
        logger.info(`${this.name} 已签到`);
        return;
      }
      let href = this.href[0];
      const curHref = globalThis?.location?.href;
      if (curHref && this.href.includes(curHref)) {
        href = curHref;
      }
      const missionUrl = genUrl(href, 'mission/daily');
      const content = await fetchText(genUrl(href, 'mission/daily'));
      if (content.match(/你是机器人么？/)) {
        logger.error(`${this.name} 需要登录`);
        return;
      }
      if (content.match(/每日登录奖励已领取/)) {
        setSignResult(this.name, true);
        return;
      }
      const m = content.match(/mission\/daily\/redeem\?once=\d+/);
      // 为匹配到链接时，已签到
      if (m) {
        await fetchText(genUrl(href, m[0]), {
          headers: {
            Referer: missionUrl,
          },
        });
      } else {
        // logger.info(`${this.name} 已签到`);
      }
      setSignResult(this.name, true);
    },
  },
  {
    name: '2djgame',
    href: 'https://bbs4.2djgame.net/home/forum.php',
    async signFn() {
      if (getSignResult(this.name)) {
        logger.info(`${this.name} 已签到`);
        return;
      }
      const content = await fetchText(
        genUrl(this.href, 'home.php?mod=task&do=apply&id=1')
      );
      if (content.match('抱歉，本期您已申請過此任務，請下期再來')) {
        setSignResult(this.name, true);
        return;
      } else if (content.match('您需要先登錄才能繼續本操作')) {
        logger.error(`${this.name} 需要登录`);
        return;
      }
      setSignResult(this.name, true);
    },
  },
  {
    name: 'zodgame',
    href: 'https://zodgame.xyz/',
    async signFn() {
      if (getSignResult(this.name)) {
        logger.info(`${this.name} 已签到`);
        return;
      }
      const content = await fetchText(
        genUrl(this.href, 'plugin.php?id=dsu_paulsign:sign')
      );
      if (content.includes('您好！登录后享受更多精彩')) {
        logger.error(`${this.name} 需要登录`);
        return;
      }
      if (content.includes('您今天已经签到过了或者签到时间还未开始')) {
        setSignResult(this.name, true);
        return;
      }
      const formhashRe =
        /<input\s*type="hidden"\s*name="formhash"\s*value="([^"]+)"\s*\/?>/;
      const matchFormhash = content.match(formhashRe);
      if (
        matchFormhash &&
        /<form\s*id="qiandao"\s*method="post"/.test(content)
      ) {
        const url =
          'plugin.php?id=dsu_paulsign:sign&operation=qiandao&infloat=1&inajax=1';
        const fd = new URLSearchParams();
        fd.append('formhash', matchFormhash[1]);
        const arr = ['kx', 'ym', 'wl', 'nu', 'ch', 'fd', 'yl', 'shuai'];
        fd.append('qdxq', arr[randomNum(5, 0)]);
        const signRes = await fetchInfo(
          // genUrl(this.href, $form.getAttribute('action')),
          genUrl(this.href, url),
          'text',
          {
            method: 'POST',
            body: fd,
            headers: { 'content-type': 'application/x-www-form-urlencoded' },
          }
        );
        if (signRes.includes('未定义操作')) {
          return;
        } else if (signRes.includes('恭喜你签到成功')) {
          // 刷新
          await fetchText(genUrl(this.href, 'plugin.php?id=dsu_paulsign:sign'));
          setSignResult(this.name, true);
          return;
        }
      }
    },
  },
  {
    name: '2dfan',
    href: 'https://galge.fun/',
    async signFn() {
      if (getSignResult(this.name)) {
        logger.info(`${this.name} 已签到`);
        return;
      }
      const content = await fetchText('https://galge.fun');
      if (content.includes('已连续签到')) {
        setSignResult(this.name, true);
        return;
      } else if (content.includes('/users/not_authenticated')) {
        logger.error(`${this.name} 需要登录`);
        return;
      }
      const checkRes = await fetchInfo(genUrl(this.href, 'checkins'), 'json', {
        method: 'POST',
        data: null,
        headers: {
          'x-requested-with': 'XMLHttpRequest',
        },
      });
      if (!checkRes.checkins_count) {
        logger.error(`${this.name} 签到失败`);
        return;
      }
      setSignResult(this.name, true);
    },
  },
  {
    name: 'manhuabudang',
    href: 'https://www.manhuabudang.com/u.php',
    async signFn() {
      if (getSignResult(this.name)) {
        logger.info(`${this.name} 已签到`);
        return;
      }
      const content = await fetchText('https://www.manhuabudang.com/u.php', {
        decode: 'gbk',
      });
      const $doc = getDocObj(content);
      if ($doc.querySelector('button.card.card_old')) {
        setSignResult(this.name, true);
        return;
      } else if (content.includes('您没有登录或者您没有权限访问此页面')) {
        logger.error(`${this.name} 需要登录`);
        return;
      }
      const verifyhash = new Function(
        $doc.querySelector('head > script:last-child').innerHTML +
          ' return verifyhash;'
      )();
      const fd = new URLSearchParams();
      fd.append('step', '2');
      const checkRes = await fetchInfo(
        genUrl(this.href, 'jobcenter.php'),
        'text',
        {
          method: 'POST',
          decode: 'gbk',
          params: {
            // nowtime: 1639357663589
            action: 'punch',
            nowtime: +new Date(),
            verify: verifyhash,
          },
          headers: { 'content-type': 'application/x-www-form-urlencoded' },
          body: fd,
        }
      );
      if (checkRes.includes('你已经打卡') || checkRes.includes('漫画+2')) {
        setSignResult(this.name, true);
        return;
      } else {
        logger.error(`${this.name} 签到失败`);
      }
    },
  },
  {
    name: 'kafan',
    href: 'https://bbs.kafan.cn/',
    async signFn() {
      if (getSignResult(this.name)) {
        logger.info(`${this.name} 已签到`);
        return;
      }
      const content = await fetchText(this.href, {
        decode: 'gbk',
      });

      if (content.includes('快速登录')) {
        logger.error(`${this.name} 需要登录`);
        return;
      }
      const $doc = getDocObj(content);
      const $signBtn = $doc.querySelector('#pper_a') as HTMLAnchorElement;
      const src =
        ($signBtn.querySelector('.qq_bind') as HTMLImageElement)?.src || '';
      if (src.endsWith('wb.png')) {
        setSignResult(this.name, true);
        return;
      }
      if ($signBtn) {
        const checkRes = await fetchText(genUrl(this.href, $signBtn.href), {
          decode: 'gbk',
        });
        setSignResult(this.name, true);
      } else {
        logger.error(`${this.name} 签到失败。没有签到按钮`);
      }
    },
  },
];
async function main() {
  for (let i = 0; i < siteDict.length; i++) {
    const obj = siteDict[i];
    if (['zodgame', 'kafan'].includes(obj.name)) {
      continue;
    }
    // for test
    // if (obj.name === 'kafan') {
    //   await obj.signFn();
    //   return;
    // }
    try {
      await obj.signFn();
    } catch (error) {
      console.error(error);
    }
  }
}
main();
