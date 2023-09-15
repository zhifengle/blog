const path = require('path');
const fs = require('fs');
const { execSync } = require('child_process');
const { rename, opendir } = require('fs/promises');

const targetPath = String.raw`C:\test`;

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

function getNewName(name) {
  var newName = name;
  newName = newName.replace('_z_lib_org', '')
  newName = newName.replace('(z-lib.org)', '')
  newName = newName.replace('(Z-Library)', '')
  return newName.trim();
}

async function run(dir, recursive = false) {
  for await (const p of walk(dir, recursive)) {
    const newName = getNewName(p.name);
    if (newName && p && p.name && p.path) {
      await renameFile(p.path, p.name, newName);
    }
  }
}

function execGitCmd() {
  if (fs.existsSync('.git')) {
    execSync(`git config user.name "zhifengle"`);
    changeGitRemote();
  }
}

function changeGitRemote() {
  try {
    const remote = execSync(`git remote get-url origin`).toString();
    if (remote.includes('22earth')) {
      execSync(
        `git remote set-url origin ${remote.replace('22earth', 'zhifengle')}`
      );
    }
  } catch (error) {}
}

async function execute(dir, recursive = false) {
  for await (const p of walk(dir, recursive)) {
    const curFolder = path.join(dir, p.name);
    if (fs.lstatSync(curFolder).isDirectory()) {
      process.chdir(curFolder);
      execGitCmd()
    }
  }
}

run(targetPath);
// execute(targetPath);
