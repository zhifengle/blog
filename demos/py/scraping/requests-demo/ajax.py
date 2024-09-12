import json
from pathlib import Path
from urllib.parse import urlparse, urlunparse
import requests

default_headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.56'
}

def change_url_host(original_url, new_host):
    parsed_url = urlparse(original_url)
    new_url = urlunparse(parsed_url._replace(netloc=new_host))
    
    return new_url

def gen_url(old_url, new_path):
    # Parse the old URL
    parsed_url = urlparse(old_url)
    
    # Build a new URL with the same components but with the new path
    new_url = urlunparse((parsed_url.scheme, parsed_url.netloc, new_path, '', '', ''))
    
    return new_url

def get_node_site_config():
    file = 'node-site-config.json'
    path_config_file = Path(file)
    if path_config_file.is_file():
        with open(file) as f:
            return json.load(f)
    path_config_file = Path.home() / 'node-site-config.json'
    if path_config_file.is_file():
        with open(path_config_file) as f:
            return json.load(f)
    return {}


def get_config_by_url(url):
    config = get_node_site_config()
    url_obj = requests.utils.urlparse(url)
    return config.get(url_obj.netloc, {})


def gen_session_by_url(url):
    config = get_config_by_url(url)
    headers = config.get('headers', {})
    session = requests.Session()
    session.headers.update(default_headers)
    session.headers.update(headers)
    if config.get('httpsAgent', False):
        proxy_url = 'http://127.0.0.1:10809'
        session.proxies = {
            'http': proxy_url,
            'https': proxy_url,
        }
    return session


if __name__ == '__main__':
    url = 'https://bbs.acgrip.com'
    session = gen_session_by_url(url)
    res = session.get(url)
    pass
