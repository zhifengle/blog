from scrapy.cmdline import execute

import sys
import os

if __name__ == '__main__':
    cur_path = os.path.dirname(os.path.abspath(__file__))
    os.chdir(cur_path)
    sys.path.append(cur_path)

    # execute(["scrapy", "crawl", "quotes", "-a", "path=c:\\temp\\quotes", "-o", "quotes.jsonl"])
    # execute(["scrapy", "crawl", "yande_rank"])
    execute(
        [
            "scrapy",
            "crawl",
            "getchu_product",
            '-a',
            "genre=pc_soft",
            # "-o",
            # "getchu_product.jsonl",
        ]
    )
