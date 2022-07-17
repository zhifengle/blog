import Parser, { Enclosure } from 'rss-parser';
import { filterItems, Item, Pattern } from './common';

// 示例链接
// https://share.dmhy.org/topics/rss/rss.xml?keyword=Kiss+baha&sort_id=0&team_id=801&order=date-desc

export function convertToMagnet(enclosure: Enclosure): string {
  return enclosure.url.replace(/&dn=.*$/, '');
}
export async function getItems(
  rss: string,
  pattern?: Pattern
): Promise<Item[]> {
  const parser = new Parser();
  const feed = await parser.parseString(rss);
  return filterItems(feed.items, pattern).map((item) => {
    item.magnet = convertToMagnet(item.enclosure);
    return item;
  });
}
