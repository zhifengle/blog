from pathlib import Path, PurePosixPath
from random import random
from urllib.parse import urlparse, quote, unquote
import scrapy
import os
import re

from tutorial.items import ImageItem
from tutorial.utils import (
    get_config_by_url,
    get_cookies_dict,
    get_query_dict,
    patch_url,
    sanitize_name,
)

IMAGES_STORE = str(Path.home() / 'Downloads/pic/anime_pictures')

# def get_downloaded_ids(pic_path):
#     download_ids = []
#     for file in os.listdir(pic_path):
#         search_result = re.search(r'^(\d+)', file)
#         if search_result:
#             post_id = search_result.group(1)
#             download_ids.append(post_id)
#     return download_ids


def get_downloaded_ids(pic_path):
    download_ids = []
    # root, dirs, files = os.walk(pic_path)
    # use os.walk to get all files in subfolders
    for _, _, files in os.walk(pic_path):
        for file in files:
            search_result = re.search(r'^(\d+)', file)
            if search_result:
                post_id = search_result.group(1)
                download_ids.append(int(post_id))
    return download_ids


# @deprecated
class AnimePicturesHtml(scrapy.Spider):
    custom_settings = {
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        'DOWNLOAD_DELAY': 0.5,
        'MEDIA_ALLOW_REDIRECTS': True,
        'IMAGES_STORE': IMAGES_STORE,
        'IMAGES_EXPIRES': 0,
    }
    name = "anime_pictures_html"
    start = 0
    end = 0
    page_size = 80
    api_url = "https://anime-pictures.net/api/v3"
    downloaded_ids = []

    def start_requests(self):
        search_tag = getattr(self, 'search_tag', '')
        query = getattr(self, 'query', None)
        if search_tag == '':
            self.logger.error("No search tag provided")
            return
        # save_path = str(Path(IMAGES_STORE) / sanitize_name(search_tag))
        self.downloaded_ids = get_downloaded_ids(IMAGES_STORE)
        url = patch_url(
            self.api_url + '/posts',
            page=self.start,
            search_tag=search_tag,
            **{
                # 'posts_per_page': self.page_size,
                'order_by': 'rating',
                'ldate': '5',
            },
        )
        if query:
            url = patch_url(url, **get_query_dict(query))
        yield scrapy.Request(
            url,
            headers={'Content-Type': 'application/json'},
            callback=self.parse,
        )

    def parse(self, response):
        end = int(self.end)
        data = response.json()
        cur_page = data['page_number']
        if data['posts']:
            for post in data['posts']:
                # skip downloaded posts
                if int(post['id']) in self.downloaded_ids:
                    continue
                yield scrapy.Request(
                    f"https://anime-pictures.net/posts/{post['id']}",
                    callback=self.parse_img_post,
                )
            if cur_page < data['max_pages'] and cur_page < end:
                yield scrapy.Request(
                    patch_url(response.url, page=cur_page + 1),
                    headers={'Content-Type': 'application/json'},
                    callback=self.parse,
                )

    def parse_img_post(self, response):
        search_tag = getattr(self, 'search_tag', '')
        img_url = response.css("#big_preview_cont > a::attr(href)").get()
        # randomize the image url to avoid 403
        # if random() > 0.9:
        #     img_url = response.css("#rating > a.download_icon::attr(href)").get()
        img_name = PurePosixPath(urlparse(img_url).path).name
        img_name = sanitize_name(unquote(img_name))
        if search_tag:
            img_name = f"{sanitize_name(search_tag)}/{img_name}"
        yield ImageItem(image_name=img_name, image_url=img_url, referer=response.url)


