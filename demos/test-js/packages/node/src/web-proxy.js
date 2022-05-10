// https://imququ.com/post/web-proxy.html
// https://github.com/qgy18/proxy-demo
const http = require('http');
const https = require('https');
const net = require('net');
const path = require('path');
const fs = require('fs');

const options = {
  key: fs.readFileSync(path.join(__dirname, '../private.pem')),
  cert: fs.readFileSync(path.join(__dirname, '../public.crt')),
};
const server = http.createServer();
// const server = https.createServer(options);
server.listen('8899', '0.0.0.0');
server.on('request', (req, res) => {
  const url = new URL(req.url);
  const opt = {
    hostname: url.hostname,
    port: url.port || 80,
    path: url.path,
    method: req.method,
    headers: req.headers,
  };

  const serverReq = http
    .request(opt, (serverRes) => {
      res.writeHead(serverRes.statusCode, {
        // 中间加上响应
        'user-agent': 'node-proxy',
        ...serverRes.headers,
      });
      serverRes.pipe(res);
    })
    .on('error', (e) => {
      console.error(e);
      res.end();
    });

  req.pipe(serverReq);
});

function handleConnect(clientReq, clientSock) {
  const url = new URL(clientReq.url);
  const pipeSock = net
    .connect(url.port, url.hostname, () => {
      console.log('pipe socket request: ', pipeSock);
      clientSock.write('HTTP/1.1 200 Connection Established\r\n\r\n');
      pipeSock.pipe(clientSock);
    })
    .on('error', (e) => {
      console.log(e);
      clientSock.end();
    });
  clientSock.pipe(pipeSock);
}

server.on('connect', handleConnect);

// 浏览器访问 http://localhost:3000
// 注意 switch omega 设置
// 127.0.0.1
// [::1]
// localhost
const server2 = http.createServer();
server2.on('request', (req, res) => {
  res.end('okay');
});
server2.listen('3000', '0.0.0.0');
