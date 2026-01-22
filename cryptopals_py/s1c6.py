# BREAK Repeating-key XOR
from help import numb, hexkey, base64key, hamming_distance, etaoin
from s1c4_singlebit_xor import single_xor


def repeating_key_xor(s: numb, k: numb):
    return s.xorwith(k)


def transpose(arr: list) -> list:
    # assume list of strings
    outsize = len(arr[0])
    # remove trailing element that is too short
    if len(arr[-1]) < len(arr[0]):
        arr.pop(len(arr) - 1)

    out = ["" for i in range(outsize)]
    for i in range(len(arr)):
        for j in range(outsize):
            out[j] += arr[i][j]
    return out


def listpose(data: str, blocksize: int):
    i = 0
    out = []
    while i < len(data):
        out.append(data[i : i + blocksize])
        i += blocksize
    return out


if __name__ == "__main__":
    s = numb(input(), "base64")

    if (
        hamming_distance(numb("this is a test", "str"), numb("wokka wokka!!!", "str"))
        != 37
    ):
        print("hamming distance failure!")

    sbyte = s.str()
    scores = []
    for KEYSIZE in range(2, 41):
        s1 = numb(sbyte[0:KEYSIZE] + sbyte[KEYSIZE * 2 : KEYSIZE * 3], "str")
        s2 = numb(
            sbyte[KEYSIZE : KEYSIZE * 2] + sbyte[KEYSIZE * 3 : KEYSIZE * 4], "str"
        )
        dist = hamming_distance(s1, s2) / KEYSIZE
        scores.append([dist, KEYSIZE])

    scores.sort()
    print(scores)
    for i in range(10):
        clen = scores[i][1]
        print("\nKEYSIZE:", clen)

        blocks = listpose(sbyte, clen)
        transposed = transpose(blocks)

        gkey = "54 65 72 6d 69 6e 61 74 6f 72 20 58 3a 20 42 72 69 6e 67 20 74 68 65 20 6e 6f 69 73 65".split()
        kz = "".join([numb(a, "hex").str() for a in gkey])

        o = []
        for j in range(len(transposed)):
            v = transposed[j]
            o.append(etaoin(single_xor(numb(v, "str")), 1)[0])
        output = transpose(o)
        print("".join(output))
