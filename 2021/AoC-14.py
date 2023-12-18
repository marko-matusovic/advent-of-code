from collections import defaultdict

def parse():
    raw = open('input/14', 'r').read()

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