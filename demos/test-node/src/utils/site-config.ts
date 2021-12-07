import { AxiosRequestConfig } from 'axios';

export const USER_SITE_CONFIG: { [key: string]: AxiosRequestConfig } = {
  'm.lrts.me': {
    headers: {
      'user-agent':
        'Mozilla/5.0 (Linux; U; Android 9; tr-tr; Redmi Note 7 Build/PKQ1.180904.001) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/89.0.4389.116 Mobile Safari/537.36 XiaoMi/MiuiBrowser/12.15.2-gn',
      referrer: 'https://m.lrts.me/user',
      // Cookie: '',
    },
  },
};
