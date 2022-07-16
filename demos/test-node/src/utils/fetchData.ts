import type { AxiosRequestConfig } from 'axios';
import { SiteConfigReq } from '../types/site';
import { httpAgent, httpsAgent } from './agent';
import { request } from './request';
import { getUserSiteConfig } from './site-config';
import { execSync } from 'child_process';
import { FirefoxCookies } from './cookies';

export type IFetchOpts = {
  body?: any;
  // EUC-JP 部分网页编码
  decode?: string;
} & AxiosRequestConfig;
type IAjaxType = 'text' | 'json' | 'blob' | 'arraybuffer';

let USER_SITE_CONFIG: SiteConfigReq = {};

export function initDefaultOption(configName?: string) {
  const option = getUserSiteConfig(configName);
  for (const [key, config] of Object.entries(option)) {
    let cookie = config?.headers?.cookie as string;
    if (!cookie) {
      try {
        cookie = execSync(
          `C:\\apps\\bin\\get-site-cookies.cmd ${key}`
        ).toString();
      } catch (error) {}
    }
    if (cookie) {
      config.headers = {
        ...config.headers,
        cookie,
      };
    }
  }
  setOption(option);
}

export async function initDefaultOptionFirefox(cookiePath: string) {
  const fx = new FirefoxCookies(cookiePath);
  const option = getUserSiteConfig();
  for (const [key, config] of Object.entries(option)) {
    let cookie = config?.headers?.cookie as string;
    if (!cookie) {
      try {
        cookie = await fx.getSiteCookie(key);
      } catch (error) {}
    }
    if (cookie) {
      config.headers = {
        ...config.headers,
        cookie,
      };
    }
  }
  setOption(option);
}

export function addSiteOption(host: string, config: AxiosRequestConfig) {
  USER_SITE_CONFIG[host] = config;
}

export function setOption(config: SiteConfigReq) {
  USER_SITE_CONFIG = config;
}

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
  const config = getSiteConfg(url);
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
  const config = USER_SITE_CONFIG[hostname] || {};
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
