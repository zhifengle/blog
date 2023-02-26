from pathlib import Path
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


if __name__ == "__main__":
    # ParseResult: netloc, fragment

    assert (
        patch_url("https://httpbin.org/get", hello="world")
        == "https://httpbin.org/get?hello=world"
    )
