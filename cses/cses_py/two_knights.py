out = []
for i in range(1, int(input()) + 1):
    if i <= 5:
        out.append([0, 6, 28, 96, 252][i - 1])
    else:
        ans = (i * i) * ((i * i) - 1) // 2
        sub = 0
        sub += 32
        sub += (i - 3) * 16
        sub += (i - 4) * 24
        sub += (i - 4) * (i - 4) * 8
        ans -= sub // 2
        out.append(ans)

print("\n".join(map(str, out)))
