from urllib.parse import urlparse, urlencode, parse_qsl


def patch_url(url, **kwargs):
    url_obj = urlparse(url)
    return url_obj._replace(
        query=urlencode(dict(parse_qsl(url_obj.query), **kwargs))
    ).geturl()


if __name__ == "__main__":
    # ParseResult: netloc, fragment

    assert (
        patch_url("https://httpbin.org/get", hello="world")
        == "https://httpbin.org/get?hello=world"
    )
