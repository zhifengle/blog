import Parser from 'rss-parser';
import { filterItems, Item, MAGNET_PREFIX, Pattern } from './common';

export function convertToMagnet(link: string): string {
  return MAGNET_PREFIX + link;
}

export async function getItems(
  rss: string,
  pattern?: Pattern
): Promise<Item[]> {
  const parser = new Parser({
    customFields: {
      item: [['nyaa:infoHash', 'infoHash']],
    },
  });
  const feed = await parser.parseString(rss);
  return filterItems(feed.items, pattern).map((item) => {
    const obj: Item = { ...item };
    obj.magnet = convertToMagnet(item.infoHash);
    return obj;
  });
}
