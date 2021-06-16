def cpe(ret, ans) -> list:
    for r, a in zip(ret, ans):
        assert (r == a)