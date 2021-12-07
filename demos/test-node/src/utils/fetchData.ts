import { AxiosRequestConfig } from 'axios';
import { httpsAgent } from './agent';
import { request } from './request';

export type IFetchOpts = {
  method?: string;
  body?: any;
  // EUC-JP 部分网页编码
  decode?: string;
  [key: string]: any;
};
type IAjaxType = 'text' | 'json' | 'blob' | 'arraybuffer';

// @TODO 添加 cookie 签到才有登录信息
const req_site_configs: { [key: string]: AxiosRequestConfig } = {
  'm.lrts.me': {
    headers: {
      'user-agent':
        'Mozilla/5.0 (Linux; U; Android 9; tr-tr; Redmi Note 7 Build/PKQ1.180904.001) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/89.0.4389.116 Mobile Safari/537.36 XiaoMi/MiuiBrowser/12.15.2-gn',
      referrer: 'https://m.lrts.me/user',
    },
  },
  'www.south-plus.net': {
    httpsAgent: httpsAgent,
    headers: {
      referrer: 'https://www.south-plus.net/',
    },
  },
  'bbs4.2djgame.net': {
    httpsAgent: httpsAgent,
    headers: {
      referrer: 'https://bbs4.2djgame.net/home/forum.php',
    },
  },
  'zodgame.xyz': {
    httpsAgent: httpsAgent,
    headers: {
      referrer: 'https://zodgame.xyz/',
    },
  },
  'www.52pojie.cn': {
    headers: {
      Host: 'www.52pojie.cn',
    },
  },
  'v2ex.com': {
    httpsAgent: httpsAgent,
    headers: {
      referrer: 'https://v2ex.com/',
    },
  },
};
export async function fetchText(url: string, opts: any = {}) {
  return fetchInfo(url, 'text', opts);
}

export async function fetchInfo(
  url: string,
  type: IAjaxType,
  opts: IFetchOpts = {},
  TIMEOUT = 10 * 1000
): Promise<any> {
  const method: any = opts?.method?.toUpperCase() || 'GET';
  const gmXhrOpts = { ...opts };
  if (method === 'POST' && gmXhrOpts.body) {
    gmXhrOpts.data = gmXhrOpts.body;
  }
  if (opts.decode) {
    type = 'arraybuffer';
  }
  const hostname = new URL(url)?.hostname;
  const res = await request(url, {
    timeout: TIMEOUT,
    method,
    responseType: type,
    ...gmXhrOpts,
    ...req_site_configs[hostname],
    headers: {
      ...gmXhrOpts.headers,
      ...req_site_configs[hostname]?.headers,
    },
  });
  return res.data;
}
