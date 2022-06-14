const util = require('util');
const exec = util.promisify(require('child_process').exec);
const { opendir } = require('fs/promises');
const path = require('path');

const targetDir = String.raw`D:\music`;
const excludes = ['done'];
// const outputDir = String.raw`D:\d`;
const outputDir = './';

async function init() {
  try {
    const dir = await opendir(targetDir);
    for await (const dirent of dir) {
      if (
        !new RegExp(excludes.join('|')).test(dirent.name) &&
        dirent.isDirectory()
      ) {
        const p = path.join(targetDir, dirent.name);
        const outputPath = path.join(outputDir, dirent.name);
        // 命令存在空格需要使用引号; 可以换成 outputPath; -pXX 密码
        const s = `7z a "${p}.7z" -mhe -pXX "${p}"`;
        console.log('Start: ', p);
        // stdout 乱码参考下面
        const { stderr } = await exec(s);
        if (stderr) {
          console.log(stderr);
        } else {
          console.log('Ok:', p + '.7z');
        }
      }
    }
  } catch (err) {
    console.error(err);
  }
}
init();

// Windows 下乱码
/*
const encoding = 'cp936';
const binaryEncoding = 'binary';

function iconvDecode(str = '') {
  return iconv.decode(Buffer.from(str, binaryEncoding), encoding);
}

const { exec } = require('child_process');
exec('xxx', { encoding: 'binary' }, (err, stdout, stderr) => {
  const result = iconvDecode(stdout);
  xxx;
});
*/
