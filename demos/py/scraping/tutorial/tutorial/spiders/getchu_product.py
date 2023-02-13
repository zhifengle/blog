import json
import scrapy
from pathlib import PurePosixPath, Path
from urllib.parse import urlparse

OUTPUT_PATH = r"C:\pic\getchu_game"


class GetchuProductItem(scrapy.Item):
    url = scrapy.Field()
    cover_url = scrapy.Field()
    title = scrapy.Field()
    brand = scrapy.Field()
    release_date = scrapy.Field()
    scenario = scrapy.Field()
    illustrator = scrapy.Field()
    story = scrapy.Field()
    raw_info = scrapy.Field()

class GetchuGame(scrapy.Spider):
    custom_settings = {
        'DEFAULT_REQUEST_HEADERS': {
            'User-Agent':
            # 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.83 Mobile/15E148 Safari/604.1'
            # 'Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.5481.63 Mobile Safari/537.36'
            'Mozilla/5.0 (Macintosh; Intel Mac OS X 13_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15'
        },
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        # 'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        'DOWNLOAD_DELAY': 0.5,
        'IMAGES_STORE': OUTPUT_PATH,
    }
    name = "getchu_game"

    def start_requests(self):
        for year in range(2023, 2024):
            for i in range(1, 2):
                ym = f"{year}-{i:02d}"
                # p = Path(rf"{OUTPUT_PATH}\{ym}")
                # if not p.exists():
                #     p.mkdir(parents=True)
                yield scrapy.Request(
                    f"https://www.getchu.com/all/month_title.html?genre=pc_soft&gage=adult&year={year}&month={i}",
                    cookies={'getchu_adalt_flag': 'getchu.com'},
                    meta={'ym': ym},
                    callback=self.parse,
                )
    def parse(self, response):
        product_urls = response.css(".product .black a::attr(href)").getall()
        for url in product_urls:
            yield response.follow(url,
                                  callback=self.parse_product,
                                  meta=response.meta)
    def parse_product(self, response):
        map_keys = response.css("#soft_table table tr > td:nth-child(1)::text").getall()
        map_values = response.css("#soft_table table tr > td:nth-child(2)::text").getall()
        info_map = dict(zip(map(str.strip, map_keys), map(str.strip, map_values)))
        raw_info = json.dumps(info_map, ensure_ascii=False)
        story = response.css(".tabletitle.tabletitle_1 + div::text").get()
        pass