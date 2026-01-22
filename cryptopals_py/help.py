from help2 import scoreDecryptedBuffer

hexkey: str = "0123456789abcdef"
base64key: str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"


class numb:
    def __init__(self, val, typ):
        if typ == "hex":
            self.val = "".join([format(hexkey.find(a), "04b") for a in val])
        if typ == "base64":
            self.val = "".join([format(base64key.find(a), "06b") for a in val])
        if typ == "bin":
            if "b" in val:
                raise ValueError
            self.val = val
        if typ == "str":
            self.val = "".join([format((ord(a)), "08b") for a in val])

    def hex(self):
        return "".join(
            [
                hexkey[int(self.val[i * 4 : i * 4 + 4], 2)]
                for i in range(len(self.val) // 4)
            ]
        )

    def base64(self):
        return "".join(
            [
                base64key[int(self.val[i * 6 : i * 6 + 6], 2)]
                for i in range(len(self.val) // 6)
            ]
        )

    def bin(self):
        return self.val

    def str(self):
        return "".join(
            [
                chr(int(self.val[i * 8 : i * 8 + 8], 2))
                for i in range(len(self.val) // 8)
            ]
        )

    def xorwith(self, other):
        b1 = self.bin()
        b2 = other.bin()
        ret = "".join(
            ["1" if b1[i] != b2[i % len(b2)] else "0" for i in range(len(b1))]
        )
        return numb(ret, "bin")


def hex_to_bin(s: str) -> str:
    return "".join([format(hexkey.find(a), "04b") for a in s])


def hex_to_64(s: str) -> str:
    vbin: str = hex_to_bin(s)
    v64 = "".join(
        [base64key[int(vbin[i * 6 : i * 6 + 6], 2)] for i in range(len(vbin) // 6)]
    )
    return v64


# https://gist.github.com/pozhidaevak/0dca594d6f0de367f232909fe21cdb2f
freq = {
    "E": 12.0,
    "T": 9.10,
    "A": 8.12,
    "O": 7.68,
    "I": 7.31,
    "N": 6.95,
    "S": 6.28,
    "R": 6.02,
    "H": 5.92,
    "D": 4.32,
    "L": 3.98,
    "U": 2.88,
    "C": 2.71,
    "M": 2.61,
    "F": 2.30,
    "Y": 2.11,
    "W": 2.09,
    "G": 2.03,
    "P": 1.82,
    "B": 1.49,
    "V": 1.11,
    "K": 0.69,
    "X": 0.17,
    "Q": 0.11,
    "J": 0.10,
    "Z": 0.07,
    " ": 0,
}


def score(s: str):
    # return sum([freq[a.upper()] if a.upper() in freq else -1 for a in s])
    return sum(
        [
            freq[a.upper()] if a.upper() in freq else (0 if not a.isascii() else 0)
            for a in s
        ]
    )


def etaoinshrdlu(objs, ind: int = 1):
    enl = [[score(objs[b]), objs[b]] for b in range(len(objs))]
    enl = sorted(enl, reverse=True)
    return [enl[i][1] for i in range(0, ind)]


def etaoinenglish(objs, ind: int = 1):
    enl = [
        [scoreDecryptedBuffer(objs[b].lower()), objs[b], hex(b)]
        for b in range(len(objs))
    ]
    enl = sorted(enl, reverse=True)
    return [enl[i][1] for i in range(0, ind)]


def etaoinhistogram(objs, ind: int = 1):
    enl = []
    for text in objs:
        hist = [0 for i in range(26)]
        jv = ""
        for a in text:
            letr = ord(a) - ord("a")
            if letr < 26 and letr >= 0:
                jv += a
                hist[letr] += 1
            letr = ord(a) - ord("A")
            if letr < 26 and letr >= 0:
                jv += a
                hist[letr] += 1
        cnt = sum(hist)
        if cnt < len(text) * 0.8:
            continue
        hist = [a / cnt * 100 for a in hist]  # out of 100
        difference = sum([abs(hist[i] - freq[chr(ord("A") + i)]) for i in range(26)])
        enl.append([difference, text])
    enl = sorted(enl, reverse=False)
    return [enl[i][1] for i in range(0, min(ind, len(enl)))]


def etaoin(text: list, idx: int) -> list:
    return etaoinenglish(text[0], idx)


def hamming_distance(l: numb, r: numb):
    lb = l.bin()
    rb = r.bin()
    if len(lb) != len(rb):
        return -1
    return sum([1 for i in range(len(lb)) if lb[i] != rb[i]])


# unit tests
if __name__ == "__main__":
    t = input()
    if t == "hamming_distance":
        s1 = input()
        s2 = input()
        print(hamming_distance(numb(s1, "str"), numb(s2, "str")))
