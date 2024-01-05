import math


def parse():
    data = open('input/10.d', 'r').read().split('\n')

    return data

def part1(data):
    points = {
        ')': 3,
        ']': 57,
        '}': 1197,
        '>': 25137
    }

    bracket = {
        '(': ')',
        '[': ']',
        '{': '}',
        '<': '>'
    }

    score = 0

    for line in data:
        buff = []
        for c in line:
            if c in '([{<':
                buff.append(c)
            else:
                if c != bracket[buff.pop()]:
                    score += points[c]
                    break

    print("Part 1:", score)

def part2(data):
    points = {
        ')': 1,
        ']': 2,
        '}': 3,
        '>': 4
    }

    bracket = {
        '(': ')',
        '[': ']',
        '{': '}',
        '<': '>'
    }

    scores = []

    for line in data:
        buff = []
        corrupt = False
        for c in line:
            if c in '([{<':
                buff.append(c)
            else:
                if c != bracket[buff.pop()]:
                    corrupt = True
                    break
        if corrupt:
            continue
        this_score = 0
        buff.reverse()
        for c in buff:
            this_score *= 5
            this_score += points[bracket[c]]

        scores.append(this_score)

    scores.sort()
    print("Part 2:", scores[math.floor(len(scores)/2)])

if __name__ == '__main__':
    data = parse()
    part1(data)
    part2(data)