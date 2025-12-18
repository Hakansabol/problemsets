# Single-byte XOR cipher
from help import numb, hexkey, base64key, etaoinshrdlu

if __name__ == "__main__":
    o = []
    for i in range(327):
        n = numb(input(), "hex")

        for i in range(256):
            xo = numb(bin(i).zfill(8), "bin")
            o.append(n.xorwith(xo).str())
    for i in range(30):
        print(etaoinshrdlu(o, i))
    print()
