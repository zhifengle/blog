import { AxiosRequestConfig } from 'axios';

export type SiteConfigReq = Record<string, AxiosRequestConfig>;

// https://www.typescriptlang.org/docs/handbook/utility-types.html#picktype-keys
// 使用 Pick 可以精简类型
// export type SiteConfigReq = {
//   [key: string]: Pick<
//     AxiosRequestConfig,
//     'headers' | 'httpsAgent' | 'httpAgent'
//   >;
// };
