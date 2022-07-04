#!/usr/bin/env node

import { exec, execSync } from 'child_process';
import util from 'util';
import { Command } from 'commander';
import { opendir } from 'fs/promises';
import path from 'path';
import fs from 'fs';
import { encodeBase64 } from './utils/crypto';

const execPromise = util.promisify(exec);
const archiveNameMap = new Map<string, string>();
const homedir = require('os').homedir();
const WINRAR_PATH = String.raw`D:\Program Files\WinRAR\WinRAR.exe`;

type CompressConfig = {
  outputPath: string;
  targetPath: string;
  password: string;
  volume?: string;
};

function genWinrarCmd(config: CompressConfig) {
  const { outputPath, targetPath, password, volume } = config;
  let vol = '';
  if (volume) {
    vol = `-v${volume}`;
  }
  // 加密文件名称 -hp
  // -ep1 只保留文件夹名字
  // -v3.8g 分卷 3.8g
  return `"${WINRAR_PATH}" a ${vol} -r -rr3p -hp${password} -ep1 "${outputPath}" "${targetPath}"`;
}
function gen7zCmd(config: CompressConfig) {
  const { outputPath, targetPath, password, volume } = config;
  // 3892m; 7z 不支持小数点分卷
  let vol = '';
  if (volume) {
    vol = `-v${volume}`;
  }
  return `7z a "${outputPath}.7z" "${targetPath}" ${vol} -mhe -p${password}`;
}

function getOutputName(name: string): string {
  return encodeBase64(name).replace(/\//g, '_').replace(/\+/g, '-');
}

function outputNameMap() {
  const file = path.join(homedir, 'archive-name-map.txt');
  // let content = '';
  let content = fs.readFileSync(file, 'utf-8');
  for (const [key, val] of archiveNameMap.entries()) {
    content += `${key}=${val}\n`;
  }
  fs.writeFileSync(file, content, 'utf-8');
  execSync(`C:\\Windows\\System32\\notepad.exe ${file}`);
}

function isTargetDirectory(name: string, excludeDirs = ['done']): boolean {
  return !excludeDirs.some((dir) => name.includes(dir));
}

const program = new Command('archive');
program.version('0.0.1').description('archive folder');
program
  .requiredOption('-p, --password <type>', 'password')
  .option('-v, --volume <type>', 'volume size')
  .option('-t, --type <type>', 'compress type, rar or 7z', 'rar')
  .option('-e, --encryption', 'encrypt name')
  .option('-x, --exclude-dirs [dirs...]', 'exclude dirs')
  .argument('<target>', 'target folder')
  .argument('[output]', 'output folder')
  .action(async (target, output, options) => {
    try {
      let type = options.type;
      if (type === 'rar') {
        if (!fs.existsSync(WINRAR_PATH)) {
          throw new Error(`${WINRAR_PATH} 不存在`);
        }
      } else {
        type = '7z';
      }
      for await (const dirent of await opendir(target)) {
        if (
          isTargetDirectory(dirent.name, options.targetDirs) &&
          dirent.isDirectory()
        ) {
          let name = dirent.name;
          if (options.encryption) {
            name = getOutputName(name);
          }
          if (!output) {
            output = target;
          }
          const targetPath = path.join(target, dirent.name);
          const outputPath = path.join(output, name);
          const config: CompressConfig = {
            outputPath,
            targetPath,
            password: options.password,
            volume: options.volume,
          };
          let s = '';
          if (type === 'rar') {
            s = genWinrarCmd(config);
          } else {
            s = gen7zCmd(config);
          }
          console.log('Start: ', targetPath);
          await execPromise(s);
          archiveNameMap.set(name, dirent.name);
          console.log('Output:', outputPath + '.' + type);
        }
      }
      if (options.encryption) {
        outputNameMap();
      }
    } catch (err) {
      console.error(err);
    }
  });

program.parse();
