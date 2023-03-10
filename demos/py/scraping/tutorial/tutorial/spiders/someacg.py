from pathlib import Path
import scrapy

from tutorial.items import ImageItem

class SomeAcg(scrapy.Spider):
    custom_settings = {
        'DOWNLOADER_MIDDLEWARES': {'tutorial.middlewares.ProxyMiddleware': 543},
        'ITEM_PIPELINES': {'tutorial.pipelines.MyImagesPipeline': 1},
        'DOWNLOAD_DELAY': 0.5,
        'MEDIA_ALLOW_REDIRECTS': True,
        'IMAGES_STORE': str(Path.home() / 'Downloads/pic/someacg'),
        'IMAGES_EXPIRES': 0,
    }
    name = "someacg"


    def start_requests(self):
        # 1-25
        for i in range(2, 26):
            url = f"https://www.someacg.top/api/list?page={i}"
            yield scrapy.Request(url, callback=self.parse)

    def parse(self, response):
        res = response.json()
        if res.get('status') != 200:
            return
        for post_obj in res['body']:
            item = self.get_post_item(post_obj)
            if item is not None:
                yield item


    def get_post_item(self, post_obj):
        file_name  = post_obj.get('file_name')
        if file_name is None:
            return None
        image_url = f'https://www.someacg.top/api/file/{file_name}'
        image_name = file_name
        post_item = ImageItem(
            image_name=image_name,
            image_url=image_url,
            referer=f'https://www.someacg.top/detail/{post_obj["_id"]}',
        )
        return post_item
