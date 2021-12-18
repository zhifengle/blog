if __name__ == '__main__':
    from subprocess import check_output
    o = 'arg1'
    # bytes 需要 decode
    url = check_output(['node', './path_decode.js', o]).decode("utf-8")
    print(url)
