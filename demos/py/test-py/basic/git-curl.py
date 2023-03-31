import subprocess
import json


def run_cmd():
    cmd = r'"C:\Program Files\Git\usr\bin\ls.exe" -al'
    # cmd 为字符串有参数， shell=True
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    if result.returncode == 0:
        print(result.stdout)
    else:
        print(f'An error occurred while executing "{cmd}": {result.stderr}')


def get_curl_headers(headers):
    cmd = []
    for key, value in headers.items():
        cmd.append('-H')
        cmd.append(f'{key}: {value}')
    return cmd


# https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#get-the-latest-release
def get_github_lastest(repo, headers={}, tag=None):
    repos_url = f'https://api.github.com/repos/{repo}/releases/latest'
    if tag:
        repos_url = f'https://api.github.com/repos/{repo}/releases/tags/{tag}'
    cmd = [
        'curl',
        '-sL',
    ]
    cmd.extend(
        get_curl_headers(
            {
                'Accept': 'application/vnd.github+json',
                'X-GitHub-Api-Version': '2022-11-28',
                **headers,
            }
        )
    )
    cmd.append(repos_url)
    return json.loads(subprocess.check_output(cmd))


def get_asset_by_name(assets, name):
    for item in assets:
        if name in item['name'].lower():
            return item
    return None


def download_asset(asset, headers):
    cmd = [
        'curl',
        '-sL',
    ]
    cmd.extend(get_curl_headers({**headers, 'Accept': 'application/octet-stream'}))
    cmd.extend(['-o', asset['name'], asset['url']])
    subprocess.run(cmd)


def download_file(repo, path, headers):
    url = f'https://api.github.com/repos/{repo}/contents/{path}'
    cmd = [
        'curl',
        '-sL',
    ]
    cmd.extend(get_curl_headers({**headers, 'Accept': 'application/vnd.github.v3.raw'}))
    # cmd.append('-O')
    cmd.append(url)
    res = subprocess.check_output(cmd)
    return res


def run_curl():
    cmd = 'curl -h'
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    print(result.stdout)


if __name__ == '__main__':
    # headers = {'Authorization': 'token TOKEN'}
    # res = get_github_lastest('zhifengle/xx', headers=headers)
    # asset = get_asset_by_name(res['assets'], 'linux-gnu')
    # download_asset(asset, headers)
    pass
