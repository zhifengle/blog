#
import os
from pathlib import Path

# 包含当前文件夹的目录
# resolve 生成绝对路径
this_dir = Path(__file__).resolve().parent

# 得到上一级的目录
other_dir = Path('.').resolve().parent

# rglob 是递归
for p in (other_dir / 'test-py').glob("*.py"):
    print(f'{p.name}')


# files = [f for f in os.listdir(workDir)]
# zipFiles = filter(lambda name : fnmatch(name, "*.zip"), files)
# rarFiles = filter(lambda name : fnmatch(name, "*.rar"), files)

if __name__ == '__main__':
    print(Path(os.path.expanduser('~/node-site-config.json')).read_text())
    with open('path_demo.py', 'r', encoding="utf-8") as f:
        output = f.read()
        print(output)
