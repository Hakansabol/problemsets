n = int(input())

for _ in range(n):
    s = input().lower()
    chrs = set(s)

    aaaah = False

    sl = False # last was lettery
    switches = 0
    for a in s:
        if a.isnumeric() != sl:
            sl = a.isnumeric()
            switches += 1
    aaaah = switches >= 3


    if aaaah:
        vr = int(s[s.find('r') + 1:s.find('c')])
        vc = int(s[s.find('c') + 1:])
        ls = ""
        while vc > 0:
            ls = chr(ord('A') + ((vc-1) % 26)) + ls
            vc = (vc - 1) // 26
        print(str(ls) + str(vr))
    else:
        faln = [i for i in range(len(s)) if s[i].isnumeric()][0]
        vr = int(s[faln:])
        vc = s[0:faln]
        ln = 0
        for i in range(len(vc)):
            ln *= 26
            ln += (ord(vc[i]) - ord('a') + 1)
        print("R" + str(vr) + "C" + str(ln))

