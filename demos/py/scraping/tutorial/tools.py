from pathlib import Path
import re


def get_downloaded_ids(pic_path):
    download_ids = []
    for path in Path(pic_path).glob('**/*'):
        if path.is_dir():
            continue
        search_result = re.search(r'(\d+)', path.name)
        if search_result:
            post_id = search_result.group(1)
            download_ids.append(post_id)
    return download_ids


def save_downloaded_ids_txt(pic_path):
    ids = get_downloaded_ids(pic_path)
    cur_path = Path(pic_path)
    with open(cur_path / f'{cur_path.name}.txt', 'w') as f:
        f.write('\n'.join(ids))



if __name__ == '__main__':
    # save_downloaded_ids_txt(r'G:\downloads\pic\yande_post')
    # save_downloaded_ids_txt(r'G:\downloads\pic\konachan_post')
    # save_downloaded_ids_txt(r'G:\downloads\pic\anime_pictures')
    pass