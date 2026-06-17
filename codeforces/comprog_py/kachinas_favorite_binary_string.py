from sys import stdout
for _ in range(int(input())):
    n = int(input())
    jury = [0 for _ in range(n)]
    for i in range(n-1):
        print("?",i+1,n)
        stdout.flush()
        jury[i] = int(input())
    jury.append(0)
    b = ""
    if sum(jury) == 0:
        print("! IMPOSSIBLE")
        stdout.flush()
    else:
        for i in range(1,n):
            if jury[i] + jury[i-1] == 0:
                b += '0'
            elif jury[i] < jury[i-1]:
                b += '0'
            else:
                b += '1'
        last_zero = b.rfind('0')
        # print(last_zero,n,jury[last_zero])
        if jury[last_zero]+1 >= n-last_zero:
            b += '1'
        else:
            b += '0'
        print("!", b)
