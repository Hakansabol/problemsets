hexkey: str = "0123456789abcdef"
base64key: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="


class numb:
    def __init__(self, val, typ):
        if (typ == "hex"):
            self.val = "".join([format(hexkey.find(a), '04b') for a in val])
        if (typ == "base64"):
            self.val = "".join([format(base64key.find(a), '06b') for a in val])
        if (typ == "bin"):
            self.val = val

    def hex(self):
        return "".join([hexkey[int(self.val[i*4:i*4+4], 2)] for i in range(len(self.val) // 4)])

    def base64(self):
        return "".join([base64key[int(self.val[i*6:i*6+6], 2)] for i in range(len(self.val) // 6)])

    def bin(self):
        return self.val

    def str(self):
        return ''.join([chr(int(self.val[i*8:i*8+8], 2)) for i in range(len(self.val) // 8)])

    def xorwith(self, other):
        b1 = self.bin()
        b2 = other.bin()
        ret = ''.join(["1" if b1[i] != b2[i % len(b2)]
                      else "0" for i in range(len(b1))])
        return numb(ret, "bin")


def hex_to_bin(s: str) -> str:
    return "".join([format(hexkey.find(a), '04b') for a in s])


def hex_to_64(s: str) -> str:
    vbin: str = hex_to_bin(s)
    v64 = "".join([base64key[int(vbin[i*6:i*6+6], 2)]
                  for i in range(len(vbin) // 6)])
    return v64


def etaoinshrdlu(objs, ind: int = 0):
    enl = [[sum([1 for a in objs[b] if a.lower() in "etaoinshrdlu"]), objs[b]]
           for b in range(len(objs))]
    enl = sorted(enl, reverse=True)
    return enl[ind][1]
