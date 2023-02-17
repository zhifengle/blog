from scrapy.cmdline import execute

import sys
import os

if __name__ == '__main__':
    cur_path = os.path.dirname(os.path.abspath(__file__))
    os.chdir(cur_path)
    sys.path.append(cur_path)
    # scrapy crawl yande_rank -a host=konachan.com
    # scrapy crawl yande_product -a url="https://yande.re/post?tags=school_uniform"

    # execute(["scrapy", "crawl", "quotes", "-a", "path=c:\\temp\\quotes", "-o", "quotes.jsonl"])
    execute(["scrapy", "crawl", "yande_post"])