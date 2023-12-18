import numpy as np

def parse():
    data = open('input/8.data', 'r').read()

    return [([[s for s in d] for d in row.split(" | ")[0].split(" ")], [[s for s in d] for d in row.split(" | ")[1].split(" ")]) for row in data.split("\n")]

def part1(data):

    count = 0
    for dat in data:
        (_, b) = dat
        for s in b:
            if len(s) in [2, 3, 4, 7]:
                # print(s)
                count += 1

    print("Part 1:", count)

    # 350 too low

def intersect(a, b):
    return np.intersect1d(a, b)

def union(a, b):
    return np.union1d(a, b)

def diff(a, b):
    return np.setdiff1d(a, b)

def sameset(a, b):
    return len(a) == len(b) == len(np.intersect1d(a, b))

def part2(data):

    summ = 0

    for dat in data:

        (train, test) = dat

        # create groups

        segments = {
            2 : [s for s in 'abcdefg'],
            3 : [s for s in 'abcdefg'],
            4 : [s for s in 'abcdefg'],
            5 : [s for s in 'abcdefg'],
            6 : [s for s in 'abcdefg'],
            7 : [s for s in 'abcdefg'],
        }

        for length in range(2, 8):
            for dig in train:
                if len(dig)!=length:
                    continue
                segments[length] = intersect(segments[length], dig)

        # print(segments)

        # apply logic to obtain all segments

        mapped = {} # real segment -> shown segment
        mapped['a'] = diff(segments[3], segments[2])[0]
        mapped['f'] = intersect(segments[2], segments[6])[0]
        mapped['c'] = diff(segments[2], [mapped['f']])[0]
        mapped['b'] = diff(diff(segments[4], segments[5]), [mapped[s] for s in 'cf'])[0]
        mapped['d'] = diff(segments[4], [mapped[s] for s in 'bcf'])[0]
        mapped['g'] = diff(segments[6], [mapped[s] for s in 'abf'])[0]
        mapped['e'] = diff(segments[7], [mapped[s] for s in 'abcdfg'])[0]

        print(mapped)

        # use obtained segments to get the final number

        segments = {
            0 : [mapped[s] for s in 'abcefg'],
            1 : [mapped[s] for s in 'cf'],
            2 : [mapped[s] for s in 'acdeg'],
            3 : [mapped[s] for s in 'acdfg'],
            4 : [mapped[s] for s in 'bcdf'],
            5 : [mapped[s] for s in 'abdfg'],
            6 : [mapped[s] for s in 'abdefg'],
            7 : [mapped[s] for s in 'acf'],
            8 : [mapped[s] for s in 'abcdefg'],
            9 : [mapped[s] for s in 'abcdfg']
        }

        number = 0

        for digit in test:
            number *= 10
            for seg in segments:
                if sameset(digit,segments[seg]):
                    number += seg
                    break

        print(number)
        summ += number

    print("Part 2:", summ)

if __name__ == '__main__':
    data = parse()

    # print(data[0])

    part1(data)
    part2(data)