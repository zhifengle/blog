// 使用 fetch 发送 jsonrpc
async function sendJsonRpc() {
  let postData = {};
  const res = await fetch('http://localhost:6800/jsonrpc', {
    method: 'POST',
    body: JSON.stringify(postData),
  });
  const obj = await res.json();
}

// 分页参数范围
function genPageParams(start: number, end: number, pageSize: number) {
  const startIdx = Math.floor(start / pageSize) + 1;
  const endIdx = Math.floor(end / pageSize);
}
