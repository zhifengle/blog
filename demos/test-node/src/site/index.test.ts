import { getMagnet } from './mikanme';
import { getItems } from './nyaa';
import { getItems as getItemsDmhy } from './dmhy';

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

describe('dmhy', () => {
  test('get magnet', async () => {
    const list = await getItemsDmhy(
      `<?xml version="1.0" encoding="utf-8"?>
<rss version="2.0"
xmlns:content="http://purl.org/rss/1.0/modules/content/"
xmlns:wfw="http://wellformedweb.org/CommentAPI/"
>
<channel>
<title><![CDATA[NC-Raws-動漫花園資源網]]></title>
<link>http://share.dmhy.org</link>
<description><![CDATA[動漫花園資訊網是一個動漫愛好者交流的平台,提供最及時,最全面的動畫,漫畫,動漫音樂,動漫下載,BT,ED,動漫遊戲,資訊,分享,交流,讨论.]]></description>
<language>zh-cn</language>
<pubDate>Sun, 17 Jul 2022 09:12:06 +0800</pubDate>
<item>
<title><![CDATA[[NC-Raws] 契約之吻 / Engage Kiss - 03 (Baha 1920x1080 AVC AAC MP4)]]></title>
<link>http://share.dmhy.org/topics/view/605963_NC-Raws_Engage_Kiss_-_03_Baha_1920x1080_AVC_AAC_MP4.html</link>
<pubDate>Sun, 17 Jul 2022 01:01:18 +0800</pubDate>
<description><![CDATA[<p><img src="https://rr1---bh.raws.dev/B/2KU/19/54dc3f385b60f2038074f6f6801hax35.JPG" alt="alt" /></p> <hr /> <p><strong><i>Engage Kiss - EP 03</i></strong><br /><strong>[<a href="https://www.themoviedb.org/tv/196040-engage-kiss" target="_blank" rel="external nofollow">tmdb</a>] | [<a href="https://bgm.tv/subject/375817" target="_blank" rel="external nofollow">bangumi.tv</a>]</strong></p> <hr /> <p><strong>Information:</strong></p> <ul>
<li><strong>Overall Bit Rate:</strong> 3 128 kb/s</li> <li><strong>Subtitle:</strong> HardSub - 中文（繁體）</li> <li><strong>Duration:</strong> 00:24:00.085</li> <li><strong>CRC32:</strong> 4666B25C</li> <li><strong><a href="https://rentry.co/mvvd7/raw" target="_blank" rel="external nofollow">MediaInfo</a></strong></li>
</ul> <hr /> <p>In case of any issues with the file, please let us know via the contact details below.<br /><strong>Xunlei has been banned by default.</strong><br /><strong>Seeding after downloading is appreciated.</strong></p> <hr /> <p>Join us on Telegram at <a href="https://t.me/NC_Raws_Channel" target="_blank" rel="external nofollow">NC-Raws</a> | <a href="https://nc-raws.org" target="_blank" rel="external nofollow">Website</a></p>]]></description>
<enclosure url="magnet:?xt=urn:btih:XT22K5TO77JLROATVW5JR2SECT3JZNNT&amp;dn=&amp;tr=http%3A%2F%2F104.143.10.186%3A8000%2Fannounce&amp;tr=udp%3A%2F%2F104.143.10.186%3A8000%2Fannounce&amp;tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2Ftracker3.itzmx.com%3A6961%2Fannounce&amp;tr=http%3A%2F%2Ftracker4.itzmx.com%3A2710%2Fannounce&amp;tr=http%3A%2F%2Ftracker.publicbt.com%3A80%2Fannounce&amp;tr=http%3A%2F%2Ftracker.prq.to%2Fannounce&amp;tr=http%3A%2F%2Fopen.acgtracker.com%3A1096%2Fannounce&amp;tr=https%3A%2F%2Ft-115.rhcloud.com%2Fonly_for_ylbud&amp;tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&amp;tr=http%3A%2F%2Ftracker2.itzmx.com%3A6961%2Fannounce&amp;tr=udp%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&amp;tr=udp%3A%2F%2Ftracker2.itzmx.com%3A6961%2Fannounce&amp;tr=udp%3A%2F%2Ftracker3.itzmx.com%3A6961%2Fannounce&amp;tr=udp%3A%2F%2Ftracker4.itzmx.com%3A2710%2Fannounce&amp;tr=http%3A%2F%2Ftr.bangumi.moe%3A6969%2Fannounce&amp;tr=http%3A%2F%2Ft.nyaatracker.com%2Fannounce&amp;tr=http%3A%2F%2Fopen.nyaatorrents.info%3A6544%2Fannounce&amp;tr=http%3A%2F%2Ft2.popgo.org%3A7456%2Fannonce&amp;tr=http%3A%2F%2Fshare.camoe.cn%3A8080%2Fannounce&amp;tr=http%3A%2F%2Fopentracker.acgnx.se%2Fannounce&amp;tr=http%3A%2F%2Ftracker.acgnx.se%2Fannounce&amp;tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&amp;tr=http%3A%2F%2Ft.acg.rip%3A6699%2Fannounce"  length="1"  type="application/x-bittorrent" ></enclosure>
<author><![CDATA[九十九朔夜]]></author>
<guid isPermaLink="true" >http://share.dmhy.org/topics/view/605963_NC-Raws_Engage_Kiss_-_03_Baha_1920x1080_AVC_AAC_MP4.html</guid>
<category domain="http://share.dmhy.org/topics/list/sort_id/2" ><![CDATA[動畫]]></category>
</item>
<item>
<title><![CDATA[[NC-Raws] 契約之吻 / Engage Kiss - 02 (Baha 1920x1080 AVC AAC MP4)]]></title>
<link>http://share.dmhy.org/topics/view/605207_NC-Raws_Engage_Kiss_-_02_Baha_1920x1080_AVC_AAC_MP4.html</link>
<pubDate>Sun, 10 Jul 2022 01:01:21 +0800</pubDate>
<description><![CDATA[<p><img src="https://rr1---bh.raws.dev/B/2KU/56/7b9b9efce942457e3c770364ba1h5mg5.JPG" alt="alt" /></p> <hr /> <p><strong><i>Engage Kiss - EP 02</i></strong><br /><strong>[<a href="https://www.themoviedb.org/tv/196040-engage-kiss" target="_blank" rel="external nofollow">tmdb</a>] | [<a href="https://bgm.tv/subject/375817" target="_blank" rel="external nofollow">bangumi.tv</a>]</strong></p> <hr /> <p><strong>Information:</strong></p> <ul>
<li><strong>Overall Bit Rate:</strong> 2 667 kb/s</li> <li><strong>Subtitle:</strong> HardSub - 中文（繁體）</li> <li><strong>Duration:</strong> 00:24:00.085</li> <li><strong>CRC32:</strong> AA78F314</li> <li><strong><a href="https://rentry.co/xh2g7n/raw" target="_blank" rel="external nofollow">MediaInfo</a></strong></li>
</ul> <hr /> <p>In case of any issues with the file, please let us know via the contact details below.<br /><strong>Xunlei has been banned by default.</strong><br /><strong>Seeding after downloading is appreciated.</strong></p> <hr /> <p>Join us on Telegram at <a href="https://t.me/NC_Raws_Channel" target="_blank" rel="external nofollow">NC-Raws</a> | <a href="https://nc-raws.org" target="_blank" rel="external nofollow">Website</a></p>]]></description>
<enclosure url="magnet:?xt=urn:btih:4CH75Z2H5B2SMTBVFCGT75SDXPNE4ATO&amp;dn=&amp;tr=http%3A%2F%2F104.143.10.186%3A8000%2Fannounce&amp;tr=udp%3A%2F%2F104.143.10.186%3A8000%2Fannounce&amp;tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2Ftracker3.itzmx.com%3A6961%2Fannounce&amp;tr=http%3A%2F%2Ftracker4.itzmx.com%3A2710%2Fannounce&amp;tr=http%3A%2F%2Ftracker.publicbt.com%3A80%2Fannounce&amp;tr=http%3A%2F%2Ftracker.prq.to%2Fannounce&amp;tr=http%3A%2F%2Fopen.acgtracker.com%3A1096%2Fannounce&amp;tr=https%3A%2F%2Ft-115.rhcloud.com%2Fonly_for_ylbud&amp;tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&amp;tr=http%3A%2F%2Ftracker2.itzmx.com%3A6961%2Fannounce&amp;tr=udp%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&amp;tr=udp%3A%2F%2Ftracker2.itzmx.com%3A6961%2Fannounce&amp;tr=udp%3A%2F%2Ftracker3.itzmx.com%3A6961%2Fannounce&amp;tr=udp%3A%2F%2Ftracker4.itzmx.com%3A2710%2Fannounce&amp;tr=http%3A%2F%2Ftr.bangumi.moe%3A6969%2Fannounce&amp;tr=http%3A%2F%2Ft.nyaatracker.com%2Fannounce&amp;tr=http%3A%2F%2Fopen.nyaatorrents.info%3A6544%2Fannounce&amp;tr=http%3A%2F%2Ft2.popgo.org%3A7456%2Fannonce&amp;tr=http%3A%2F%2Fshare.camoe.cn%3A8080%2Fannounce&amp;tr=http%3A%2F%2Fopentracker.acgnx.se%2Fannounce&amp;tr=http%3A%2F%2Ftracker.acgnx.se%2Fannounce&amp;tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&amp;tr=http%3A%2F%2Ft.acg.rip%3A6699%2Fannounce"  length="1"  type="application/x-bittorrent" ></enclosure>
<author><![CDATA[九十九朔夜]]></author>
<guid isPermaLink="true" >http://share.dmhy.org/topics/view/605207_NC-Raws_Engage_Kiss_-_02_Baha_1920x1080_AVC_AAC_MP4.html</guid>
<category domain="http://share.dmhy.org/topics/list/sort_id/2" ><![CDATA[動畫]]></category>
</item>
<item>
<title><![CDATA[[NC-Raws] 契約之吻 / Engage Kiss - 01 (Baha 1920x1080 AVC AAC MP4)]]></title>
<link>http://share.dmhy.org/topics/view/604448_NC-Raws_Engage_Kiss_-_01_Baha_1920x1080_AVC_AAC_MP4.html</link>
<pubDate>Sun, 03 Jul 2022 01:01:35 +0800</pubDate>
<description><![CDATA[<p><img src="https://rr1---bh.raws.dev/B/2KU/95/4c724213d2a73740e5e3ff6a181h1n35.JPG" alt="alt" /></p> <hr /> <p><strong><i>Engage Kiss - EP 01</i></strong><br /><strong>[<a href="https://www.themoviedb.org/tv/196040-engage-kiss" target="_blank" rel="external nofollow">tmdb</a>] | [<a href="https://bgm.tv/subject/375817" target="_blank" rel="external nofollow">bangumi.tv</a>]</strong></p> <hr /> <p><strong>Information:</strong></p> <ul>
<li><strong>Overall Bit Rate:</strong> 3 309 kb/s</li> <li><strong>Subtitle:</strong> HardSub - 中文（繁體）</li> <li><strong>Duration:</strong> 00:24:00.044</li> <li><strong>CRC32:</strong> 2198BCEC</li> <li><strong><a href="https://rentry.co/kahzv/raw" target="_blank" rel="external nofollow">MediaInfo</a></strong></li>
</ul> <hr /> <p>In case of any issues with the file, please let us know via the contact details below.<br /><strong>Xunlei has been banned by default.</strong><br /><strong>Seeding after downloading is appreciated.</strong></p> <hr /> <p>Join us on Telegram at <a href="https://t.me/NC_Raws_Channel" target="_blank" rel="external nofollow">NC-Raws</a> | <a href="https://nc-raws.org" target="_blank" rel="external nofollow">Website</a></p>]]></description>
<enclosure url="magnet:?xt=urn:btih:DWZ6Z6DKEFVJ4TE5FIIP7EKJLEMNC7DU&amp;dn=&amp;tr=http%3A%2F%2F104.143.10.186%3A8000%2Fannounce&amp;tr=udp%3A%2F%2F104.143.10.186%3A8000%2Fannounce&amp;tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&amp;tr=udp%3A%2F%2Ftracker3.itzmx.com%3A6961%2Fannounce&amp;tr=http%3A%2F%2Ftracker4.itzmx.com%3A2710%2Fannounce&amp;tr=http%3A%2F%2Ftracker.publicbt.com%3A80%2Fannounce&amp;tr=http%3A%2F%2Ftracker.prq.to%2Fannounce&amp;tr=http%3A%2F%2Fopen.acgtracker.com%3A1096%2Fannounce&amp;tr=https%3A%2F%2Ft-115.rhcloud.com%2Fonly_for_ylbud&amp;tr=http%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&amp;tr=http%3A%2F%2Ftracker2.itzmx.com%3A6961%2Fannounce&amp;tr=udp%3A%2F%2Ftracker1.itzmx.com%3A8080%2Fannounce&amp;tr=udp%3A%2F%2Ftracker2.itzmx.com%3A6961%2Fannounce&amp;tr=udp%3A%2F%2Ftracker3.itzmx.com%3A6961%2Fannounce&amp;tr=udp%3A%2F%2Ftracker4.itzmx.com%3A2710%2Fannounce&amp;tr=http%3A%2F%2Ftr.bangumi.moe%3A6969%2Fannounce&amp;tr=http%3A%2F%2Ft.nyaatracker.com%2Fannounce&amp;tr=http%3A%2F%2Fopen.nyaatorrents.info%3A6544%2Fannounce&amp;tr=http%3A%2F%2Ft2.popgo.org%3A7456%2Fannonce&amp;tr=http%3A%2F%2Fshare.camoe.cn%3A8080%2Fannounce&amp;tr=http%3A%2F%2Fopentracker.acgnx.se%2Fannounce&amp;tr=http%3A%2F%2Ftracker.acgnx.se%2Fannounce&amp;tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&amp;tr=http%3A%2F%2Ft.acg.rip%3A6699%2Fannounce"  length="1"  type="application/x-bittorrent" ></enclosure>
<author><![CDATA[九十九朔夜]]></author>
<guid isPermaLink="true" >http://share.dmhy.org/topics/view/604448_NC-Raws_Engage_Kiss_-_01_Baha_1920x1080_AVC_AAC_MP4.html</guid>
<category domain="http://share.dmhy.org/topics/list/sort_id/2" ><![CDATA[動畫]]></category>
</item>
</channel>
</rss>`
    );
    expect(list.length).toBe(3);
  });
});
