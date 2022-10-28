const path = require('path');
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
  // newName = newName.replace('_z_lib_org', '')
  // newName = newName.replace('(z-lib.org)', '')
  return newName;
}

async function run(dir, recursive = false) {
  for await (const p of walk(dir, recursive)) {
    const newName = getNewName(p.name);
    if (newName && p && p.name && p.path) {
      await renameFile(p.path, p.name, newName);
    }
  }
}

run(targetPath);
