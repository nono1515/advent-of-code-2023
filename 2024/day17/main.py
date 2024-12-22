import re

a, b, c, *prog = map(int, re.findall(r"\d+", open("input.txt").read()))


def eval(a, b, c, i=0, R=[]):
    while i in range(len(prog)):
        C = {0: 0, 1: 1, 2: 2, 3: 3, 4: a, 5: b, 6: c}

        match prog[i : i + 2]:
            case 0, op:
                a = a >> C[op]
            case 1, op:
                b = b ^ op
            case 2, op:
                b = 7 & C[op]
            case 3, op:
                i = op - 2 if a else i
            case 4, op:
                b = b ^ c
            case 5, op:
                R = R + [C[op] & 7]
            case 6, op:
                b = a >> C[op]
            case 7, op:
                c = a >> C[op]
        i += 2
    return R


print(*eval(a, b, c), sep=",")

sols = []


def find(a, i):
    if eval(a, b, c) == prog:
        sols.append(a)
    if eval(a, b, c) == prog[-i:] or not i:
        for n in range(8):
            find(8 * a + n, i + 1)


find(0, 0)
print(min(sols))
