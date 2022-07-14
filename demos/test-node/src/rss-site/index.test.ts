import { getMagnet } from './mikanme';
import { getItems } from './nyaa';
describe('mikanme', () => {
  test('get magnet', async () => {
    const list = await getMagnet(
      `<?xml version="1.0" encoding="utf-8"?><rss version="2.0"><channel><title>Mikan Project - 继母的拖油瓶是我的前女友</title><link>http://mikanani.me/RSS/Bangumi?bangumiId=2777&amp;subgroupid=562</link><description>Mikan Project - 继母的拖油瓶是我的前女友</description><item><guid isPermaLink="false">[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (Baha 1920x1080 AVC AAC MP4)</guid><link>https://mikanani.me/Home/Episode/8e753b6287da23109e30bd20856fa79fe5f752ae</link><title>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (Baha 1920x1080 AVC AAC MP4)</title><description>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (Baha 1920x1080 AVC AAC MP4)[345.59 MB]</description><torrent xmlns="https://mikanani.me/0.1/"><link>https://mikanani.me/Home/Episode/8e753b6287da23109e30bd20856fa79fe5f752ae</link><contentLength>362377376</contentLength><pubDate>2022-07-13T20:30:56.945</pubDate></torrent><enclosure type="application/x-bittorrent" length="362377376" url="https://mikanani.me/Download/20220713/8e753b6287da23109e30bd20856fa79fe5f752ae.torrent" /></item><item><guid isPermaLink="false">[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (B-Global 1920x1080 HEVC AAC MKV)</guid><link>https://mikanani.me/Home/Episode/f11fbd3ecc9a504a65787e81955916d6647a04d5</link><title>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (B-Global 1920x1080 HEVC AAC MKV)</title><description>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (B-Global 1920x1080 HEVC AAC MKV)[181.31 MB]</description><torrent xmlns="https://mikanani.me/0.1/"><link>https://mikanani.me/Home/Episode/f11fbd3ecc9a504a65787e81955916d6647a04d5</link><contentLength>190117312</contentLength><pubDate>2022-07-13T20:30:40.725</pubDate></torrent><enclosure type="application/x-bittorrent" length="190117312" url="https://mikanani.me/Download/20220713/f11fbd3ecc9a504a65787e81955916d6647a04d5.torrent" /></item><item><guid isPermaLink="false">[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (B-Global 1920x1080 HEVC AAC MKV)</guid><link>https://mikanani.me/Home/Episode/01b10417b268a1029fda0838c8e12b85730c704b</link><title>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (B-Global 1920x1080 HEVC AAC MKV)</title><description>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (B-Global 1920x1080 HEVC AAC MKV)[195.94 MB]</description><torrent xmlns="https://mikanani.me/0.1/"><link>https://mikanani.me/Home/Episode/01b10417b268a1029fda0838c8e12b85730c704b</link><contentLength>205457984</contentLength><pubDate>2022-07-06T23:02:34.08</pubDate></torrent><enclosure type="application/x-bittorrent" length="205457984" url="https://mikanani.me/Download/20220706/01b10417b268a1029fda0838c8e12b85730c704b.torrent" /></item><item><guid isPermaLink="false">[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (Baha 1920x1080 AVC AAC MP4)</guid><link>https://mikanani.me/Home/Episode/83ba1540f2c8ed8f399e1576f3ae1e6217c6e186</link><title>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (Baha 1920x1080 AVC AAC MP4)</title><description>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (Baha 1920x1080 AVC AAC MP4)[380.7MB]</description><torrent xmlns="https://mikanani.me/0.1/"><link>https://mikanani.me/Home/Episode/83ba1540f2c8ed8f399e1576f3ae1e6217c6e186</link><contentLength>399192896</contentLength><pubDate>2022-07-06T23:00:00</pubDate></torrent><enclosure type="application/x-bittorrent" length="399192896" url="https://mikanani.me/Download/20220706/83ba1540f2c8ed8f399e1576f3ae1e6217c6e186.torrent" /></item></channel></rss>`,
      (str) => str.includes('Baha')
    );
    expect(list.length).toBe(2);
  });
});

