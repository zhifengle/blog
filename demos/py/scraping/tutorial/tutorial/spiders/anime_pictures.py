from pathlib import Path, PurePosixPath
from urllib.parse import urlparse, quote, unquote
import scrapy

from tutorial.items import ImageItem
from tutorial.utils import sanitize_name


class AnimePictures(scrapy.Spider):
    custom_settings = {
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        'DOWNLOAD_DELAY': 0.5,
        'MEDIA_ALLOW_REDIRECTS': True,
        'IMAGES_STORE': str(Path.home() / 'Downloads/pic/anime_pictures'),
        'IMAGES_EXPIRES': 0,
    }
    name = "anime_pictures"
    api_url = "https://anime-pictures.net/api/v3/posts"

    def start_requests(self):
        search_tag = getattr(self, 'tag', '')
        if search_tag == '':
            self.logger.error("No search tag provided")
            return
        first_page_url = f"{self.api_url}?page=0&search_tag={quote(search_tag)}"
        yield scrapy.Request(
            first_page_url,
            headers={'Content-Type': 'application/json'},
            callback=self.parse,
        )

    def parse(self, response):
        search_tag = getattr(self, 'tag', '')
        data = response.json()
        cur_page = data['page_number']
        if data['posts']:
            for post in data['posts']:
                yield scrapy.Request(
                    f"https://anime-pictures.net/posts/{post['id']}",
                    callback=self.parse_img_post,
                )
            if cur_page < data['max_pages']:
                yield scrapy.Request(
                    f"https://anime-pictures.net/api/v3/posts?page={cur_page+1}&search_tag={quote(search_tag)}",
                    headers={'Content-Type': 'application/json'},
                    callback=self.parse,
                )

    def parse_img_post(self, response):
        search_tag = getattr(self, 'tag', '')
        img_url = response.css("#rating > a.download_icon::attr(href)").get()
        img_name = PurePosixPath(urlparse(img_url).path).name
        img_name = sanitize_name(unquote(img_name))
        img_name = f"{search_tag}/{img_name}"
        yield ImageItem(image_name=img_name, image_url=img_url)

class AnimePicturesTop(scrapy.Spider):
    custom_settings = {
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        'DOWNLOAD_DELAY': 0.5,
        'MEDIA_ALLOW_REDIRECTS': True,
        'IMAGES_STORE': str(Path.home() / 'Downloads/pic/anime_pictures_top'),
        'IMAGES_EXPIRES': 0,
    }
    name = "anime_pictures_top"
    api_url = "https://anime-pictures.net/api/v3/top"

    def start_requests(self):
        type = getattr(self, 'type', 'day')
        url = f"{self.api_url}?length={type}&erotic="
        yield scrapy.Request(
            url,
            headers={'Content-Type': 'application/json'},
            callback=self.parse,
        )

    def parse(self, response):
        data = response.json()
        if data['success'] == False:
            return
        if data['top']:
            for post in data['top']:
                yield scrapy.Request(
                    f"https://anime-pictures.net/posts/{post['id']}",
                    callback=self.parse_img_post,
                )

    def parse_img_post(self, response):
        t = getattr(self, 'type', 'day')
        img_url = response.css("#rating > a.download_icon::attr(href)").get()
        img_name = PurePosixPath(urlparse(img_url).path).name
        img_name = sanitize_name(unquote(img_name))
        img_name = f"{t}/{img_name}"
        yield ImageItem(image_name=img_name, image_url=img_url, referer=response.url)
