# Single-byte XOR cipher
from help import numb, hexkey, base64key, etaoinshrdlu

if __name__ == "__main__":
    n = numb(input(), "hex")
    print(n.str())

    o = []
    for i in range(256):
        xo = numb(bin(i).zfill(8), "bin")
        o.append(n.xorwith(xo).str())
    print("\n".join(etaoinshrdlu(o, 30)))
