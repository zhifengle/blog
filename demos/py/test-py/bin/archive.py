from pathlib import Path
from subprocess import run


def run_winrar(target_path, filelist, pw):
    winrar_path = "D:\Program Files\WinRAR\WinRAR.exe"
    run(
        [
            winrar_path,
            'a',
            '-ibck',
            '-r',
            f'-n@{filelist}',
            '-v3.8g',
            '-rr3p',
            '-ep1',
            f'-p{pw}',
            target_path,
        ]
    )


def run7z(output_path, filelist, pw):
    exe_path = "7z"
    run(
        [
            exe_path,
            'a',
            f'{output_path}.7z',
            '-mhe',
            f'-p{pw}',
            '-v4g',
            f'@{filelist}',
        ]
    )

def output_filelist(target_path, output_path, num_of_file=5000):
    filelist_arr = []
    files = [str(f) for f in target_path.iterdir()]
    for n in range(0, len(files), num_of_file):
        txt_name = output_path / f'l{n // num_of_file}.txt'
        with open(txt_name, 'w', encoding='utf-8') as f:
            f.write('\n'.join(files[n:n + num_of_file]))
            filelist_arr.append(str(txt_name))
    return filelist_arr

def archive_chunks():
    import glob

    chunk_size = 5000
    file_pattern = '*.*'

    # Find all files matching the pattern in the current directory and its subdirectories
    files = glob.glob('**/' + file_pattern, recursive=True)

    # Compress the files in batches of chunk_size
    for i in range(0, len(files), chunk_size):
        chunk = files[i:i+chunk_size]
        archive_name = f'archive_{i//chunk_size}.7z'
        cmd = ['7z', 'a', archive_name] + chunk
        run(cmd)

if __name__ == '__main__':
    target_path = r'D:\downloads\pic'
    output_path = Path(r'D:\downloads\pic')
    pw = 'testpw'
    filelist_arr = output_filelist(target_path, output_path)
    for filelist in filelist_arr:
        run7z(output_path / Path(filelist).stem, filelist, pw)
