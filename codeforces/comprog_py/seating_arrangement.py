for i in range(int(input())):
    n,x,s = map(int,input().split())
    v = input()

    av = [x,0,0]
    ans = 0
    for a in v:
        idx = -1
        if a == 'I':
            idx = 0
        if a == 'E':
            idx = 1
        if a == 'A':
            idx = 2

        if idx == 2:
            if av[1] > 0:
                av[1] -= 1
                ans += 1
                av[2] += 1
            else:
                if av[0] > 0:
                    av[0] -= 1
                    av[1] += s-1
                    ans += 1
        elif idx == 0:
            if av[0] > 0:
                av[0] -= 1
                av[1] += s-1
                ans += 1
        else:
            if av[1] > 0:
                av[1] -= 1
                ans += 1
            elif av[2] > 0 and av[0] > 0:
                av[0] -= 1
                av[1] += s-1
                av[2] -= 1
                ans += 1
    print(ans)