# @deprecated
class AnimePicturesTopHtml(scrapy.Spider):
    custom_settings = {
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        'DOWNLOAD_DELAY': 0.5,
        'MEDIA_ALLOW_REDIRECTS': True,
        'IMAGES_STORE': str(Path.home() / 'Downloads/pic/anime_pictures_top'),
        'IMAGES_EXPIRES': 0,
    }
    name = "anime_pictures_top_html"
    api_url = "https://anime-pictures.net/api/v3"
    downloaded_ids = []
    folder_name = 'day'

    def start_requests(self):
        type = getattr(self, 'type', 'day')
        self.folder_name = type
        site_config = get_config_by_url('https://anime-pictures.net/')
        headers = site_config.get('headers')
        # need cookie in headers
        if int(self.erotics) == 1 and headers:
            self.folder_name += '_erotic'
        # save_path = self.custom_settings['IMAGES_STORE']
        save_path = str(Path(self.custom_settings['IMAGES_STORE']) / self.folder_name)
        self.downloaded_ids = get_downloaded_ids(save_path)
        url = f"{self.api_url}/top?length={type}&erotic={self.erotics}"
        cookies_dict = get_cookies_dict(headers.get('cookie'))
        yield scrapy.Request(
            url,
            headers={'Content-Type': 'application/json'},
            cookies=cookies_dict,
            callback=self.parse,
        )

    def parse(self, response):
        data = response.json()
        site_config = get_config_by_url('https://anime-pictures.net/')
        headers = site_config.get('headers')
        if data['success'] == False:
            return
        if data['top']:
            for post in data['top']:
                if int(post['id']) in self.downloaded_ids:
                    continue
                yield scrapy.Request(
                    f"https://anime-pictures.net/posts/{post['id']}",
                    headers=headers,
                    callback=self.parse_img_post,
                )

    def parse_img_post(self, response):
        t = self.folder_name
        img_url = response.css("#rating > a.download_icon::attr(href)").get()
        img_name = PurePosixPath(urlparse(img_url).path).name
        img_name = sanitize_name(unquote(img_name))
        img_name = f"{t}/{img_name}"
        yield ImageItem(image_name=img_name, image_url=img_url, referer=response.url)


TAG_BLACKLIST = ['realistic', '3d']


