// https://imququ.com/post/web-proxy.html
// https://github.com/qgy18/proxy-demo
const http = require('http');
const net = require('net');

const server = http.createServer();
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
  console.log('request: ', req);

  // ?? 页面重定向
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