describe('nyaa', () => {
  test('get items', async () => {
    const rss = `<rss xmlns:atom="http://www.w3.org/2005/Atom" xmlns:nyaa="https://nyaa.si/xmlns/nyaa" version="2.0">
	<channel>
		<title>Nyaa - &#34;我的前女友&#34; - Torrent File RSS</title>
		<description>RSS Feed for &#34;我的前女友&#34;</description>
		<link>https://nyaa.si/</link>
		<atom:link href="https://nyaa.si/?page=rss" rel="self" type="application/rss+xml" />
		<item>
			<title>[NC-Raws] 繼母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (Baha 1920x1080 AVC AAC MP4)</title>
				<link>https://nyaa.si/download/1552026.torrent</link>
				<guid isPermaLink="true">https://nyaa.si/view/1552026</guid>
				<pubDate>Wed, 13 Jul 2022 12:30:55 -0000</pubDate>

				<nyaa:seeders>101</nyaa:seeders>
				<nyaa:leechers>15</nyaa:leechers>
				<nyaa:downloads>615</nyaa:downloads>
				<nyaa:infoHash>8e753b6287da23109e30bd20856fa79fe5f752ae</nyaa:infoHash>
			<nyaa:categoryId>1_3</nyaa:categoryId>
			<nyaa:category>Anime - Non-English-translated</nyaa:category>
			<nyaa:size>345.6 MiB</nyaa:size>
			<nyaa:comments>0</nyaa:comments>
			<nyaa:trusted>No</nyaa:trusted>
			<nyaa:remake>No</nyaa:remake>
			<description><![CDATA[<a href="https://nyaa.si/view/1552026">#1552026 | [NC-Raws] 繼母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (Baha 1920x1080 AVC AAC MP4)</a> | 345.6 MiB | Anime - Non-English-translated | 8e753b6287da23109e30bd20856fa79fe5f752ae]]></description>
		</item>
		<item>
			<title>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (B-Global 1920x1080 HEVC AAC MKV)</title>
				<link>https://nyaa.si/download/1552025.torrent</link>
				<guid isPermaLink="true">https://nyaa.si/view/1552025</guid>
				<pubDate>Wed, 13 Jul 2022 12:30:38 -0000</pubDate>

				<nyaa:seeders>57</nyaa:seeders>
				<nyaa:leechers>10</nyaa:leechers>
				<nyaa:downloads>283</nyaa:downloads>
				<nyaa:infoHash>f11fbd3ecc9a504a65787e81955916d6647a04d5</nyaa:infoHash>
			<nyaa:categoryId>1_2</nyaa:categoryId>
			<nyaa:category>Anime - English-translated</nyaa:category>
			<nyaa:size>181.3 MiB</nyaa:size>
			<nyaa:comments>0</nyaa:comments>
			<nyaa:trusted>No</nyaa:trusted>
			<nyaa:remake>No</nyaa:remake>
			<description><![CDATA[<a href="https://nyaa.si/view/1552025">#1552025 | [NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 02 (B-Global 1920x1080 HEVC AAC MKV)</a> | 181.3 MiB | Anime - English-translated | f11fbd3ecc9a504a65787e81955916d6647a04d5]]></description>
		</item>
		<item>
			<title>[NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (B-Global 1920x1080 HEVC AAC MKV)</title>
				<link>https://nyaa.si/download/1549582.torrent</link>
				<guid isPermaLink="true">https://nyaa.si/view/1549582</guid>
				<pubDate>Wed, 06 Jul 2022 15:02:32 -0000</pubDate>

				<nyaa:seeders>37</nyaa:seeders>
				<nyaa:leechers>1</nyaa:leechers>
				<nyaa:downloads>507</nyaa:downloads>
				<nyaa:infoHash>01b10417b268a1029fda0838c8e12b85730c704b</nyaa:infoHash>
			<nyaa:categoryId>1_2</nyaa:categoryId>
			<nyaa:category>Anime - English-translated</nyaa:category>
			<nyaa:size>195.9 MiB</nyaa:size>
			<nyaa:comments>0</nyaa:comments>
			<nyaa:trusted>No</nyaa:trusted>
			<nyaa:remake>No</nyaa:remake>
			<description><![CDATA[<a href="https://nyaa.si/view/1549582">#1549582 | [NC-Raws] 继母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (B-Global 1920x1080 HEVC AAC MKV)</a> | 195.9 MiB | Anime - English-translated | 01b10417b268a1029fda0838c8e12b85730c704b]]></description>
		</item>
		<item>
			<title>[NC-Raws] 繼母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (Baha 1920x1080 AVC AAC MP4)</title>
				<link>https://nyaa.si/download/1549578.torrent</link>
				<guid isPermaLink="true">https://nyaa.si/view/1549578</guid>
				<pubDate>Wed, 06 Jul 2022 15:00:47 -0000</pubDate>

				<nyaa:seeders>56</nyaa:seeders>
				<nyaa:leechers>3</nyaa:leechers>
				<nyaa:downloads>1023</nyaa:downloads>
				<nyaa:infoHash>83ba1540f2c8ed8f399e1576f3ae1e6217c6e186</nyaa:infoHash>
			<nyaa:categoryId>1_3</nyaa:categoryId>
			<nyaa:category>Anime - Non-English-translated</nyaa:category>
			<nyaa:size>380.7 MiB</nyaa:size>
			<nyaa:comments>0</nyaa:comments>
			<nyaa:trusted>No</nyaa:trusted>
			<nyaa:remake>No</nyaa:remake>
			<description><![CDATA[<a href="https://nyaa.si/view/1549578">#1549578 | [NC-Raws] 繼母的拖油瓶是我的前女友 / Mamahaha no Tsurego - 01 (Baha 1920x1080 AVC AAC MP4)</a> | 380.7 MiB | Anime - Non-English-translated | 83ba1540f2c8ed8f399e1576f3ae1e6217c6e186]]></description>
		</item>
	</channel>
</rss>`;
    const list = await getItems(rss);
    expect(list.length).toBe(4);
  });
});
