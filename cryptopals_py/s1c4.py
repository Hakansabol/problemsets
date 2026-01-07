# Single-byte XOR cipher
from help import numb, etaoinshrdlu


def single_xor(topc: int, data: numb) -> list:
    o = []
    for i in range(256):
        xo = numb(bin(i).zfill(8), "bin")
        o.append(data.xorwith(xo).str())
    ret = []
    for i in range(topc):
        ret.append(etaoinshrdlu(o, i))
    return ret


if __name__ == "__main__":
    o = []
    for i in range(327):
        n = numb(input(), "hex")
        o.extend(single_xor(10, n))
    for i in range(30):
        print(etaoinshrdlu(o, i))
