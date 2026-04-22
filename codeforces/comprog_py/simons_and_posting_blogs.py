for i in range(int(input())):
    n = int(input())
    lists = [list(map(int, input().split())) for _ in range(n)]

    out = []
    seen = set()
    for a in lists:
        a.pop(0)
    while True:
        for a in lists:
            a = reversed(a)
        lists.sort(reverse=True)
        for a in lists:
            a = reversed(a)

        # push to out
        for a in reversed(lists[-1]):
            seen.add(a)
            out.append(a)
        lists.pop(-1)
        for j in range(len(lists)-1,-1,-1):
            a = lists[j]
            for i in range(len(a)-1,-1,-1):
                if a[i] in out:
                    a.pop(i)
            if len(a) == 0:
                lists.pop(j)
    print(lists)


    out = []

    o = []
    has = set()
    for a in lists:
        for b in a:
            o.insert(0, b)
    idx = 0
    while idx < len(o):
        a = o[idx]
        if a in has:
            o.pop(idx)
        else:
            idx += 1
            has.add(a)
    print(' '.join(map(str, o)))

