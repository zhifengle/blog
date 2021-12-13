import axios from 'axios';
import iconv from 'iconv-lite';

export const request = axios.create({
  headers: {
    'user-agent':
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36',
  },
});

request.interceptors.request.use((req) => {
  // todo
  return req;
});

request.interceptors.response.use((response) => {
  const ctype = response.headers['content-type'];
  // 返回文本并且是 arraybuffer
  if (
    response.config.responseType === 'arraybuffer' &&
    ctype.includes('text')
  ) {
    const m = ctype.match(/charset=(.+)/);
    if (m) {
      const code = m[1];
      // iconv-lite 库
      response.data = iconv.decode(response.data, code);
    }
  }
  return response;
});
