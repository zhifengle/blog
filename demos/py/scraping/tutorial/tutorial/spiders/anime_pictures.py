from pathlib import Path, PurePosixPath
from random import random
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
        start_num = int(getattr(self, 'start', '0'))
        query = getattr(self, 'query', None)
        post_ids = getattr(self, 'post_ids', None)
        if post_ids is not None:
            post_ids = post_ids.split(',')
            for post_id in post_ids:
                yield scrapy.Request(
                    f"https://anime-pictures.net/posts/{post_id}",
                    callback=self.parse_img_post,
                )
            return
        if search_tag == '':
            self.logger.error("No search tag provided")
            return
        first_page_url = self.gen_api_page_url(search_tag, page=start_num, query=query)
        yield scrapy.Request(
            first_page_url,
            headers={'Content-Type': 'application/json'},
            callback=self.parse,
        )

    def parse(self, response):
        search_tag = getattr(self, 'tag', '')
        query = getattr(self, 'query', None)
        end_num = getattr(self, 'end', None)
        if end_num is not None:
            end_num = int(end_num)
        data = response.json()
        cur_page = data['page_number']
        if data['posts']:
            for post in data['posts']:
                yield scrapy.Request(
                    f"https://anime-pictures.net/posts/{post['id']}",
                    callback=self.parse_img_post,
                )
            if cur_page < data['max_pages'] and (end_num is None or cur_page < end_num):
                yield scrapy.Request(
                    self.gen_api_page_url(search_tag, cur_page + 1, query=query),
                    headers={'Content-Type': 'application/json'},
                    callback=self.parse,
                )

    def parse_img_post(self, response):
        search_tag = getattr(self, 'tag', '')
        img_url = response.css("#big_preview_cont > a::attr(href)").get()
        # randomize the image url to avoid 403
        # if random() > 0.9:
        #     img_url = response.css("#rating > a.download_icon::attr(href)").get()
        img_name = PurePosixPath(urlparse(img_url).path).name
        img_name = sanitize_name(unquote(img_name))
        img_name = f"{search_tag}/{img_name}"
        yield ImageItem(image_name=img_name, image_url=img_url, referer=response.url)

    def gen_api_page_url(self, search_tag, page=0, query=None):
        url = f"{self.api_url}?page={page}&search_tag={quote(search_tag)}"
        if query:
            url = f"{url}&{query}"
        return url


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
