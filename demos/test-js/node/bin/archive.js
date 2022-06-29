#!/usr/bin/env node

const util = require('util');
const exec = util.promisify(require('child_process').exec);
const { opendir } = require('fs/promises');
const path = require('path');
const fs = require('fs');

// const targetDir = String.raw`D:\music`;
const excludes = ['done'];

function genWinrarCmd(outputPath, targetPath, pw) {
  const exePath = String.raw`D:\Program Files\WinRAR\WinRAR.exe`;
  // 加密文件名称 -hp
  // -ep1 只保留文件夹名字
  return `"${exePath}" a -v3.8g -r -rr3p -hp${pw} -ep1 "${outputPath}" "${targetPath}"`;
}
function gen7zCmd(outputPath, targetPath, pw) {
  return `7z a "${outputPath}.7z" -mhe -p${pw} "${targetPath}"`;
}

async function run(args) {
  const { targetDir, outputDir, pw } = args;
  try {
    const dirs = await opendir(targetDir);
    for await (const dirent of dirs) {
      if (
        !new RegExp(excludes.join('|')).test(dirent.name) &&
        dirent.isDirectory()
      ) {
        const p = path.join(targetDir, dirent.name);
        const outputPath = path.join(outputDir, dirent.name);
        // 命令存在空格需要使用引号; 可以换成 outputPath; -pXX 密码
        // const s = `7z a "${outputPath}.7z" -mhe -p${pw} "${p}"`;
        const s = genWinrarCmd(outputPath, p, pw);
        console.log('Start: ', p);
        // stdout 乱码参考下面
        const { stderr } = await exec(s);
        if (stderr) {
          console.log(stderr);
        } else {
          console.log('Ok:', outputPath);
        }
      }
    }
  } catch (err) {
    console.error(err);
  }
}
function parseArgs(argv) {
  if (argv[0] !== '-p' || !argv[1]) {
    console.log('需要指定 password');
    return;
  }

  if (!argv[2]) {
    console.log('需要指定路径');
    return;
  }
  if (!fs.existsSync(argv[2])) {
    console.log('路径不存在或者不正确');
    return;
  }
  let outputDir = argv[2];
  if (argv[3] && fs.existsSync(argv[3])) {
    outputDir = argv[3];
  }
  return {
    pw: argv[1],
    targetDir: argv[2],
    outputDir,
  };
}

// 压缩目标文件夹里面的文件夹
// node  archive.js  -p xxx  c:\
const args = parseArgs(process.argv.slice(2));
if (!args) {
  process.exit(1);
}
// for test
// var args = {
//   pw: 'test',
//   targetDir: String.raw`C:\tmp\test`,
//   outputDir: String.raw`C:\tmp\test`,
// };
run(args);

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
