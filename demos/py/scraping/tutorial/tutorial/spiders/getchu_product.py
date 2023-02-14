import json
import scrapy
from urllib.parse import urlparse, parse_qs

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


def deal_title(title):
    return (
        title.strip()
        .replace("/", "_")
        .replace(":", "_")
        .replace("?", "_")
        .replace("*", "_")
        .replace('"', "_")
        .replace("<", "_")
        .replace(">", "_")
        .replace("|", "_")
    )


def deal_info_key(key):
    return key.strip().replace("：", "").replace(":", "")


def remove_end_brackets(text, bracket="（"):
    return text.split(bracket)[0].strip()


class GetchuGame(scrapy.Spider):
    custom_settings = {
        'FEED_EXPORT_ENCODING': 'utf-8',
        'DEFAULT_REQUEST_HEADERS': {
            'User-Agent':
            # 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.83 Mobile/15E148 Safari/604.1'
            # 'Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.5481.63 Mobile Safari/537.36'
            'Mozilla/5.0 (Macintosh; Intel Mac OS X 13_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15'
        },
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.GetchuImagesPipeline': 1},
        'DOWNLOAD_DELAY': 0.5,
        'IMAGES_STORE': OUTPUT_PATH,
    }
    name = "getchu_game"

    def start_requests(self):
        for year in range(2023, 2024):
            for i in range(1, 2):
                ym = f"{year}-{i:02d}"
                yield scrapy.Request(
                    f"https://www.getchu.com/all/month_title.html?genre=pc_soft&gage=adult&year={year}&month={i}",
                    cookies={'getchu_adalt_flag': 'getchu.com'},
                    meta={'ym': ym},
                    callback=self.parse,
                )

    def parse(self, response):
        product_urls = response.css(".product a.black::attr(href)").getall()
        for url in product_urls:
            yield response.follow(url, callback=self.parse_product, meta=response.meta)

    def parse_product(self, response):
        info_map = {}
        table_trs = response.css("#soft_table th > table tr")
        for tr in table_trs:
            key = tr.css("td:first-child::text").get()
            if key:
                value = ''.join(tr.css("td:last-child ::text").getall())
                info_map[deal_info_key(key)] = value.strip()
        raw_info = json.dumps(info_map, ensure_ascii=False)
        story = ''.join(response.css(".tabletitle.tabletitle_1 + div ::text").getall())
        title = response.css("#soft-title::text").get()
        pid = parse_qs(urlparse(response.url).query).get("id")[0]
        cover_url = f"https://www.getchu.com/brandnew/{pid}/c{pid}package.jpg"
        yield GetchuProductItem(
            title=deal_title(title),
            url=response.url,
            cover_url=cover_url,
            brand=remove_end_brackets(info_map.get("ブランド", '')),
            release_date=info_map.get("発売日", ''),
            scenario=info_map.get("シナリオ"),
            illustrator=info_map.get("原画"),
            story=story,
            raw_info=raw_info,
        )
