import json
from pathlib import Path
import scrapy
from urllib.parse import urlparse, parse_qs

from tutorial.utils import get_start_and_end

OUTPUT_PATH = str(Path.home() / "Downloads/pic/getchu_product")
GENRE_ARR = ['pc_soft', 'dvd_game', 'doujin', 'anime_dvd']


class GetchuProductItem(scrapy.Item):
    url = scrapy.Field()
    cover_url = scrapy.Field()
    genre = scrapy.Field()
    title = scrapy.Field()
    brand = scrapy.Field()
    release_date = scrapy.Field()
    description = scrapy.Field()
    raw_info = scrapy.Field()



def deal_info_key(key):
    return key.strip().replace("：", "").replace(":", "")


def remove_end_brackets(text, bracket="（"):
    return text.split(bracket)[0].strip()


class GetchuProduct(scrapy.Spider):
    custom_settings = {
        'FEED_EXPORT_ENCODING': 'utf-8',
        'DEFAULT_REQUEST_HEADERS': {
            'User-Agent':
            'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/109.0'
            # 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.83 Mobile/15E148 Safari/604.1'
            # 'Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.5481.63 Mobile Safari/537.36'
            # 'Mozilla/5.0 (Macintosh; Intel Mac OS X 13_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15'
        },
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {
            'tutorial.pipelines.GetchuImagesPipeline': 1,
            'tutorial.pipelines.GetchuSqlitePipeline': 300,
        },
        # 'DOWNLOAD_DELAY': 0.5,
        'IMAGES_STORE': OUTPUT_PATH,
        'IMAGES_EXPIRES': 0,
        'SQLITE_DB_PATH': f"{OUTPUT_PATH}/getchu_product.db"
    }
    name = "getchu_product"

    def start_requests(self):
        genre = getattr(self, 'genre', 'pc_soft')
        year_range = get_start_and_end(getattr(self, 'year_range', '2010-2021'))
        month_range = get_start_and_end(getattr(self, 'month_range', '1-12'))
        if genre not in GENRE_ARR:
            genre = GENRE_ARR[0]
        for year in range(year_range[0], year_range[1] + 1):
            for i in range(month_range[0], month_range[1] + 1):
                ym = f"{year}-{i:02d}"
                yield scrapy.Request(
                    f"https://www.getchu.com/all/month_title.html?genre={genre}&gage=adult&year={year}&month={i}",
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
        info_map['ブランド'] = remove_end_brackets(info_map.get('ブランド', ''))
        raw_info = json.dumps(info_map, ensure_ascii=False)
        description = ''.join(
            response.css(".tabletitle.tabletitle_1 + div ::text").getall()
        ).strip()
        title = response.css("#soft-title::text").get()
        pid = parse_qs(urlparse(response.url).query).get("id")[0]
        cover_url = f"https://www.getchu.com/brandnew/{pid}/c{pid}package.jpg"
        genre = getattr(self, 'genre', 'pc_soft')
        yield GetchuProductItem(
            title=title.strip(),
            url=response.url,
            cover_url=cover_url,
            brand=info_map.get("ブランド", ''),
            release_date=info_map.get("発売日", ''),
            description=description,
            raw_info=raw_info,
            genre=GENRE_ARR.index(genre),
        )
