#!/usr/bin/env node
const fs = require('fs').promises;

// console.log('simple cli');
// console.log('hello ', process.argv[2]);
// const query = process.argv[2];
// const filename = process.argv[3];

function main() {
  try {
    const config = new Config(process.argv);
    run(config)
      .then((v) => {
        for (const i in v) {
          console.log(v[i]);
        }
      })
      .catch((e) => {
        console.error(e);
        process.exit(1);
      });
  } catch (e) {
    console.error(e);
    process.exit(1);
  }
}
class Config {
  constructor(argv) {
    if (argv && argv.length < 4) {
      throw new Error('not enough arguments');
    }
    this.query = argv[2];
    this.filename = argv[3];
  }
}
async function run(config) {
  let results = [];
  // const contents = await fs.readFile(config.filename, 'utf-8');
  const contents = await fs.readFile(config.filename, 'utf-8');
  for (const line of contents.split('\n')) {
    if (line && line.includes(config.query)) {
      results.push(line);
    }
  }
  return results;
}

main();
