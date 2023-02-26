# Define your item pipelines here
#
# Don't forget to add your pipeline to the ITEM_PIPELINES setting
# See: https://docs.scrapy.org/en/latest/topics/item-pipeline.html


# useful for handling different item types with a single interface
import json
from pathlib import PurePosixPath
import sqlite3
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
        yield Request(item['image_url'], headers={'Referer': item.get('referer')})

    def file_path(self, request, response=None, info=None, *, item=None):
        return item['image_name']


class GetchuImagesPipeline(ImagesPipeline):
    def get_media_requests(self, item, info):
        yield Request(item['cover_url'], headers={'Referer': item['url']})

    def file_path(self, request, response=None, info=None, *, item=None):
        date_arr = item['release_date'].split('/')
        name = PurePosixPath(urlparse(item['cover_url']).path).name
        if len(date_arr) == 3:
            ym = f"{date_arr[0]}-{int(date_arr[1]):02d}"
            return f"{ym}/{name}"
        return name


class GetchuSqlitePipeline:
    product_game_info_dict = {
        'シナリオ': 'scenario',
        'イラスト': 'illustrator',
        '原画': 'illustrator',
        '声優': 'voice_actor',
        # 'システム': 'system',
        # 'ボイス': 'voice',
        # '動作環境': 'environment',
        # 'サイズ': 'size',
        '販売日': 'release_date',
        '価格': 'price',
        '定価': 'price',
        'ブランド': 'brand',
        # 'シリーズ': 'series',
        # 'メーカー': 'maker',
        # 'レーティング': 'rating',
        'カテゴリ': 'category',
        '音楽': 'music',
        'アーティスト': 'artist',
        'ジャンル': 'genre',
    }

    def open_spider(self, spider):
        database_name = spider.settings.get('SQLITE_DB_PATH')
        self.conn = sqlite3.connect(database_name)
        self.cursor = self.conn.cursor()
        self.cursor.execute(
            '''
            CREATE TABLE IF NOT EXISTS products(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT,
                url TEXT,
                cover_url TEXT,
                release_date TEXT,
                brand TEXT,
                genre INTEGER)
                '''
        )
        self.cursor.execute(
            '''
            CREATE TABLE IF NOT EXISTS product_game_info(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                product_id INTEGER, 
                title TEXT,
                url TEXT,
                cover_url TEXT,
                release_date TEXT,
                brand TEXT,

                illustrator TEXT,
                scenario TEXT,
                price TEXT,
                genre TEXT,
                music TEXT,
                artist TEXT,
                voice_actor TEXT,
                category TEXT,
                description TEXT,
                raw_info TEXT)
            '''
        )

    def close_spider(self, spider):
        self.conn.commit()
        self.conn.close()

    def process_item(self, item, spider):
        # check item in db by url
        res = self.cursor.execute(
            'SELECT id FROM products WHERE url = ?', (item['url'],)
        ).fetchone()
        if res is not None:
            return item

        if item.get('title'):
            self.cursor.execute(
                'INSERT INTO products(title, url, cover_url, release_date, brand, genre) VALUES(?, ?, ?, ?, ?, ?)',
                (
                    item['title'],
                    item['url'],
                    item['cover_url'],
                    item['release_date'],
                    item['brand'],
                    item['genre'],
                ),
            )
            product_id = self.cursor.lastrowid
            # genre 0 为游戏
            if item['genre'] == 0:
                self.cursor.execute(
                    'INSERT INTO product_game_info(product_id, raw_info) VALUES(?, ?)',
                    (product_id, item['raw_info']),
                )
                game_info_id = self.cursor.lastrowid
                game_info = json.loads(item['raw_info'])
                item_save_keys = [
                    'title',
                    'url',
                    'cover_url',
                    'release_date',
                    'brand',
                    'description',
                ]
                for k in item_save_keys:
                    if k in item:
                        self.cursor.execute(
                            f'UPDATE product_game_info SET {k} = ? WHERE id = ?',
                            (item[k], game_info_id),
                        )
                for k, v in game_info.items():
                    if k in self.product_game_info_dict and v not in item_save_keys:
                        self.cursor.execute(
                            f'UPDATE product_game_info SET {self.product_game_info_dict[k]} = ? WHERE id = ?',
                            (v, game_info_id),
                        )
        return item


