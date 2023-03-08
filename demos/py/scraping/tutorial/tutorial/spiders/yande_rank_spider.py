import scrapy
import re
from pathlib import PurePosixPath, Path
from urllib.parse import unquote, urlparse

from tutorial.items import ImageItem
from tutorial.utils import get_start_and_end, patch_url, sanitize_name

OUTPUT_PATH = str(Path.home() / "Downloads/pic/yande")


class YandePostItem(scrapy.Item):
    post_id = scrapy.Field()
    parent_id = scrapy.Field()
    has_children = scrapy.Field()
    tags = scrapy.Field()
    author = scrapy.Field()
    creator = scrapy.Field()
    creator_id = scrapy.Field()
    created_at = scrapy.Field()
    size = scrapy.Field()
    source = scrapy.Field()
    rating = scrapy.Field()
    score = scrapy.Field()
    image_name = scrapy.Field()
    image_url = scrapy.Field()


class YandeRank(scrapy.Spider):
    custom_settings = {
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        'DOWNLOAD_DELAY': 0.5,
        'IMAGES_STORE': OUTPUT_PATH,
    }
    name = "yande_rank"

    def start_requests(self):
        host = getattr(self, 'host', 'yande.re')
        year_range = get_start_and_end(getattr(self, 'year_range', '2007-2022'))
        month_range = get_start_and_end(getattr(self, 'month_range', '1-12'))

        for year in range(year_range[0], year_range[1] + 1):
            for i in range(month_range[0], month_range[1] + 1):
                p = Path(rf"{OUTPUT_PATH}/{year}-{i:02d}")
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
        image_url = response.css("a#highres::attr(href)").get()
        image_name = unquote(PurePosixPath(urlparse(image_url).path).name)
        if response.meta['year'] and response.meta['month']:
            image_name = f"{response.meta['year']}-{response.meta['month']:02d}/{sanitize_name(image_name)}"
        yield ImageItem(image_name=image_name, image_url=image_url)