class AnimePictures(scrapy.Spider):
    custom_settings = {
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        'CONCURRENT_REQUESTS_PER_DOMAIN': 1,
        'DOWNLOAD_DELAY': 2.5,
        'IMAGES_STORE': str(Path.home() / 'Downloads/pic/anime_pictures'),
        'IMAGES_EXPIRES': 0,
    }
    name = "anime_pictures"
    api_url = "https://anime-pictures.net/api/v3"
    start = 0
    end = 0
    page_size = 80
    downloaded_ids = []
    erotics = 0
    folder_name = ''

    def start_requests(self):
        search_tag = getattr(self, 'search_tag', '')
        if search_tag == '' and self.folder_name not in ['month']:
            self.logger.error("No search tag provided")
            return
        if self.folder_name == '':
            self.folder_name = search_tag
        # self.set_downloaded_ids(self.folder_name)
        self.set_downloaded_ids('')
        url = patch_url(
            self.api_url + '/posts',
            page=self.start,
            search_tag=search_tag,
            **{
                # 'denied_tags': '3d||realistic',
                'posts_per_page': self.page_size,
                'order_by': 'rating',
                'ldate': '5',
            },
        )
        query = getattr(self, 'query', None)
        if query:
            url = patch_url(url, **get_query_dict(query))
        yield scrapy.Request(
            url,
            headers={'Content-Type': 'application/json'},
            cookies=self.get_cookies(),
            callback=self.parse,
        )

    def parse(self, response):
        data = response.json()
        cur_page = data['page_number']
        if data['posts']:
            for post in data['posts']:
                # skip downloaded posts
                if int(post['id']) in self.downloaded_ids:
                    continue
                yield response.follow(
                    # f"{self.api_url}/posts/{post['id']}?extra=similar_pictures&lang=en",
                    f"{self.api_url}/posts/{post['id']}",
                    headers={'Content-Type': 'application/json'},
                    callback=self.parse_post_json,
                )
            if cur_page < data['max_pages'] and cur_page < int(self.end):
                yield scrapy.Request(
                    patch_url(response.url, page=cur_page + 1),
                    headers={'Content-Type': 'application/json'},
                    callback=self.parse,
                )

    def parse_items(self, response):
        data = response.json()
        if data['success'] == False:
            return
        if data['top']:
            for post in data['top']:
                if int(post['id']) in self.downloaded_ids:
                    continue
                yield self.get_image_item(post)

    def parse_post_json(self, response):
        res = response.json()
        tags = res['tags']
        # if any of tag_obj.tag.tag in black list return
        if any(tag_obj['tag']['tag'] in TAG_BLACKLIST for tag_obj in tags):
            return
        yield self.get_image_item(res['post'], res["file_url"])
        if res['tied']:
            for tied_post in res['tied']:
                yield scrapy.Request(
                    f"https://anime-pictures.net/api/v3/posts/{tied_post['id']}",
                    headers={'Content-Type': 'application/json'},
                    callback=self.parse_post_json,
                )

    def get_cookies(self):
        site_config = get_config_by_url('https://anime-pictures.net/')
        headers = site_config.get('headers')
        return get_cookies_dict(headers.get('cookie'))

    def set_downloaded_ids(self, folder_name):
        save_path = self.custom_settings['IMAGES_STORE']
        if folder_name:
            save_path = str(Path(save_path) / self.folder_name)
        self.downloaded_ids = get_downloaded_ids(save_path)

    def get_image_item(self, post_dict, image_name):
        post_md5 = post_dict['md5']
        post_ext = post_dict['ext']
        md5_name = f'{post_md5}{post_ext}'
        if not image_name:
            image_name = f'{post_dict["id"]}-{md5_name}'
        else:
            image_name = sanitize_name(unquote(image_name))
        if self.folder_name:
            image_name = f'{self.folder_name}/{image_name}'
        image_url = f'https://images.anime-pictures.net/{post_md5[:3]}/{md5_name}'
        post_url = (f"https://anime-pictures.net/posts/{post_dict['id']}",)
        return ImageItem(image_name=image_name, image_url=image_url, referer=post_url)


class AnimePicturesTop(AnimePictures):
    name = "anime_pictures_top"
    downloaded_ids = []
    erotics = 0
    folder_name = 'day'

    def start_requests(self):
        type = getattr(self, 'type', 'day')
        self.folder_name = type
        if int(self.erotics) == 1:
            self.folder_name += '_erotic'
        # exclude downloaded posts in parent folder
        self.set_downloaded_ids('')
        url = f"{self.api_url}/top?length={type}&erotic={self.erotics}"
        yield scrapy.Request(
            url,
            headers={'Content-Type': 'application/json'},
            cookies=self.get_cookies(),
            callback=self.parse,
        )

    def parse(self, response):
        data = response.json()
        if data['success'] == False:
            self.logger.error("Failed to get top")
            return
        if data['top']:
            for post in data['top']:
                if int(post['id']) in self.downloaded_ids:
                    continue
                yield response.follow(
                    # f"{self.api_url}/posts/{post['id']}?extra=similar_pictures&lang=en",
                    f"{self.api_url}/posts/{post['id']}",
                    headers={'Content-Type': 'application/json'},
                    callback=self.parse_post_json,
                )


class AnimePicturesStars(AnimePictures):
    name = "anime_pictures_stars"
    folder_name = 'stars'

    def start_requests(self):
        post_ids = getattr(self, 'post_ids', None)
        cookies = self.get_cookies()
        self.set_downloaded_ids(self.folder_name)
        if post_ids is None:
            self.logger.error("No post ids provided")
            return
        post_ids = post_ids.split(',')
        for post_id in post_ids:
            if int(post_id) in self.downloaded_ids:
                continue
            yield scrapy.Request(
                f"{self.api_url}/posts/{post_id}",
                headers={'Content-Type': 'application/json'},
                cookies=cookies,
                callback=self.parse_post_json,
            )
