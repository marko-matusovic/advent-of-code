import numpy as np


def parse():
    raw = open('input/13.d', 'r').read()

    [a, b] = raw.split('\n\n')

    points = [[int(n) for n in x.split(',')] for x in a.split('\n')]
    instructions = [(x[11], int(x[13:])) for x in b.split('\n')]

    return (points, instructions)


def fold_x(points, x):
    for (i, p) in enumerate(points):
        if p[0] > x:
            points[i][0] = 2 * x - p[0]


def fold_y(points, y):
    for (i, p) in enumerate(points):
        if p[1] > y:
            points[i][1] = 2 * y - p[1]


def part1(data):
    (points, instructions) = data

    ins = instructions[0]

    if ins[0] == 'x':
        fold_x(points, ins[1])
    elif ins[0] == 'y':
        fold_y(points, ins[1])

    map = np.zeros(10000 * 10000).reshape((10000, 10000))
    for p in points:
        map[p[0], p[1]] = 1

    dots = np.sum(map)
    print("Part 1:", int(dots))


def part2(data):
    (points, instructions) = data

    for ins in instructions:
        if ins[0] == 'x':
            fold_x(points, ins[1])
        elif ins[0] == 'y':
            fold_y(points, ins[1])

    points = np.array(points)
    max_x = np.max(points[:, 0]) + 1
    max_y = np.max(points[:, 1]) + 1

    map = np.zeros(max_x * max_y).reshape((max_x, max_y))
    for p in points:
        map[p[0], p[1]] = 1

    for row in map.T:
        for c in row:
            if c == 0:
                print('.', end='')
            if c == 1:
                print('#', end='')
        print()

    print("Part 2:")


if __name__ == '__main__':
    part1(parse())
    part2(parse())
