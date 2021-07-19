import { httpsAgent } from './utils/agent';
import { request } from './utils/request';

async function main() {
  const content = (
    await request('https://bgm.tv/', {
      httpsAgent: httpsAgent,
      headers: {
        cookie:
          // @TODO 填入自己的 cookie
          '',
      },
    })
  ).data;
  // @TODO 填入自己的用户名
  console.log(content.match('拿月亮自刎'));
}

main();
