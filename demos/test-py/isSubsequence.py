def is_subsequence(s, t):
    if not len(s):
        return True
    ps = 0
    pt = 0
    while pt < len(t):
        if s[ps] == t[pt]:
            ps += 1
            pt += 1
            if ps >= len(s):
                return True
        else:
            pt += 1
    return False
