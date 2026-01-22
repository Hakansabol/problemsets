# Single-byte XOR cipher
from help import numb, etaoin
import time


def single_xor(data: numb) -> list:
    o = []
    o2 = []
    for i in range(256):
        xo = numb(bin(i)[2:].zfill(8), "bin")
        o.append(data.xorwith(xo).str())
        o2.append(xo)
    return [o, o2]


if __name__ == "__main__":
    o = []
    file = open("c4")
    for line in file:
        o.extend(single_xor(10, numb(line[0 : min(60, len(line))], "hex")))
    print("\n".join(etaoin(o, 80)))
