from scrapy.cmdline import execute

import sys
import os


if __name__ == '__main__':
    cur_path = os.path.dirname(os.path.abspath(__file__))
    os.chdir(cur_path)
    sys.path.append(cur_path)

    # execute(["scrapy", "crawl", "quotes"])
    # 提取 json
    execute(["scrapy", "crawl", "quotes", "-O", "quotes.json"])
