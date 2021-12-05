import { Command } from 'commander';
import path from 'path';

// https://github.com/sonnyp/aria2.js
const homedir = require('os').homedir();
const downloadPath = path.join(homedir, 'Downloads');
const program = new Command('tingshu');
program.version('0.0.1').description('A downloader for lrts');
program
  .requiredOption('-u, --url <type>', 'lrts book url')
  .option('-p, --path <type>', 'the path for saving audio', downloadPath)
  // .argument('<start>', 'start chapter')
  // .argument('[end]', 'last chapter', 0)
  .action(async (options) => {
    try {
      console.log(options);
      // console.log(start, end);
      // await run(reStr, dir, options);
    } catch (error) {
      console.error(error.message);
    }
  });
program.parse();
