import { fetchInfo } from '../utils/fetchData';

var sign_data: Record<string, string> = {};
const HOST = '115.com';

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
export async function addOfflineTask(task: string | string[], cid?: string) {
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
  var code = res.errcode || res.error_code || '';
  if (code === 911) {
    throw new Error(`${HOST}: abnormal operation`);
  }
  if (code === 10008) {
    throw new Error(`${HOST}: task exist`);
  }
  if (!res?.state) {
    throw new Error(`${HOST}: add offline task failed`);
  }
  return res;
}

export async function getUploadinfo() {
  const res = await fetchInfo('https://proapi.115.com/app/uploadinfo', 'json', {
    host: '115.com',
  });
  if (!res || res.errno === 99) {
    throw new Error(`${HOST}: need login`);
  }
  return res;
}

export async function getSign(): Promise<{ sign: string; time: string }> {
  const res = await fetchInfo('http://115.com/?ct=offline&ac=space', 'json');
  if (!res || !res.sign) {
    throw new Error(`${HOST}: need login`);
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
  return await fetchInfo(url, 'json', {
    method: 'post',
    data: fd,
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded; charset=UTF-8',
      Accept: 'application/json, text/javascript, */*; q=0.01',
      'X-Requested-With': 'XMLHttpRequest',
    },
  });
}
