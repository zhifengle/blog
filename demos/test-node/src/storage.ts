import fs from 'fs';
import util from 'util';

export default class Storage {
  filename: string;
  constructor(filename: string) {
    this.filename = filename;
  }
  writeFile(name: string, obj: any = {}) {
    fs.writeFileSync(
      name,
      JSON.stringify(obj),
      // `${util.inspect(obj, { maxArrayLength: null })}`,
      'utf-8'
    );
  }
  getConfig() {
    let config: any = {};
    try {
      config = JSON.parse(fs.readFileSync(this.filename, 'utf-8'));
    } catch (error) {
      config = {};
    }
    return config;
  }
  getValue(key: string) {
    const config = this.getConfig();
    return config[key];
  }
  setValue(key: string, val: any) {
    const config = this.getConfig();
    config[key] = val;
    this.writeFile(this.filename, config);
  }
}
