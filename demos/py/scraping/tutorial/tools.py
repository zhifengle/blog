from pathlib import Path
import re


def get_downloaded_ids(pic_path):
    download_ids = set()
    for path in Path(pic_path).glob('**/*'):
        if path.is_dir():
            continue
        search_result = re.search(r'(\d+)', path.name)
        if search_result:
            post_id = search_result.group(1)
            download_ids.add(post_id)
    return download_ids


def save_downloaded_ids_txt(pic_path):
    cur_path = Path(pic_path)
    filelist_txt = cur_path / f'{cur_path.name}.txt'
    
    downloaded_ids = set()
    if filelist_txt.exists():
        with open(filelist_txt, 'r') as f:
            downloaded_ids.update([line.strip() for line in f.readlines()])
    
    downloaded_ids = downloaded_ids.union(get_downloaded_ids(pic_path))
    with open(cur_path / f'{cur_path.name}.txt', 'w') as f:
        f.write('\n'.join(list(downloaded_ids)))



if __name__ == '__main__':
    # save_downloaded_ids_txt(r'G:\downloads\pic\yande_post')
    # save_downloaded_ids_txt(r'G:\downloads\pic\konachan_post')
    # save_downloaded_ids_txt(r'G:\downloads\pic\anime_pictures')
    pass