class YandePost(scrapy.Spider):
    custom_settings = {
        'DEFAULT_REQUEST_HEADERS': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:108.0) Gecko/20100101 Firefox/108.0'
            # 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.83 Mobile/15E148 Safari/604.1'
            # 'Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.5481.63 Mobile Safari/537.36'
            # 'Mozilla/5.0 (Macintosh; Intel Mac OS X 13_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15'
            # 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.49'
        },
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {
            'tutorial.pipelines.YandeImagesPipeline': 1,
            'tutorial.pipelines.YandePostSqlitePipeline': 3,
        },
        'DOWNLOAD_DELAY': 0.5,
        # 'IMAGES_STORE': str(Path.home() / 'Downloads/pic/yande_post'),
        'IMAGES_STORE': str(Path('G:\\') / 'Downloads/pic/yande_post'),
        'IMAGES_EXPIRES': 0,
        'SQLITE_DB_PATH': str(Path('G:\\') / 'Downloads/pic/yande_post/posts.sqlite'),
    }
    name = "yande_post"
    folder = 'yande.re'

    def start_requests(self):
        host = getattr(self, 'host', 'yande.re')
        tags = getattr(self, 'tags', '')
        url = getattr(self, 'url', '')
        pic_path = getattr(self, 'pic_path', None)
        if pic_path:
            self.logger.info("Getting failed post ids")
            failed_post_ids = self.get_failed_post_ids(pic_path)
            for post_id in failed_post_ids:
                yield scrapy.Request(
                    f"https://{host}/post/show/{post_id}",
                    callback=self.parse_post,
                )
            return
        if '/post/show' in url:
            yield scrapy.Request(url, callback=self.parse_post)
            return
        if tags:
            url = f"https://{host}/post?tags={tags}"
            # 去除部分筛选用的 tag
            # folder = tags.split('score:>=')[0].replace('-photoshop', '').strip()
            # self.folder = f'{host}_{folder}'
            self.folder = host
        elif url == '':
            self.folder = f'{host}_popular_recent'
            url = f'https://{host}/post/popular_recent'
            # url = f'https://{host}/post'
        yield scrapy.Request(url, callback=self.parse)

    def parse(self, response):
        post_urls = response.css("a.thumb::attr(href)").getall()
        for url in post_urls:
            yield response.follow(url, callback=self.parse_post)
        next_page = response.css("a.next_page::attr(href)").get()
        if next_page is not None:
            yield response.follow(next_page, callback=self.parse)

    def parse_image(self, response):
        img_urls = response.css("#post-list-posts li>a.directlink::attr(href)").getall()
        for url in img_urls:
            img_name = unquote(PurePosixPath(urlparse(url).path).name)
            img_name = f"{sanitize_name(self.folder)}/{sanitize_name(img_name)}"
            yield ImageItem(image_name=img_name, image_url=url)
        next_page = response.css("a.next_page::attr(href)").get()
        if next_page is not None:
            yield response.follow(next_page, callback=self.parse)

    def parse_post(self, response):
        image_url = response.css("a#highres::attr(href)").get()
        image_name = unquote(PurePosixPath(urlparse(image_url).path).name)
        pic_path = getattr(self, 'pic_path', None)
        if pic_path:
            image_name = f"{Path(pic_path).name}/{sanitize_name(image_name)}"
        else:
            image_name = f"{sanitize_name(self.folder)}/{sanitize_name(image_name)}"
        # yield ImageItem(
        #     image_name=image_name, image_url=image_url, referer=response.url
        # )

        tags = response.css("#tag-sidebar li>a[href^='/post?tags']::attr(href)").re(
            r'/post\?tags=(.+)$'
        )
        post_id = re.match(r'.*/post/show/(\d+)', response.url).group(1)
        post_item = YandePostItem(
            post_id=post_id,
            tags=unquote(','.join(tags)),
            image_name=image_name,
            image_url=image_url,
        )
        for li in response.css("#stats > ul > li"):
            txt = li.css("::text").get()
            if 'Posted:' in txt:
                post_item['creator'] = li.css("a[href^='/user/show']::text").get()
                post_item['creator_id'] = li.css(
                    "a[href^='/user/show']::attr(href)"
                ).re_first(r'/user/show/(\d+)')
                post_item['created_at'] = li.css(
                    "a[href^='/post?tags=date']::attr(href)"
                ).re_first(r'date%3A(\d+-\d+-\d+)')
                continue
            if 'Rating:' in txt:
                post_item['rating'] = txt.split('Rating:')[1].strip()
                continue
            if 'Size:' in txt:
                post_item['size'] = txt.split('Size:')[1].strip()
                continue
            if 'Score:' in txt:
                post_item['score'] = li.css("span::text").get()
                continue
            if 'Source:' in txt:
                post_item['source'] = li.css("a::attr(href)").get()
                continue
        p_txt = response.css(
            "#post-view > .status-notice>a[href^='/post?tags=parent']::text"
        ).get()
        post_item['has_children'] = False
        if p_txt is None:
            pass
        elif 'child posts' in p_txt:
            post_item['has_children'] = True
        post_links = response.css("#post-view > .status-notice>a[href^='/post/show']")
        if len(post_links) == 1 and post_links.css('::text').get() == 'parent post':
            post_item['parent_id'] = post_links.re_first(r'/post/show/(\d+)')
        yield post_item

        if post_item['has_children']:
            child_post_urls = response.css(
                "#post-view > .status-notice>a[href^='/post/show']::attr(href)"
            ).getall()
            for child_post_url in child_post_urls:
                yield response.follow(child_post_url, callback=self.parse_post)

    def get_failed_post_ids(self, pic_path):
        import os

        failed_post_ids = []
        for file in os.listdir(pic_path):
            if os.path.getsize(os.path.join(pic_path, file)) == 0:
                search_result = re.search(r'yande.re (\d+)', file)
                if search_result:
                    post_id = search_result.group(1)
                    failed_post_ids.append(post_id)
        return failed_post_ids


