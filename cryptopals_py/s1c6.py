# BREAK Repeating-key XOR
from help import numb, hexkey, base64key, etaoinshrdlu, hamming_distance
from s1c4 import single_xor
import time


def repeating_key_xor(s: numb, k: numb):
    return s.xorwith(k)


if __name__ == "__main__":
    s = numb(input(), "base64")

    if (hamming_distance(numb("this is a test", "str"), numb("wokka wokka!!!", "str")) != 37):
        print("hamming distance failure!")

    scores = []
    for KEYSIZE in range(2, 40):
        s1 = numb(s.str()[0:KEYSIZE], 'str')
        s2 = numb(s.str()[KEYSIZE:KEYSIZE*2], 'str')
        dist = hamming_distance(s1, s2) // KEYSIZE
        scores.append([dist, KEYSIZE])

    scores.sort()
    for i in range(4):
        clen = scores[i][1]

        sbyte = s.str()

        blocks = []
        for i in range(len(sbyte) // clen):
            blocks.append(sbyte[clen*i: clen*(i+1)])

        transp = ["" for j in range(clen)]
        for j in range(clen):
            for a in blocks:
                transp[j] += a[j]

        o = []
        for j in range(len(transp)):
            v = transp[j]
            o.append(single_xor(1, numb(v, 'str'))[0])

        out = ""
        for i in range(len(o[0])):
            for j in range(len(o)):
                out += (transp[j][i])
        print(numb(out, 'str').base64())
        time.sleep(6)
