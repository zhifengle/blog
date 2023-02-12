import argparse


def get_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('-u', '--url', nargs=1, type=str, help='page url')
    parser.add_argument('start', type=int, help='start number')
    parser.add_argument('end', type=int, help='end number')
    parser.add_argument('-p', '--path', nargs=1, type=str, help='save path')
    return parser.parse_args()


def init():
    args = get_args()

    print('my args: ', args)
    if args.url is None:
        print('url 不能为空')
        return
    pageurl = args.url[0]
    if args.path is not None:
        file_path = args.path[0]


if __name__ == '__main__':
    # python cli_demo2.py -u 123 -p 123 2 3
    pageurl = 'https://xx.com'
    file_path = r'D:\test\python\d'
    init()
