import os
from pathlib import Path


def get_txt_files():
    """Get the text files"""
    return [f for f in os.listdir('.') if os.path.isfile(f) and f.endswith('.txt')]


# use pathlib
def get_txt_files2():
    """Get the text files"""
    return [f for f in Path('.').iterdir() if f.is_file() and f.suffix == '.txt']

if __name__ == '__main__':
    # print(Path(os.path.expanduser('~/node-site-config.json')).read_text())
    # with open('path_demo.py', 'r', encoding="utf-8") as f:
    #     output = f.read()
    #     print(output)

    this_dir = Path(__file__).resolve().parent

    # other_dir = Path('.').resolve().parent

    # glob py file
    for p in (this_dir).glob("*.py"):
        print(f'{p.name}')