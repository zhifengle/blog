import json
import os
from pathlib import Path
import requests


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
    return config.get(url_obj.netloc)

def get_session_by_url(url):
    config = get_config_by_url(url)
    use_proxy = config.get('httpsAgent', False)
    headers = config.get('headers', {})
    session = requests.Session()
    session.headers['User-Agent'] = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.56'
    for key, value in headers.items():
        session.headers[key] = value
    if use_proxy:
        proxy_url = '127.0.0.1:10809'
        session.proxies = {
            'http': proxy_url,
            'https': proxy_url,
        }
    return session

if __name__ == '__main__':
    session = get_session_by_url('https://bbs.acgrip.com')
    res = session.get('https://bbs.acgrip.com/forum.php')
