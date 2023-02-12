# Define your item pipelines here
#
# Don't forget to add your pipeline to the ITEM_PIPELINES setting
# See: https://docs.scrapy.org/en/latest/topics/item-pipeline.html


# useful for handling different item types with a single interface
from pathlib import PurePosixPath
from urllib.parse import urlparse

from scrapy.pipelines.images import ImagesPipeline
from scrapy import Request

from .items import User


class TutorialPipeline:
    def process_item(self, item, spider):
        return item


class BgmPipeline:
    def process_item(self, item, spider):
        if isinstance(item, User):
            print('================= pipe ============')
            pass
        return item

class MyImagesPipeline(ImagesPipeline):
    def get_media_requests(self, item, info):
        yield Request(item['image_url'])
    def file_path(self, request, response=None, info=None, *, item=None):
        return item['image_name']