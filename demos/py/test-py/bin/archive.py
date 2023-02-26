import os
from pathlib import Path
from subprocess import check_output


def run_winrar(target_path, output_path, pw):
    winrar_path = "D:\Program Files\WinRAR\WinRAR.exe"
    output = check_output(
        [
            winrar_path,
            'a',
            '-ibck',
            '-r',
            '-v3.8g',
            '-rr3p',
            '-ep1',
            f'-p{pw}',
            output_path,
            target_path,
        ]
    ).decode("utf-8")


def run7z(output_path, filelist, pw):
    exe_path = "7z"
    output = check_output(
        [
            exe_path,
            'a',
            f'{output_path}.7z',
            '-mhe',
            f'-p{pw}',
            '-v4g',
            f'@{filelist}',
        ]
    ).decode("utf-8")


if __name__ == '__main__':
    target_path = r'D:\downloads\pic'
    output_path = Path(r'D:\downloads\pic')
    pw = 'testpw'
    filelist = output_path / 'filelist.txt'
    files = []
    counter = 0
    num_of_file = 10000
    for file in Path(target_path).iterdir():
        if file.is_file():
            counter += 1
            files.append(file)
        if counter % num_of_file == 0:
            with open(filelist, 'w', encoding='utf-8') as f:
                f.write('\n'.join([str(f) for f in files]))
            files = []
            run7z(output_path / f'chunk_{counter // num_of_file}', filelist, pw)
    if len(files) > 0:
        with open(filelist, 'w', encoding='utf-8') as f:
            f.write('\n'.join([str(f) for f in files]))
        files = []
        run7z(output_path / f'chunk_{counter // num_of_file}', filelist, pw)
