from pathlib import Path
from urllib.parse import quote
import scrapy

from tutorial.utils import patch_url


OUTPUT_PATH = str(Path.home() / "Downloads/spider")


class NyaaItem(scrapy.Item):
    url = scrapy.Field()
    torrent_url = scrapy.Field()
    magnet = scrapy.Field()
    title = scrapy.Field()
    category = scrapy.Field()
    size = scrapy.Field()
    date = scrapy.Field()
    seeders = scrapy.Field()
    leechers = scrapy.Field()
    completed = scrapy.Field()


class Nyaa(scrapy.Spider):
    custom_settings = {
        'FEED_EXPORT_ENCODING': 'utf-8',
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'DOWNLOAD_DELAY': 3,
        'SQLITE_DB_PATH': f"{OUTPUT_PATH}/nyaa.db",
        'ITEM_PIPELINES': {'tutorial.pipelines.NyaaSqlitePipeline': 300},
    }
    name = "nyaa"

    def start_requests(self):
        search_tag = getattr(self, 'q', None)
        category = getattr(self, 'c', '0_0')
        host = getattr(self, 'host', 'nyaa.si')
        url = f"https://{host}/?f=0&c={category}"
        desc = getattr(self, 'desc', False)
        mode = getattr(self, 'mode', None)
        if mode == 't':
            tags = []
            for tag in tags:
                url = patch_url(url, q=quote(tag))
                if desc:
                    url = patch_url(url, s='downloads', o='desc')
                yield scrapy.Request(url=url, callback=self.parse)
        elif mode is None:
            if search_tag:
                url = patch_url(url, q=search_tag)
            if desc:
                url = patch_url(url, s='downloads', o='desc')
            yield scrapy.Request(url=url, callback=self.parse)

    def parse(self, response):
        tr_list = response.css(
            'body > div.container > div.table-responsive > table > tbody > tr'
        )
        for tr in tr_list:
            category = tr.css('td:nth-child(1) > a::attr(href)').get().split('=')[1]
            title_selector = tr.css('td:nth-child(2) > a')
            title = title_selector.css('::text').get().strip()
            url = title_selector.css('::attr(href)').get()
            if len(title_selector) > 1:
                title = title_selector[1].css('::text').get().strip()
                url = title_selector[1].css('::attr(href)').get()
            anchor_selector = tr.css('td:nth-child(3) > a')
            torrent_url = None
            if len(anchor_selector) > 1:
                torrent_url = anchor_selector[0].css('::attr(href)').get()
                magnet = anchor_selector[1].css('::attr(href)').get().split('&dn=')[0]
            else:
                temp_url = anchor_selector.css('::attr(href)').get()
                if temp_url.startswith('magnet'):
                    magnet = temp_url.split('&dn=')[0]
            size = tr.css('td:nth-child(4)::text').get()
            date = tr.css('td:nth-child(5)::text').get()
            seeders = tr.css('td:nth-child(6)::text').get()
            leechers = tr.css('td:nth-child(7)::text').get()
            completed = tr.css('td:nth-child(8)::text').get()
            yield NyaaItem(
                url=url,
                torrent_url=torrent_url,
                magnet=magnet,
                title=title,
                category=category,
                size=size,
                date=date,
                seeders=seeders,
                leechers=leechers,
                completed=completed,
            )
        next_page = response.css('ul.pagination li:last-child > a::attr(href)').get()
        if next_page is not None:
            yield response.follow(next_page, self.parse)
