import scrapy
from pathlib import PurePosixPath, Path
from urllib.parse import urlparse

from tutorial.items import YandeImageItem

OUTPUT_PATH = r"D:\pic\yande_rank"


class YandeRank(scrapy.Spider):
    custom_settings = {
        'DEFAULT_REQUEST_HEADERS': {
            'User-Agent':
            # 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.83 Mobile/15E148 Safari/604.1'
            # 'Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.5481.63 Mobile Safari/537.36'
            'Mozilla/5.0 (Macintosh; Intel Mac OS X 13_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15'
        },
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        'DOWNLOAD_DELAY': 0.5,
        'IMAGES_STORE': OUTPUT_PATH,
    }
    name = "yande_rank"

    def start_requests(self):
        host = getattr(self, 'host', 'yande.re')
        # @TODO 2007 - 2022

        for year in range(2007, 2014):
            for i in range(1, 13):
                p = Path(rf"{OUTPUT_PATH}\{year}-{i:02d}")
                if not p.exists():
                    p.mkdir(parents=True)
                yield scrapy.Request(
                    f"https://{host}/post/popular_by_month?month={i}&year={year}",
                    meta={'month': i, 'year': year},
                    callback=self.parse,
                )

    def parse(self, response):
        post_urls = response.css("a.thumb::attr(href)").getall()
        for url in post_urls:
            yield response.follow(url, callback=self.parse_post, meta=response.meta)

    def parse_post(self, response):
        img_url = response.css("a#highres::attr(href)").get()
        img_name = PurePosixPath(urlparse(img_url).path).name
        if response.meta['year'] and response.meta['month']:
            img_name = (
                f"{response.meta['year']}-{response.meta['month']:02d}\{img_name}"
            )
        yield YandeImageItem(image_name=img_name, image_url=img_url)


class YandePost(scrapy.Spider):
    custom_settings = {
        'DEFAULT_REQUEST_HEADERS': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) Gecko/20100101 Firefox/102.0'
            # 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.83 Mobile/15E148 Safari/604.1'
            # 'Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.5481.63 Mobile Safari/537.36'
            # 'Mozilla/5.0 (Macintosh; Intel Mac OS X 13_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15'
        },
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        # 'DOWNLOAD_DELAY': 0.5,
        'IMAGES_STORE': r"D:\pic\yande_posts",
        'IMAGES_EXPIRES': 0,
    }
    name = "yande_post"

    def start_requests(self):
        host = getattr(self, 'host', 'yande.re')
        tags = getattr(self, 'tags', '')
        url = getattr(self, 'url', '')
        if tags:
            url = f"https://{host}/post?tags={tags}"
        elif url == '':
            url = f'https://{host}/post/popular_recent'
        yield scrapy.Request(url, callback=self.parse)

    def parse(self, response):
        img_urls = response.css("a.directlink.largeimg::attr(href)").getall()
        for url in img_urls:
            img_name = PurePosixPath(urlparse(url).path).name
            yield YandeImageItem(image_name=img_name, image_url=url)
        next_page = response.css("a.next_page::attr(href)").get()
        if next_page is not None:
            yield response.follow(next_page, callback=self.parse)
