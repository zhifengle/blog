import { Sequelize } from 'sequelize-typescript';
import { Enclosure } from './models/Enclosure.model';
import { Item } from './models/Item.model';

export class RssService {
  sequelize: Sequelize;
  constructor() {
    this.sequelize = new Sequelize({
      database: 'rss2pan',
      dialect: 'sqlite',
      storage: 'db.sqlite',
      models: [Item, Enclosure],
      // 这种方式失败
      // models: [__dirname + '/models'],
    });
    this.sequelize.sync();
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
