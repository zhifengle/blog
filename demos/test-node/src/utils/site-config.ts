import os from 'os';
import fs from 'fs';
import path from 'path';
import { SiteConfigReq } from '../types/site';

export function getUserSiteConfig(
  name: string = 'node-site-config.json'
): SiteConfigReq {
  let filename = path.join(process.cwd(), name);
  if (!fs.existsSync(filename)) {
    filename = path.join(os.homedir(), name);
  }
  let config: any = {};
  if (!fs.existsSync(filename)) {
    return config;
  }
  try {
    config = JSON.parse(fs.readFileSync(filename, 'utf-8'));
  } catch (error) {
    console.error(error);
    config = {};
  }
  return config;
}
