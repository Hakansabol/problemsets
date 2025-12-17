# Fixed XOR
from help import numb


if __name__ == "__main__":
    n1 = numb(input(), "hex")
    n2 = numb(input(), "hex")

    print(n1.xorwith(n2).hex())
