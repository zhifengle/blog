import { HttpProxyAgent, HttpsProxyAgent } from 'hpagent';

const proxy = 'http://127.0.0.1:10809';

export const httpsAgent = new HttpsProxyAgent({
  keepAlive: true,
  keepAliveMsecs: 1000,
  maxSockets: 256,
  maxFreeSockets: 256,
  proxy,
});

export const httpAgent = new HttpProxyAgent({
  keepAlive: true,
  keepAliveMsecs: 1000,
  maxSockets: 256,
  maxFreeSockets: 256,
  proxy,
});
