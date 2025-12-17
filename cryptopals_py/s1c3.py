# Single-byte XOR cipher
from help import numb, hexkey, base64key, etaoinshrdlu

if __name__ == "__main__":
    n = numb(input(), "hex")

    o = []
    for i in range(256):
        xo = numb(bin(i).zfill(8), "bin")
        o.append(n.xorwith(xo).str())
    for i in range(10):
        print(etaoinshrdlu(o, i))
