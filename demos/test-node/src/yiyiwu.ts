import { execSync } from 'child_process';
import { fetchInstance, addSiteOption } from './utils/fetchData';

var sign_data: Record<string, string> = {};

// 使用客户端的 UA
// https://github.com/orzogc/fake115uploader/blob/master/main.go

/*
// 离线的响应数据; 批量任务是 res.result,  result 就是下面的集合
{
  state: true,
  errno: 0,
  errtype: "",
  errcode: 0,
  info_hash: "xx",
  name: "nxx",
  url: "magnet:?xt=urn:btih:afaf",
}
*/
async function addOfflineTask(task: string | string[], cid?: string) {
  let data: Record<string, string> = {};
  let reqURL = 'http://115.com/web/lixian/?ct=lixian&ac=add_task_url';
  if (typeof task === 'string') {
    data = { url: task };
  } else {
    // result=[]
    reqURL = 'http://115.com/web/lixian/?ct=lixian&ac=add_task_urls';
    task.forEach((url, index) => {
      data[`url[${index}]`] = url;
    });
  }
  if (cid) {
    data['wp_path_id'] = cid;
  }
  const res = await postData(reqURL, data);
  if (res?.errcode === 10008) {
    throw new Error('task exist');
  }
  if (!res?.state) {
    throw new Error('add offline task failed');
  }
  return res;
}

async function getSign(): Promise<{ sign: string; time: string }> {
  const res = await fetchInstance('http://115.com/?ct=offline&ac=space', {
    responseType: 'json',
  });
  if (!res || !res.sign) {
    throw new Error('need login');
  }
  return res;
}
// wp_path_id: cid,
// savepath: title
async function postData(url: string, data: Record<string, string> = {}) {
  sign_data = await getSign();
  const fd = new URLSearchParams({
    // 测试一下不用传 uid
    // uid: '99',
    ...sign_data,
    ...data,
  });
  return await fetchInstance(url, {
    method: 'post',
    responseType: 'json',
    data: fd,
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded; charset=UTF-8',
      Accept: 'application/json, text/javascript, */*; q=0.01',
      'X-Requested-With': 'XMLHttpRequest',
    },
  });
}
async function init() {
  // ref: test-js/node/batch-demo.cmd
  const cookie = execSync(
    `C:\\apps\\bin\\get-site-cookies.cmd 115.com`
  ).toString();
  addSiteOption('115.com', {
    headers: {
      'User-Agent':
        'Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.94 Safari/537.36 115Browser/23.9.2',
      Origin: 'https://115.com',
      cookie,
    },
  });
  sign_data = await getSign();
  // console.log(signData);
}
init();
