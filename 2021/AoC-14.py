from collections import defaultdict

def parse():
    raw = '''ONSVVHNCFVBHKVPCHCPV

VO -> C
VV -> S
HK -> H
FC -> C
VB -> V
NO -> H
BN -> B
FP -> K
CS -> C
HC -> S
FS -> K
KH -> V
CH -> H
BP -> K
OF -> K
SS -> F
SP -> C
PN -> O
CK -> K
KS -> H
HO -> K
FV -> F
SN -> P
HN -> O
KK -> H
KP -> O
CN -> N
BO -> C
CC -> H
PB -> F
PV -> K
BV -> K
PP -> H
KB -> F
NC -> F
PC -> V
FN -> N
NH -> B
CF -> V
PO -> F
KC -> S
VP -> P
HH -> N
OB -> O
KN -> O
PS -> N
SF -> V
VK -> F
CO -> N
KF -> B
VC -> C
SH -> S
HV -> V
FK -> O
NV -> N
SC -> O
BK -> F
BB -> K
HF -> K
OC -> O
KO -> V
OS -> P
FF -> O
PH -> F
FB -> O
NN -> C
NK -> C
HP -> B
PF -> H
PK -> C
NP -> O
NS -> V
CV -> O
VH -> C
OP -> N
SO -> O
SK -> H
SV -> O
NF -> H
BS -> K
BH -> O
VN -> S
HB -> O
OH -> K
CB -> B
BC -> S
OV -> F
BF -> P
OO -> F
HS -> H
ON -> P
NB -> F
CP -> S
SB -> V
VF -> C
OK -> O
FH -> H
KV -> S
FO -> C
VS -> B'''

#     # example
#     raw = '''NNCB
#
# CH -> B
# HH -> N
# CB -> H
# NH -> C
# HB -> C
# HC -> B
# HN -> C
# NN -> C
# BH -> H
# NC -> B
# NB -> B
# BN -> B
# BB -> N
# BC -> B
# CC -> N
# CN -> C'''

    (start, rules_raw) = raw.split('\n\n')

    rules = {}

    for rule in rules_raw.split('\n'):
        (a, b) = rule.split(' -> ')
        rules[a] = b

    return (start, rules)

def part1(data):
    (start, rules) = data

    poly = start

    # print(poly)
    for step in range(10):
        i = 1
        while i < len(poly):
            if poly[i-1:i+1] in rules:
                poly = poly[:i] + rules[poly[i-1:i+1]] + poly[i:]
                i += 1
            i += 1
        # print(poly)

    counts = {}
    for c in 'ABCDEFGHIJKLMNOPQRSTUVWXYZ':
        counts[c] = 0

    for p in poly:
        counts[p] += 1

    maxv = max(counts.values())
    minv = maxv
    for v in counts.values():
        if v > 0 and v < minv:
            minv = v

    print("Part 1:", maxv - minv)

def part2(data):
    (start, rules) = data

    pairs = defaultdict(int)
    elements = defaultdict(int)
    elements[start[0]] += 1
    for i in range(1, len(start)):
        pairs[start[i - 1] + start[i]] += 1
        elements[start[i]] += 1

    for _ in range(40):
        new_pairs = defaultdict(int)
        for k, v in pairs.items():
            c = rules[k]
            new_pairs[k[0] + c] += v
            new_pairs[c + k[1]] += v
            elements[c] += v
        pairs = new_pairs

    most_common = max(elements, key=elements.get)
    least_common = min(elements, key=elements.get)

    print("Part 2:", elements[most_common] - elements[least_common])

if __name__ == '__main__':
    part1(parse())
    part2(parse())