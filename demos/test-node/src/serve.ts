import path from 'path';
import http from 'http';
// import https from 'https';
import fs from 'fs';

type IServeConfig = {
  public: string;
  [key: string]: any;
};
const handler = require('serve-handler');

const registerShutdown = (fn: (...arg: any) => any) => {
  let run = false;

  const wrapper = () => {
    if (!run) {
      run = true;
      fn();
    }
  };

  process.on('SIGINT', wrapper);
  process.on('SIGTERM', wrapper);
  process.on('exit', wrapper);
};

async function run() {
  const cwd = process.cwd();
  // 参数配置？
  const entry = cwd;
  startServe({
    public: entry,
  });
}

function startServe(config: IServeConfig) {
  const server = http.createServer(async (req, res) => {
    handler(req, res, {
      public: config.public,
    });
  });
  server.listen('3000', async () => {
    registerShutdown(() => server.close());
  });
  server.on('error', (err) => {
    console.error(`Failed to serve: ${err.stack}`);
    process.exit(1);
  });
}

run();
