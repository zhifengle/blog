import { Sequelize } from 'sequelize-typescript';
import { Enclosure } from './models/Enclosure.model';
import { Item } from './models/Item.model';
import { SiteStatus } from './models/SiteStatus.model';

export class RssService {
  sequelize: Sequelize;
  constructor() {
    this.sequelize = new Sequelize({
      dialect: 'sqlite',
      storage: 'db.sqlite',
      models: [Item, Enclosure],
      // 禁用打印 log
      // logging: false,
      // 这种方式失败
      // models: [__dirname + '/models'],
    });
  }
  saveItems(items: any[]) {
    return Item.bulkCreate(items);
  }
  getItemByMagent(magnet: string): Promise<Item> {
    return Item.findOne({
      attributes: ['title', 'magnet', 'done'],
      where: {
        magnet,
      },
    });
  }
  async isItemExist(magnet: string): Promise<boolean> {
    const num = await Item.count({
      where: {
        magnet,
      },
    });
    return num !== 0;
  }
}

const STATUS_ARR = ['needLogin', 'abnormalOp'];

// 没有 table 时需要调用 sync
export class SiteStatusService {
  sequelize: Sequelize;
  constructor() {
    this.sequelize = new Sequelize({
      dialect: 'sqlite',
      storage: 'db.sqlite',
      models: [SiteStatus],
    });
  }
  // type SiteStatusProp = 'needLogin' | 'abnormalOp';
  async updateStatus(name: string, data: Record<string, any>) {
    const [obj] = await SiteStatus.findOrBuild({
      where: {
        name,
      },
    });
    await obj.update(data);
    await obj.save();
  }
  async resetStatus(name: string) {
    await this.updateStatus(name, {
      needLogin: false,
      abnormalOp: false,
    });
  }
  async isReady(name: string): Promise<boolean> {
    const [site, init] = await SiteStatus.findOrBuild({
      where: {
        name,
      },
    });
    if (init) {
      await site.save();
      return true;
    }
    for (const key of STATUS_ARR) {
      if (site[key as keyof SiteStatus] === true) {
        return false;
      }
    }
    return true;
  }
}
