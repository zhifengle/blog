import atexit
import random
import re
import time
from ajax import gen_session_by_url, gen_url
from kv import KvExpiration, JsonEngine
from my_logger import get_logger

storage = KvExpiration(JsonEngine('qd-record.json'), 'QD_')

def save():
    storage.save()

atexit.register(save)

logger = get_logger('qiandao')

def qiandao(name, checkin_fn):
    err_key = name + '_err'
    is_err = storage.get(err_key)
    if is_err:
        logger.error(f"[{name}] error: {is_err}")
        return
    result = storage.get(name)
    if result:
        # storage.set_next_day(name, 2)
        logger.info(f"[{name}] has checked in today")
        return
    code = checkin_fn()
    # 特殊code 返回
    if code >= 200:
        return
    if code == 0:
        logger.info(f"[{name}] success")
        storage.set_next_day(name, 1)
    elif code == 1:
        logger.error(f"[{name}] need login")
        storage.set(err_key, 1)
    else:
        logger.error(f"[{name}] error: {code}")
        storage.set(err_key, 2)

def qd_masiro():
    url = 'https://masiro.me/admin/dailySignIn'
    session = gen_session_by_url(url)
    res = session.get(url)
    if "登录" in res.text:
        return 1
    return 0

def qd_v2ex():
    url = 'https://v2ex.com/'
    mission_url = 'https://v2ex.com/mission/daily'
    session = gen_session_by_url(url)
    res = session.get(mission_url)
    if '你是机器人么？' in res.text:
        return 1
    if '每日登录奖励已领取' in res.text:
        return 0
    # /mission\/daily\/redeem\?once=\d+/
    pattern = r'/mission\/daily\/redeem\?once=(\d+)'
    match = re.search(pattern, res.text)
    if match:
        checkin_url = 'https://v2ex.com/mission/daily/redeem?once=' + match.group(1)
        session.get(checkin_url)
        return 0
    return 1

def qd_2djgame():
    url = 'https://bbs4.2djgame.net/home/forum.php'
    session = gen_session_by_url(url)
    task_url = gen_url(url, 'home.php?mod=task&do=apply&id=1')
    res = session.get(task_url)
    if '抱歉，本期您已申請過此任務，請下期再來' in res.text:
        return 0
    elif '您需要先登錄才能繼續本操作' in res.text:
        return 1
    return 0
def qd_dsu_paulsign(url):
    session = gen_session_by_url(url)
    sign_url = gen_url(url, 'dsu_paulsign-sign.html')
    res = session.get(sign_url)
    if '您需要先登录才能继续本操作' in res.text:
        return 1
    elif '您今天已经签到过了或者签到时间还未开始' in res.text:
        return 0
    hash_pattern = r'<input\s*type="hidden"\s*name="formhash"\s*value="([^"]+?)"\s*/?>'
    form_pattern = r'<form\s*id="qiandao"\s*method="post"'
    hash_match = re.search(hash_pattern, res.text)
    if re.search(form_pattern, res.text) and hash_match:
        subimit_url = gen_url(url, 'plugin.php?id=dsu_paulsign:sign&operation=qiandao&infloat=1&inajax=1')
        lst = ['kx', 'ym', 'wl', 'nu', 'ch', 'fd', 'yl', 'shuai']
        fd = {
            'formhash': hash_match.group(1),
            'qdxq': random.choice(lst),
            'fastreply': '0',
            'qdmode': '3',
        }
        # post data
        res = session.post(subimit_url, data=fd)
        if '签到成功' in res.text:
            # refresh
            session.get(sign_url)
            return 0
        elif '未定义操作' in res.text:
            return 1

def qd_south_plus(task_id, home_url = 'https://www.south-plus.net'):
    url = f'{home_url}/plugin.php?H_name=tasks&action=ajax&actions=job&cid={task_id}'
    apply_url = f'{home_url}/plugin.php?H_name=tasks&action=ajax&actions=job2&cid={task_id}'
    session = gen_session_by_url(url)
    name = f'south-plus{task_id}'
    res = session.get(url)
    if '登录' in res.text:
        return 1
    elif '未开始' in res.text:
        logger.error(f"[south-plus] {task_id} not start")
        return 1
    elif 'success' in res.text:
        # sleep 5 seconds
        time.sleep(2)
        session.get(apply_url)
        logger.info(f"{name} success")
        if task_id == 14:
            storage.set_expiration_days(name, 7)
        elif task_id == 15:
            storage.set_expiration_days(name, 1)
        return 200
    elif '拒离上次申请' in res.text:
        logger.info(f"{name} 已申请过")
        storage.set_expiration_days(name, 1)
        return 200
    else:
        logger.error(f"[south-plus] {task_id} unknown operation")
        return 1

def qd_manhuabudangbbs():
    url = 'https://www.manhuabudangbbs.com/u.php'
    session = gen_session_by_url(url)
    res = session.get(url)
    if '已连续签到' in res.text:
        return 0
    elif '没有权限访问此页面' in res.text:
        return 1
    hash_pattern = r"verifyhash = '(.+?)'"
    match = re.search(hash_pattern, res.text)
    if match is None:
        return 2
    fd = {
        'action': 'punch',
        'verify': match.group(1),
        'step': 2
    }
    # post x-www-form-urlencoded
    res = session.post('https://www.manhuabudangbbs.com/jobcenter.php', data=fd, headers={ 'Content-Type': 'application/x-www-form-urlencoded' })
    if '你已经打卡' in res.text:
        return 0
    elif "\"flag\":'1'" not in res.text:
        return 1
    else:
        return 0


qd_fn_dict = {
    'masiro': qd_masiro,
    'v2ex': qd_v2ex,
    '2djgame': qd_2djgame,
    'acgrip': lambda: qd_dsu_paulsign('https://bbs.acgrip.com/'),
    'south-plus14': lambda: qd_south_plus(14),
    'south-plus15': lambda: qd_south_plus(15),
    'manhuabudangbbs': qd_manhuabudangbbs
}
if __name__ == '__main__':
    sites = storage.get('sites') or []
    for site in sites:
        err_key = site + '_err'
        err_res = storage.get(err_key)
        if err_res:
            if err_res == 1:
                logger.error(f"[{site}] need login")
            else:
                logger.error(f"[{site}] unknown error")
            continue
        checkin_fn = qd_fn_dict.get(site)
        if checkin_fn:
            qiandao(site, checkin_fn)