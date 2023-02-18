# Define here the models for your scraped items
#
# See documentation in:
# https://docs.scrapy.org/en/latest/topics/items.html

import scrapy


class User(scrapy.Item):
    name = scrapy.Field()
    nickname = scrapy.Field()
    uid = scrapy.Field()
    joindate = scrapy.Field()

class ImageItem(scrapy.Item):
    image_url = scrapy.Field()
    image_name = scrapy.Field()