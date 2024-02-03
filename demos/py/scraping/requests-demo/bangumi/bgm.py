import re
import time
import requests
from fake_useragent import UserAgent

session = requests.Session()
proxy_url = 'http://127.0.0.1:10809'
session.proxies = {
    'http': proxy_url,
    'https': proxy_url,
}
default_headers = {
    # current cookie
    'Cookie': '',
    # login UA
    'User-Agent': ''
}

ua = UserAgent(browsers=['edge', 'chrome', 'firefox'])
url = 'https://bgm.tv/'
# session.cookies.update(cookies)

pattern = r'online: (\d+)'

first = True
while True:
    current_ua = ua.random
    headers = {
        'User-Agent': current_ua
    }
    if first:
        headers = default_headers
        first = False
    res = session.get(url, headers=headers)
    print("request ua: ", headers['User-Agent'])
    content = res.content.decode('utf-8')
    matches = re.search(pattern, content)
    if matches:
        number = int(matches.group(1))
        print("online number: ", number)
    else:
        print("no online number")
        break
    time.sleep(60 * 10)