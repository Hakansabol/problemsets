# Repeating-key XOR
from help import numb, hexkey, base64key, etaoinshrdlu


def repeating_key_xor(s: numb, k: numb):
    return s.xorwith(k)


if __name__ == "__main__":
    s = input()
    s2 = input()
    s3 = s + '\n' + s2

    n = numb(s3, "str")
    print(repeating_key_xor(n, numb("ICE", 'str')).hex())
