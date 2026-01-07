# BREAK Repeating-key XOR
from help import numb, hexkey, base64key, etaoinshrdlu, hamming_distance
from s1c4_singlebit_xor import single_xor
import time


def repeating_key_xor(s: numb, k: numb):
    return s.xorwith(k)


def transpose(arr: list) -> list:
    print("Transpose!")
    # assume list of strings
    outsize = len(arr[0])
    print("type:", type(arr[0]), "len [0]:", outsize, "len:", len(arr))
    # remove trailing element that is too short
    if (len(arr[-1]) < len(arr[0])):
        arr.pop(len(arr)-1)

    out = ["" for i in range(outsize)]
    for i in range(len(arr)):
        for j in range(outsize):
            out[j] += arr[i][j]
    print(arr[0][0:2])
    print(arr[1][0:2])
    print(out[0][0:2])
    print(out[1][0:2])
    print("type:", type(out[0]), "len [0]:", len(out[0]), "len:", len(out))
    return out


def listpose(data: str, blocksize: int):
    i = 0
    out = []
    while i < len(data):
        out.append(data[i:i+blocksize])
        i += blocksize
    return out


if __name__ == "__main__":
    s = numb(input(), "base64")

    if (hamming_distance(numb("this is a test", "str"), numb("wokka wokka!!!", "str")) != 37):
        print("hamming distance failure!")

    scores = []
    for KEYSIZE in range(2, 40):
        s1 = numb(s.str()[0:KEYSIZE], 'str')
        s2 = numb(s.str()[KEYSIZE:KEYSIZE*2], 'str')
        dist = hamming_distance(s1, s2) / KEYSIZE
        scores.append([dist, KEYSIZE])

    scores.sort()
    for i in range(39):
        clen = scores[i][1]
        print("\nKEYSIZE:", clen)
        input()

        sbyte = s.str()

        blocks = listpose(sbyte, clen)
        transposed = transpose(blocks)

        o = []
        for j in range(len(transposed)):
            v = transposed[j]
            o.append(single_xor(1, numb(v, 'str'))[0])
        print(''.join(o))
        output = transpose(o)
        print(''.join(output))
