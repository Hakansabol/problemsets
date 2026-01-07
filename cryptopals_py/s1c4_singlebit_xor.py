# Single-byte XOR cipher
from help import numb, etaoinshrdlu
import time


def single_xor(topc: int, data: numb) -> list:
    o = []
    for i in range(256):
        xo = numb(bin(i).zfill(8), "bin")
        o.append(data.xorwith(xo).str())
    out = etaoinshrdlu(o, topc)
    return out


if __name__ == "__main__":
    o = []
    for i in range(327):
        n = numb(input(), "hex")
        print(n.str())
        time.sleep(5)
        o.extend(single_xor(10, n))
    for i in range(30):
        print(etaoinshrdlu(o, i))
