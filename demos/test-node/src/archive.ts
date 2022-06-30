#!/usr/bin/env node

import { exec } from 'child_process';
import { Command } from 'commander';
import { opendir } from 'fs/promises';
import path from 'path';
import fs from 'fs';
import { getUserSiteConfig } from './utils/site-config';
import { encodeBase64 } from './utils/crypto';

const excludes = ['done'];
const archiveNameMap = new Map<string, string>();
const homedir = require('os').homedir();

let USER_CONFIG: any = {};

type CompressConfig = {
  outputPath: string;
  targetPath: string;
  password: string;
};

function genWinrarCmd(config: CompressConfig) {
  const exePath = String.raw`D:\Program Files\WinRAR\WinRAR.exe`;
  // 加密文件名称 -hp
  // -ep1 只保留文件夹名字
  return `"${exePath}" a -v3.8g -r -rr3p -hp${config.password} -ep1 "${config.outputPath}" "${config.targetPath}"`;
}
function gen7zCmd(config: CompressConfig) {
  const { outputPath, targetPath, password } = config;
  return `7z a "${outputPath}.7z" -mhe -p${password} "${targetPath}"`;
}

function getOutputName(name: string): string {
  // const ARCHIVE_NAME_KEY = 'archive-name-key';
  // return encryptAesEcb(name, USER_CONFIG[ARCHIVE_NAME_KEY]);
  return encodeBase64(name).replace(/\//g, '_').replace(/+/g, '-');
}

function outputNameMap() {
  const file = path.join(homedir, 'archive-name-map.txt');
  // let content = '';
  let content = fs.readFileSync(file, 'utf-8');
  for (const [key, val] of archiveNameMap.entries()) {
    content += `${key}=${val}\n`;
  }
  fs.writeFileSync(file, content, 'utf-8');
}

const program = new Command('archive');
program.version('0.0.1').description('archive folder');
program
  .requiredOption('-p, --password <type>', 'password')
  .option('-t, --type <type>', 'compress type', 'rar')
  .option('-e, --encryption', 'encrypt name')
  .argument('<target>', 'target folder')
  .argument('[output]', 'output folder')
  .action(async (target, output, options) => {
    try {
      USER_CONFIG = getUserSiteConfig(homedir, 'node-user-config.json');
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
          const targetPath = path.join(target, name);
          const outputPath = path.join(output, dirent.name);
          const config: CompressConfig = {
            outputPath,
            targetPath,
            password: options.password,
          };
          let s = '';
          if (options.type === 'rar') {
            s = genWinrarCmd(config);
          } else {
            s = gen7zCmd(config);
          }
          console.log('Start: ', targetPath + '.' + options.type);
          const { stderr } = await exec(s);
          // let stderr = null;
          if (stderr) {
            console.log(stderr);
          } else {
            archiveNameMap.set(name, dirent.name);
            console.log('Ok:', outputPath + '.' + options.type);
          }
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
