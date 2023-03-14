import os
from pathlib import Path


def get_txt_files():
    """Get the text files"""
    return [f for f in os.listdir('.') if os.path.isfile(f) and f.endswith('.txt')]


# use pathlib
def get_txt_files2():
    """Get the text files"""
    return [f for f in Path('.').iterdir() if f.is_file() and f.suffix == '.txt']

def write_file_list(start_dir, ext_list):
    with open('file_list.txt', 'w') as f:
        for dirpath, dirnames, filenames in os.walk(start_dir):
            for filename in filenames:
                ext = os.path.splitext(filename)[1].lower()
                if ext in ext_list:
                    # f.write(os.path.join(dirpath, filename) + '\n')
                    f.write(filename + '\n')

def test_path():
    # print(Path(os.path.expanduser('~/node-site-config.json')).read_text())
    # with open('path_demo.py', 'r', encoding="utf-8") as f:
    #     output = f.read()
    #     print(output)

    this_dir = Path(__file__).resolve().parent

    # other_dir = Path('.').resolve().parent

    # glob py file
    for p in (this_dir).glob("*.py"):
        print(f'{p.name}')
    # home dir
    home_dir = Path.home() # or Path(os.path.expanduser('~'))
    config_file = home_dir / 'node-site-config.json'
    # check if file exists
    if config_file.exists():
        print(config_file.read_text())

if __name__ == '__main__':
    # ext png jpg jpeg
    # write_file_list(r'C:\Users\alan\Downloads\pic\anime_pictures', ['.png', '.jpg', '.jpeg'])
    pass