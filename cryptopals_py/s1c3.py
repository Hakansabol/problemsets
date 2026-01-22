# Single-byte XOR cipher
from help import numb, hexkey, base64key, etaoin

if __name__ == "__main__":
    n = numb(input(), "hex")

    o = []
    for i in range(256):
        xo = numb(bin(i).zfill(8), "bin")
        o.append(n.xorwith(xo).str())
    print("\n".join(etaoin(o, 30)))
