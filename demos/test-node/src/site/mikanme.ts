import Parser from 'rss-parser';
import { JSDOM } from 'jsdom';
import { filterItems, Item, MAGNET_PREFIX, Pattern } from './common';

export function parseXML(str: string): Document {
  const dom = new JSDOM(str, { contentType: 'text/xml' });
  return dom.window.document;
}

export function getMagnetXML(
  xmlStr: string,
  isMatch: (str: string) => boolean
): string[] {
  const doc = parseXML(xmlStr);
  return [...doc.querySelectorAll('item')]
    .filter((item) => {
      return isMatch(item.querySelector('title').innerText);
    })
    .map(
      (item) =>
        MAGNET_PREFIX +
        item.querySelector('link').innerText.split('Episode/')[1]
    );
}

export function convertToMagnet(link: string): string {
  return MAGNET_PREFIX + link.split('Episode/')[1];
}

export async function getMagnet(
  rss: string,
  pattern?: Pattern
): Promise<string[]> {
  const items = await getItems(rss, pattern);
  return items.map((item) => convertToMagnet(item.link));
}

export async function getItems(
  rss: string,
  pattern?: Pattern
): Promise<Item[]> {
  const parser = new Parser();
  const feed = await parser.parseString(rss);
  return filterItems(feed.items, pattern).map((item) => {
    item.magnet = convertToMagnet(item.link);
    return item;
  });
}