class NyaaSqlitePipeline:
    def open_spider(self, spider):
        database_name = spider.settings.get('SQLITE_DB_PATH')
        self.conn = sqlite3.connect(database_name)
        self.cursor = self.conn.cursor()
        self.cursor.execute(
            '''
            CREATE TABLE IF NOT EXISTS nyaa(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT,
                url TEXT,
                torrent_url TEXT,
                category TEXT,
                magnet TEXT,
                size TEXT,
                date TEXT,
                seeders INTEGER,
                leechers INTEGER,
                completed INTEGER
            )
                '''
        )

    def close_spider(self, spider):
        self.conn.commit()
        self.conn.close()

    def process_item(self, item, spider):
        # check item in db by url
        res = self.cursor.execute(
            'SELECT id FROM nyaa WHERE url = ?', (item['url'],)
        ).fetchone()
        if res is not None:
            update_keys = ['seeders', 'leechers', 'completed']
            for k in update_keys:
                if k in item:
                    self.cursor.execute(
                        f'UPDATE nyaa SET {k} = ? WHERE id = ?',
                        (item[k], res[0]),
                    )
            return item

        if item.get('title'):
            self.cursor.execute(
                'INSERT INTO nyaa(title, url, torrent_url, category, magnet, size, date, seeders, leechers, completed) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)',
                (
                    item['title'],
                    item['url'],
                    item['torrent_url'],
                    item['category'],
                    item['magnet'],
                    item['size'],
                    item['date'],
                    item['seeders'],
                    item['leechers'],
                    item['completed'],
                ),
            )
        return item


class YandePostSqlitePipeline:
    def open_spider(self, spider):
        database_name = spider.settings.get('SQLITE_DB_PATH')
        self.conn = sqlite3.connect(database_name)
        self.cursor = self.conn.cursor()
        self.cursor.execute(
            '''
            CREATE TABLE IF NOT EXISTS yande_post(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                post_id INTEGER,
                parent_id INTEGER,
                has_children INTEGER,
                tags TEXT,
                author TEXT,
                creator TEXT,
                creator_id INTEGER,
                created_at TEXT,
                size TEXT,
                source TEXT,
                rating TEXT,
                score INTEGER,
                image_url TEXT,
                image_name TEXT)
                '''
        )
        self.counter = 0

    def close_spider(self, spider):
        self.conn.commit()
        self.conn.close()

    def process_item(self, item, spider):
        # check item in db by url
        rowid = self.cursor.execute(
            'SELECT id FROM yande_post WHERE post_id = ?', (item['post_id'],)
        ).fetchone()

        if rowid is None and item.get('post_id'):
            self.cursor.execute(
                'INSERT INTO yande_post(post_id, image_url, image_name) VALUES(?, ?, ?)',
                (
                    item['post_id'],
                    item['image_url'],
                    item['image_name'],
                ),
            )
            row_id = self.cursor.lastrowid

            opt_keys = [
                'parent_id',
                'has_children',
                'tags',
                'author',
                'creator',
                'creator_id',
                'created_at',
                'size',
                'source',
                'rating',
                'score',
            ]
            for k in opt_keys:
                if k in item:
                    self.cursor.execute(
                        f'UPDATE yande_post SET {k} = ? WHERE id = ?',
                        (item[k], row_id),
                    )
        elif rowid is not None:
            opt_keys = [
                'image_url',
                'image_name',
                'parent_id',
                'has_children',
                'tags',
                'source',
                'score',
            ]
            for k in opt_keys:
                if k in item:
                    self.cursor.execute(
                        f'UPDATE yande_post SET {k} = ? WHERE id = ?',
                        (item[k], rowid[0]),
                    )
        self.counter += 1
        if self.counter % 100 == 0:
            self.conn.commit()
        return item


class YandeImagesPipeline(ImagesPipeline):
    def get_media_requests(self, item, info):
        # score = int(item.get('score', 0))
        # if score < 100:
        #     return
        # tags = item.get('tags', '')
        # allowed_tags = ['school_uniform']
        # if not any([tag in tags for tag in allowed_tags]):
        #     return
        yield Request(item['image_url'], headers={'Referer': item.get('referer')})

    def file_path(self, request, response=None, info=None, *, item=None):
        return item['image_name']