class YandePostJson(scrapy.Spider):
    custom_settings = {
        'DEFAULT_REQUEST_HEADERS': {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:108.0) Gecko/20100101 Firefox/108.0'
            # 'Mozilla/5.0 (iPhone; CPU iPhone OS 16_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.83 Mobile/15E148 Safari/604.1'
            # 'Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.5481.63 Mobile Safari/537.36'
            # 'Mozilla/5.0 (Macintosh; Intel Mac OS X 13_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15'
            # 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.49'
        },
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {
            'tutorial.pipelines.YandeImagesPipeline': 1,
            'tutorial.pipelines.YandePostSqlitePipeline': 3,
        },
        'DOWNLOAD_DELAY': 0.5,
        # 'IMAGES_STORE': str(Path.home() / 'Downloads/pic/yande_post'),
        'IMAGES_STORE': str(Path('G:\\') / 'Downloads/pic/yande_post'),
        'IMAGES_EXPIRES': 0,
        'SQLITE_DB_PATH': str(Path('G:\\') / 'Downloads/pic/yande_post/posts_json.sqlite'),
    }
    name = "yande_post_json"
    page_size = 100
    folder = 'yande.re'

    def get_default_url(self):
        host = getattr(self, 'host', 'yande.re')
        return f"https://{host}/post.json?page=1&limit={self.page_size}"

    def start_requests(self):
        host = getattr(self, 'host', 'yande.re')
        url = getattr(self, 'url', f"https://{host}/post.json?page=1&limit={self.page_size}")
        tags = getattr(self, 'tags', '')
        folder = getattr(self, 'folder', None)
        if tags:
            url = patch_url(url, tags=tags)
            self.folder = host if folder is None else folder
        yield scrapy.Request(url, callback=self.parse)

    def parse(self, response):
        posts_arr = response.json()
        if len(posts_arr) == 0:
            return
        for post_obj in posts_arr:
            item = self.get_post_item(post_obj)
            if item is not None:
                if item['parent_id'] is not None and response.url.find('parent') == -1:
                    url = patch_url(self.get_default_url(), tags=f"parent:{item['parent_id']}")
                    yield scrapy.Request(url, callback=self.parse_parent_list)
                else:
                    yield item
        if len(posts_arr) < self.page_size:
            return
        cur_page = re.match(r'.*page=(\d+).*', response.url).group(1)
        if cur_page is None:
            cur_page = 1
        yield response.follow(
            patch_url(response.url, page=int(cur_page) + 1), callback=self.parse
        )

    def parse_parent_list(self, response):
        posts_arr = response.json()
        if len(posts_arr) == 0:
            return
        for post_obj in posts_arr:
            yield self.get_post_item(post_obj)

    def get_post_item(self, post_obj):
        status = post_obj.get('status')
        if status == 'deleted':
            return
        image_url = post_obj.get('file_url')
        jpeg_url = post_obj.get('jpeg_url')
        if jpeg_url is not None:
            image_url = jpeg_url
        if image_url is None:
            return
        image_name = unquote(PurePosixPath(urlparse(image_url).path).name)
        pic_path = getattr(self, 'pic_path', None)
        if pic_path:
            image_name = f"{Path(pic_path).name}/{sanitize_name(image_name)}"
        else:
            image_name = f"{sanitize_name(self.folder)}/{sanitize_name(image_name)}"
        post_item = YandePostItem(
            post_id=post_obj['id'],
            tags=post_obj.get('tags', ''),
            image_name=image_name,
            image_url=image_url,
        )
        opt_keys = [
            'creator_id',
            'created_at',
            'rating',
            'score',
            'source',
            'has_children',
            'parent_id',
        ]
        for key in opt_keys:
            if key in post_obj:
                post_item[key] = post_obj[key]
        if 'width' in post_obj and 'height' in post_obj:
            post_item['size'] = f"{post_obj['width']}x{post_obj['height']}"
        if 'author' in post_obj:
            post_item['creator'] = post_obj['author']
        return post_item
