import os from 'os';
import type { AxiosRequestConfig } from 'axios';
import { SiteConfigReq } from 'site';
import { httpAgent, httpsAgent } from './agent';
import { request } from './request';
import { getUserSiteConfig } from './site-config';

export type IFetchOpts = {
  body?: any;
  // EUC-JP 部分网页编码
  decode?: string;
} & AxiosRequestConfig;
type IAjaxType = 'text' | 'json' | 'blob' | 'arraybuffer';

let USER_SITE_CONFIG: SiteConfigReq = {};

export function initDefaultOption() {
  const homedir = os.homedir();
  setOption(getUserSiteConfig(homedir));
}

export function setOption(config: SiteConfigReq) {
  USER_SITE_CONFIG = config;
}

// @TODO 添加 cookie 签到才有登录信息
const req_site_configs: { [key: string]: AxiosRequestConfig } = {
  'v2ex.com': {
    httpsAgent: httpsAgent,
    headers: {
      Referer: 'https://v2ex.com/',
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
  const config = { ...req_site_configs, ...USER_SITE_CONFIG }[hostname] || {};
  // JSON 配置 HttpsAgent
  if (config.httpsAgent === 'httpsAgent') {
    config.httpsAgent = httpsAgent;
  }
  if (config.httpAgent && config.httpAgent === 'httpAgent') {
    config.httpAgent = httpAgent;
  }
  const res = await request(url, {
    timeout: TIMEOUT,
    method,
    responseType: type,
    ...gmXhrOpts,
    ...config,
    headers: {
      ...gmXhrOpts.headers,
      ...config?.headers,
    },
  });
  return res.data;
}

function getSiteConfg(url: string): AxiosRequestConfig {
  const hostname = new URL(url)?.hostname;
  const config = { ...req_site_configs, ...USER_SITE_CONFIG }[hostname] || {};
  // JSON 配置 HttpsAgent
  if (config.httpsAgent === 'httpsAgent') {
    config.httpsAgent = httpsAgent;
  }
  if (config.httpAgent && config.httpAgent === 'httpAgent') {
    config.httpAgent = httpAgent;
  }
  return config;
}

// 2022-07-12 使用 axios 的 api; fetchInfo 的代码就不动了
export async function fetchInstance(
  url: string,
  config: AxiosRequestConfig = {}
) {
  const siteConfig = getSiteConfg(url);
  const res = await request(url, {
    ...siteConfig,
    ...config,
    headers: {
      ...siteConfig.headers,
      ...config.headers,
    },
  });
  return res.data;
}
