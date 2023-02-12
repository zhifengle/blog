import os
import json
import base64
import sqlite3
import win32crypt
from Crypto.Cipher import AES
# import shutil
from urllib.parse import urlparse

# pip install pypiwin32 pycryptodome


def get_encryption_key():
    local_state_path = os.path.join(os.environ["USERPROFILE"],
                                    "AppData", "Local", "Google", "Chrome",
                                    "User Data", "Local State")
    with open(local_state_path, "r", encoding="utf-8") as f:
        local_state = f.read()
        local_state = json.loads(local_state)

    key = base64.b64decode(local_state["os_crypt"]["encrypted_key"])

    key = key[5:]

    return win32crypt.CryptUnprotectData(key, None, None, None, 0)[1]


def decrypt_password(password, key):
    try:
        iv = password[3:15]
        password = password[15:]
        cipher = AES.new(key, AES.MODE_GCM, iv)
        return cipher.decrypt(password)[:-16].decode()
    except:
        try:
            return str(win32crypt.CryptUnprotectData(password, None, None, None, 0)[1])
        except:
            # not supported
            return ""

# https://gist.github.com/DakuTree/428e5b737306937628f2944fbfdc4ffc


def decrypt_chrome_cookies(host):
    cookies_path = os.path.join(
        os.environ["USERPROFILE"], 'AppData/Local/Google/Chrome/User Data/Default/Network/Cookies')
    conn = sqlite3.connect(cookies_path)
    key = get_encryption_key()
    cursor = conn.cursor()
    cursor.execute(
        f'SELECT name, value, encrypted_value FROM cookies WHERE host_key = ".{host}" or host_key = "{host}"')
    cookies = {}
    for name, value, encrypted_value in cursor.fetchall():
        # Decrypt the encrypted_value
        value = decrypt_password(encrypted_value, key)
        cookies[name] = value
    # conn.commit()
    conn.close()
    return cookies


def cookies_to_dict(cookies_str):
    from http.cookies import SimpleCookie

    cookie = SimpleCookie()
    cookie.load(cookies_str)
    cookies = {k: v.value for k, v in cookie.items()}
    print(cookies)


def dict_to_cookies(cookies):
    return "; ".join([str(x)+"="+str(y) for x, y in cookies.items()])


def get_site_cookies(url):
    host = url
    if url.startswith('http'):
        host = urlparse(url).netloc

    return dict_to_cookies(decrypt_chrome_cookies(host))


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description='get cookies')
    parser.add_argument('url', type=str, help="input url or host")
    args = parser.parse_args()
    # 不打印换行
    print(get_site_cookies(args.url), end='')
