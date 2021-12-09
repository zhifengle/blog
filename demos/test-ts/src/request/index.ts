// 使用 fetch 发送 jsonrpc
async function sendJsonRpc() {
  let postData = {};
  const res = await fetch('http://localhost:6800/jsonrpc', {
    method: 'POST',
    body: JSON.stringify(postData),
  });
  const obj = await res.json();
}
