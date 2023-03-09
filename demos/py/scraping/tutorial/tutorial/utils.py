from pathlib import Path
import random
from urllib.parse import unquote, urlparse, urlencode, parse_qsl


def patch_url(url, **kwargs):
    url_obj = urlparse(url)
    return url_obj._replace(
        query=urlencode(dict(parse_qsl(url_obj.query), **kwargs))
    ).geturl()


def sanitize_name(name):
    return (
        name.strip()
        .replace("/", "%2F")
        .replace(":", "%3A")
        .replace("?", "%3F")
        .replace("*", "%2A")
        .replace('"', "%22")
        .replace("<", "%3C")
        .replace(">", "%3E")
        .replace("|", "%7C")
    )


def rename_quote_files(path):
    for file in Path(path).iterdir():
        new_name = sanitize_name(unquote(file.name))
        file.rename(file.parent / new_name)


def get_start_and_end(t_range):
    t_range = t_range.split("-")
    if len(t_range) == 0:
        return 0, 0
    if len(t_range) == 1:
        return int(t_range[0]), int(t_range[0])
    return int(t_range[0]), int(t_range[1])


def is_target_str(str_arr, target_str):
    return any([s in target_str for s in str_arr])


def random_useragent():
    # from fake_useragent import UserAgent
    # ua = UserAgent()
    # return ua.random

    ua_list = [
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:108.0) Gecko/20100101 Firefox/108.0',
        'Mozilla/5.0 (iPhone; CPU iPhone OS 16_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/110.0.5481.83 Mobile/15E148 Safari/604.1',
        'Mozilla/5.0 (Linux; Android 10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.5481.63 Mobile Safari/537.36',
        'Mozilla/5.0 (Macintosh; Intel Mac OS X 13_2) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.3 Safari/605.1.15',
        'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.49',
    ]
    return random.choice(ua_list)


if __name__ == "__main__":
    # ParseResult: netloc, fragment

    assert (
        patch_url("https://httpbin.org/get", hello="world")
        == "https://httpbin.org/get?hello=world"
    )
