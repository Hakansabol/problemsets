from math import lcm, gcd

for i in range(int(input())):
    n = int(input())
    v = list(map(int, input().split()))
    input()

    ans = 0
    for i in range(n):
        m = v[i]
        gcdv = 0
        if i == 0:
            gcdv = v[i + 1]
        elif i == n - 1:
            gcdv = v[i - 1]
        else:
            gcdv = (v[i - 1] * v[i + 1]) // gcd(v[i - 1], v[i + 1])
        if m != gcd(m, gcdv):
            ans += 1
    print(ans)

# 4 12 6
