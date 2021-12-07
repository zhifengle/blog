import fs from 'fs';
import path from 'path';
import { SiteConfigReq } from '../types/site';

export function getUserSiteConfig(
  basePath: string,
  name = 'node-site-config.json'
): SiteConfigReq {
  let config: any = {};
  const filename = path.join(basePath, name);
  if (!fs.existsSync(filename)) {
    return config;
  }
  try {
    config = JSON.parse(fs.readFileSync(filename, 'utf-8'));
  } catch (error) {
    console.log(error);
    config = {};
  }
  return config;
}
