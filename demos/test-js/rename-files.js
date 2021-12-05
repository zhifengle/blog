const path = require('path');
const { rename, opendir } = require('fs/promises');

const targetPath = String.raw`D:\sakura`;
// 曲目可以在 amazon 等上面找
const nameArr = `
01. この櫻ノ詩の下
02. 透明な白い日常
03. 呼吸の様に筆は踊る
04. 心模型
05. 花弁となり桜は大いに歌う
06. 因果的交流の世界
07. 舞い上がる因果交流
08. 夢の歩みを見上げて
09. 優雅な音階
10. ♪模型
11. 月の眼球譚
12. 心象の中の光
13. 在りし日のために - inst ver. -
14. 空を舞う月 空を舞う翼
15. ジムノペディ
16. 軽やかに! 軽やかに!
17. 夜空は奏でるだろう
18. 真っ赤な果実
19. 天球の奇蹟
20. シューマン交響曲第一番的日常
21. ありがとう在りし日
22. 風の筆射す春日花抄
`
  .trim()
  .split('\n');

async function renameFile(dirPath, name, newName) {
  const oldFullName = path.join(dirPath, name);
  const newFullName = path.join(dirPath, newName);
  await rename(oldFullName, newFullName);
}

async function* walk(dir, recursive) {
  for await (const d of await opendir(dir)) {
    if (!recursive) {
      yield {
        name: d.name,
        path: dir,
      };
    } else if (d.isDirectory()) {
      const entry = path.join(dir, d.name);
      yield* walk(entry, true);
    } else if (d.isFile())
      yield {
        name: d.name,
        path: dir,
      };
  }
}

async function run(dir, prefix = '') {
  let counter = 0;
  for await (const p of walk(dir, false)) {
    let newName = nameArr[counter];
    if (newName && p && p.name && p.path) {
      if (prefix) {
        newName = `${newName}.${prefix}`;
      }
      await renameFile(p.path, p.name, newName);
    }
    counter++;
  }
}

run(targetPath, 'mp3');
