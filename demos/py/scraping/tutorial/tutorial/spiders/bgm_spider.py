# https://github.com/wattlebird/Bangumi_Spider/blob/master/bgm/spiders/spider.py
import scrapy
import re
import datetime

from tutorial.items import User

mpa = dict([(i, None) for i in range(32)])

ptn = re.compile(
    r"""(((1\d{3})|(20\d{2}))([./\-\\]|年)( #
                     (((0?[13578])|(1[0-2]))(月|[./\-\\])(([1-2][0-9])|(3[01])|(0?[1-9])))日?| #
                     (((0?[469])|(11))(月|[./\-\\])(([1-2][0-9])|(30)|(0?[1-9])))日?| #
                     (0?2(月|[./\-\\])(([1-2][0-9])|(0?[1-9])))日? #
                    ))| #
                    (((1\d{3})|(20\d{2}))([./\-\\]|年)(1[0-2]|0?[1-9])月?)| #
                    (((1\d{3})|(20\d{2}))([./\-\\]|年)?)""",
    re.X,
)


def parsedate(bgmdate):
    mt = ptn.search(bgmdate)
    if mt.groups()[0] is not None:  # year, month and date are all matched.
        year = int(mt.groups()[1])
        if mt.groups()[6] is not None:  # 1,3,5,...
            month = int(mt.groups()[7])
            days = int(mt.groups()[11])
        elif mt.groups()[15] is not None:  # 4,6,...
            month = int(mt.groups()[16])
            days = int(mt.groups()[20])
        else:
            month = 2
            days = int(mt.groups()[26])
        dt = datetime.date(year, month, days)
    elif mt.groups()[29] is not None:
        year = int(mt.groups()[30])
        month = int(mt.groups()[34])
        dt = datetime.date(year, month, 1)
    elif mt.groups()[35] is not None:
        year = int(mt.groups()[36])
        dt = datetime.date(year, 1, 1)
    else:
        return None
    return dt


class UserSpider(scrapy.Spider):
    name = 'user'

    def __init__(self, *args, **kwargs):
        super(UserSpider, self).__init__(*args, **kwargs)
        if not hasattr(self, 'id_max'):
            self.id_max = 2
            # self.id_max = 400000
        if not hasattr(self, 'id_min'):
            self.id_min = 1
        self.start_urls = [
            "http://mirror.bgm.rincat.ch/user/" + str(i)
            for i in range(int(self.id_min), int(self.id_max))
        ]

    def parse(self, response):
        if len(response.xpath(".//*[@id='headerProfile']")) == 0:
            return
        user = response.xpath(
            ".//*[@id='headerProfile']/div/div/h1/div[3]/small/text()"
        ).extract()[0][1:]
        nickname = (
            response.xpath(".//*[@class='headerContainer']//*[@class='inner']/a/text()")
            .extract()[0]
            .translate(mpa)
        )

        # Is blocked?
        if len(response.xpath("//ul[@class='timeline']/li")) == 0:
            return

        if not 'redirect_urls' in response.meta:
            uid = int(user)
        else:
            uid = int(response.meta['redirect_urls'][0].split('/')[-1])
        date = (
            response.xpath(
                ".//*[@id='user_home']/div[@class='user_box clearit']/ul/li[1]/span[2]/text()"
            )
            .extract()[0]
            .split(' ')[0]
        )
        date = parsedate(date)
        # 每个属性设置的方式
        # user =  User()
        # user['name'] = user

        yield User(name=user, nickname=nickname, uid=uid, joindate=date)

    # def get_request_with_proxy(self, url, callback=None, **kwargs):
    #     req = scrapy.Request(url=url, callback=callback, **kwargs)
    #     req.meta['proxy'] = "http://127.0.0.1:10809"
    #     return req