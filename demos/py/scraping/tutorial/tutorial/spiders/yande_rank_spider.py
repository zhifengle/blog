import scrapy
from pathlib import PurePosixPath, Path
from urllib.parse import urlparse

from tutorial.items import YandeImageItem

OUTPUT_PATH = r"D:\pic\yande_rank"

class YandeRank(scrapy.Spider):
    custom_settings = {
        'DEFAULT_REQUEST_HEADERS': {
            'User-Agent':
            'Mozilla/5.0 (Macintosh; Intel Mac OS X 13_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15'
        },
        'DOWNLOADER_MIDDLEWARES': {
            'tutorial.middlewares.ProxyMiddleware': 543
        },
        'ITEM_PIPELINES': {
            'tutorial.pipelines.MyImagesPipeline': 1
        },
        'DOWNLOAD_DELAY': 0.5,
        'IMAGES_STORE': OUTPUT_PATH
    }
    name = "yande_rank"

    def start_requests(self):
        year = 2020
        for i in range(1, 13):
            p = Path(rf"{OUTPUT_PATH}\{year}-{i:02d}")
            if not p.exists():
                p.mkdir(parents=True)
            yield scrapy.Request(
                f"https://yande.re/post/popular_by_month?month={i}&year={year}",
                meta={
                    'month': i,
                    'year': year
                },
                callback=self.parse)

    def parse(self, response):
        post_urls = response.css("a.thumb::attr(href)").getall()
        for url in post_urls:
            yield response.follow(url,
                                  callback=self.parse_post,
                                  meta=response.meta)

    def parse_post(self, response):
        img_url = response.css("a#highres::attr(href)").get()
        img_name = f"{response.meta['year']}-{response.meta['month']:02d}\{PurePosixPath(urlparse(img_url).path).name}"
        yield YandeImageItem(image_name=img_name, image_url=img_url)
