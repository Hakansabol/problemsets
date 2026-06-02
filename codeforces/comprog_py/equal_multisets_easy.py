t = int(input())
for _ in range(t):
    n,k = list(map(int, input().split()))
    v1 = list(map(int, input().split()))
    v2 = list(map(int, input().split()))

    # check for dupes
    v2t = [a for a in v2 if a != -1]
    if len(v2t) != len(set(v2t)):
        print("NO")
        continue

    if k*2 > n:
        v1s = set(v1[n-k:k])
        v2s = set([a for a in v2[n-k:k] if a != -1])
        if len(v1s.intersection(v2s)) != len(v2s):
            print("nO")
            continue

    for i in range(n):
        if n-k <= i < k:
            continue
        if v2[i] != -1 and v2[i] != v1[i]:
            print("no")
            break
    else:
        print("YES")

