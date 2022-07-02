#!/usr/bin/env node

import { exec, execSync } from 'child_process';
import util from 'util';
import { Command } from 'commander';
import { opendir } from 'fs/promises';
import path from 'path';
import fs from 'fs';
import { getUserSiteConfig } from './utils/site-config';
import { encodeBase64 } from './utils/crypto';

const execPromise = util.promisify(exec);
const excludes = ['done'];
const archiveNameMap = new Map<string, string>();
const homedir = require('os').homedir();
const WINRAR_PATH = String.raw`D:\Program Files\WinRAR\WinRAR.exe`;

let USER_CONFIG: any = {};

type CompressConfig = {
  outputPath: string;
  targetPath: string;
  password: string;
};

function genWinrarCmd(config: CompressConfig) {
  // 加密文件名称 -hp
  // -ep1 只保留文件夹名字
  return `"${WINRAR_PATH}" a -v3.8g -r -rr3p -hp${config.password} -ep1 "${config.outputPath}" "${config.targetPath}"`;
}
function gen7zCmd(config: CompressConfig) {
  const { outputPath, targetPath, password } = config;
  return `7z a "${outputPath}.7z" "${targetPath}" -v3.8g -mhe -p${password}`;
}

function getOutputName(name: string): string {
  // const ARCHIVE_NAME_KEY = 'archive-name-key';
  // return encryptAesEcb(name, USER_CONFIG[ARCHIVE_NAME_KEY]);
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

const program = new Command('archive');
program.version('0.0.1').description('archive folder');
program
  .requiredOption('-p, --password <type>', 'password')
  .option('-t, --type <type>', 'compress type, rar or 7z', 'rar')
  .option('-e, --encryption', 'encrypt name')
  .argument('<target>', 'target folder')
  .argument('[output]', 'output folder')
  .action(async (target, output, options) => {
    try {
      USER_CONFIG = getUserSiteConfig(homedir, 'node-user-config.json');
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
          !new RegExp(excludes.join('|')).test(dirent.name) &&
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
