from urllib.parse import urlparse, urlencode, parse_qsl


def patch_url(url, **kwargs):
    url_obj = urlparse(url)
    return url_obj._replace(
        query=urlencode(dict(parse_qsl(url_obj.query), **kwargs))
    ).geturl()

def sanitize_name(name):
    return (
        name.strip()
        .replace("/", "_")
        .replace(":", "_")
        .replace("?", "_")
        .replace("*", "_")
        .replace('"', "_")
        .replace("<", "_")
        .replace(">", "_")
        .replace("|", "_")
    )

def get_start_and_end(t_range):
    t_range = t_range.split("-")
    if len(t_range) == 0:
        return 0, 0
    if len(t_range) == 1:
        return int(t_range[0]), int(t_range[0])
    return int(t_range[0]), int(t_range[1])

if __name__ == "__main__":
    # ParseResult: netloc, fragment

    assert (
        patch_url("https://httpbin.org/get", hello="world")
        == "https://httpbin.org/get?hello=world"
    )
