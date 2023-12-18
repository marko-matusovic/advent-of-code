import numpy as np

def parse():
    data = open('input/6.data', 'r').read()

    return [int(n) for n in data.split(",")]

def part1(data):
    count = np.zeros(10, np.int64)

    for n in data:
        count[n] += 1

    for i in range(80):
        count[7] += count[0]
        count[9] += count[0]
        count[0] = 0
        count = np.roll(count, -1)

    print("Part 1:", np.sum(count))

def part2(data):
    count = np.zeros(10, np.int64)

    for n in data:
        count[n] += 1

    for i in range(256):
        count[7] += count[0]
        count[9] += count[0]
        count[0] = 0
        count = np.roll(count, -1)

    print("Part 2:", np.sum(count))

if __name__ == '__main__':
    data = parse()
    part1(data)
    part2(data)