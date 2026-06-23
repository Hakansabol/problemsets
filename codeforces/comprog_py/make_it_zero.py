for _ in range(int(input())):
    n = int(input())
    ans = 0
    while n > 1:
        if n % 6 == 0:
            n //= 6
            ans += 1
        else:
            n *= 2
            ans += 1
        if n > 100000000000:
            print(-1)
            break
    else:
        print(ans)
