from scrapy.cmdline import execute

import sys
import os


if __name__ == '__main__':
    cur_path = os.path.dirname(os.path.abspath(__file__))
    os.chdir(cur_path)
    sys.path.append(cur_path)

    # execute(["scrapy", "crawl", "quotes", "-a", "path=c:\\temp\\quotes", "-o", "quotes.jsonl"])
    # 提取 json
    # execute(["scrapy", "crawl", "yande_rank"])
    execute(["scrapy", "crawl", "getchu_game", "-o", "getchu_game.jsonl"])
