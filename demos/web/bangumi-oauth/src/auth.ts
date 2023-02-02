import axios from 'axios';
import qs from 'qs';
import koa from 'koa';

type AuthInfo = {
  access_token: string;
  user_id: string;
  refresh_token: string;
  expires_in: number;
};

type AppCred = {
  client_id: string;
  client_secret: string;
};

type OAuthOpts = {
  code?: string;
  refresh_token?: string;
};

const appCred: AppCred = {
  // @TODO
  client_id: '',
  // @TODO
  client_secret: '',
};
const port = 3000;
const redirect_uri = `http://localhost:${port}/`;
const app = new koa();
// response
app.use(async (ctx) => {
  console.log(ctx.url);
  const code = ctx?.query?.code ?? '';
  if (code) {
    const info = await getAuthInfo({
      code: code as string,
    });
    console.log('info: ', info);
    const { data: user } = await instance.get(
      `https://api.bgm.tv/user/${info.user_id}`,
      {
        headers: {
          Authorization: `Bearer ${info.access_token}`,
        },
      }
    );
    console.log('user: ', user);
    ctx.body = info;
  } else {
    ctx.body = 'Hello Koa';
  }
});

app.listen(port);

const instance = axios.create({
  headers: {
    'User-Agent':
      'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36',
  },
});

instance.interceptors.response.use(
  function (response) {
    return response;
  },
  function (error) {
    return Promise.reject(error);
  }
);

function authorize() {
  const client_id = appCred.client_id;
  const authorizeUrl = `https://bgm.tv/oauth/authorize?client_id=${client_id}&response_type=code&redirect_uri=${redirect_uri}`;
  console.log('authorize url: ', authorizeUrl);
}

async function getAuthInfo(opts: OAuthOpts): Promise<AuthInfo> {
  const res = await instance.post(
    'https://bgm.tv/oauth/access_token',
    qs.stringify({
      grant_type: 'authorization_code',
      redirect_uri,
      // code: opts.code,
      ...opts,
      ...appCred,
    })
  );
  return res.data;
}
