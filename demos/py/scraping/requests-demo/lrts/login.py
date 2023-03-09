import os
import base64
import requests
import http.cookiejar as cookielib

requests.packages.urllib3.disable_warnings()

headers = {
    "user-agent": "Mozilla/5.0 (Linux; U; Android 9; tr-tr; Redmi Note 7 Build/PKQ1.180904.001) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/89.0.4389.116 Mobile Safari/537.36 XiaoMi/MiuiBrowser/12.15.2-gn",
}


def is_login(session):
    url = 'https://m.lrts.me/user'
    try:
        session.cookies.load(ignore_discard=True)
    except Exception:
        pass
    response = session.get(url, verify=False)
    if '马上登录' in response.text:
        return session, False
    else:
        return session, True


def login():
    cookie_file = '.cookie/lrts.txt'
    if not os.path.exists(".cookie"):
        os.makedirs('.cookie')
    if not os.path.exists(cookie_file):
        print("hello")
        with open(cookie_file, 'w') as f:
            f.write("")
    session = requests.session()
    session.headers = headers
    session.cookies = cookielib.LWPCookieJar(filename=cookie_file)
    session, status = is_login(session)
    if not status:
        url = 'https://m.lrts.me/ajax/logon'
        account = input('输入用户名: ')
        password = input('输入密码: ')
        form = {"account": account,
                "pwd": base64.b64encode(bytes(password, 'ascii'))}
        res_obj = session.post(url, data=form).json()
        if res_obj['phone']:
            print(f'登录{res_obj["phone"]}成功')
        else:
            print('登陆失败')
        session.cookies.save()
    return session


if __name__ == '__main__':
    login()
