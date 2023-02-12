import scrapy

# https://docs.scrapy.org/en/latest/intro/tutorial.html
class QuotesSpider(scrapy.Spider):
    name = "quotes"
    start_urls = [
        "https://quotes.toscrape.com/page/1/",
        "https://quotes.toscrape.com/page/2/",
    ]

    # 这块不用写了。默认会调用 https://docs.scrapy.org/en/latest/intro/tutorial.html#extracting-data-in-our-spider
    # def start_requests(self):
    # urls = [
    #     "https://quotes.toscrape.com/page/1/",
    #     "https://quotes.toscrape.com/page/2/",
    # ]
    # for url in urls:
    #     yield scrapy.Request(url=url, callback=self.parse)

    # def parse(self, response, **kwargs):
    #     page = response.url.split('/')[-2]
    #     filename = f"quotes-{page}.html"
    #     with open(filename, 'wb') as f:
    #         f.write(response.body)
    #     self.log(f'saved file {filename}')

    def parse(self, response):
        for quote in response.css('div.quote'):
            yield {
                'text': quote.css('span.text::text').get(),
                'author': quote.css('small.author::text').get(),
                'tags': quote.css('div.tags a.tag::text').getall(),
            }
        next_page = response.css('li.next a::attr(href)').get()
        if next_page is not None:
            yield response.follow(next_page, callback=self.parse)
