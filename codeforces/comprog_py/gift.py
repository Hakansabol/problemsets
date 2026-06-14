n = int(input())
g = []
ans = [[] for _ in range(n)]
for _ in range(n):
    q=list(map(int, input().split()))
    for i in range(1, len(q)):
        ans[q[i] - 1].append(_+1)
for a in ans:
    print(len(a),end=' ')
    print(' '.join(map(str, a)))


