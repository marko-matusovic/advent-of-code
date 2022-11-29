def parse():
    inp = """forward 8
    forward 3
    down 8
    forward 4
    down 2
    down 4
    down 8
    down 4
    down 7
    up 4
    down 6
    down 2
    forward 7
    down 5
    down 7
    down 5
    forward 8
    forward 4
    forward 3
    down 4
    forward 2
    up 3
    up 6
    forward 7
    down 4
    down 2
    forward 7
    up 3
    forward 8
    down 4
    down 6
    forward 3
    forward 9
    down 9
    down 6
    forward 8
    up 8
    forward 3
    down 8
    down 4
    forward 7
    forward 3
    up 9
    down 8
    forward 7
    forward 5
    forward 3
    down 1
    down 4
    forward 1
    down 6
    forward 9
    up 2
    forward 3
    down 2
    down 1
    up 5
    down 8
    down 7
    down 5
    down 9
    up 7
    forward 4
    down 9
    down 8
    down 4
    forward 3
    down 1
    up 8
    down 1
    down 8
    forward 4
    up 1
    forward 7
    down 6
    down 2
    forward 3
    down 9
    forward 9
    forward 4
    down 1
    up 6
    down 1
    up 2
    down 4
    down 9
    down 7
    forward 5
    down 3
    up 9
    forward 4
    up 5
    down 9
    forward 1
    forward 7
    forward 1
    down 8
    forward 5
    down 2
    down 4
    down 3
    forward 4
    down 9
    up 7
    up 1
    forward 4
    up 2
    forward 4
    forward 4
    down 1
    down 5
    forward 6
    forward 1
    down 6
    forward 8
    forward 8
    forward 6
    down 3
    forward 3
    forward 1
    forward 5
    forward 7
    down 5
    forward 8
    down 7
    down 8
    forward 7
    forward 8
    down 7
    forward 7
    up 2
    up 9
    down 1
    forward 1
    forward 6
    up 3
    up 6
    down 2
    down 4
    forward 2
    up 8
    forward 5
    up 9
    up 5
    down 2
    forward 5
    forward 9
    up 3
    forward 7
    down 4
    down 3
    down 2
    forward 9
    up 4
    down 7
    down 1
    forward 9
    forward 3
    up 4
    forward 1
    up 6
    forward 5
    forward 2
    forward 2
    down 9
    forward 1
    up 7
    forward 3
    down 1
    down 5
    up 8
    down 5
    forward 5
    forward 5
    down 9
    up 5
    forward 8
    up 9
    down 1
    forward 1
    down 7
    up 3
    down 9
    down 9
    down 5
    up 6
    forward 7
    down 5
    down 2
    down 5
    forward 3
    forward 2
    forward 9
    forward 5
    forward 6
    up 7
    up 7
    forward 4
    forward 2
    up 4
    up 6
    down 8
    up 9
    up 5
    down 8
    forward 4
    up 9
    forward 8
    forward 3
    down 6
    down 6
    down 7
    down 6
    forward 7
    up 5
    down 2
    down 5
    forward 2
    up 5
    forward 7
    down 5
    up 4
    up 2
    forward 2
    forward 9
    forward 5
    forward 8
    up 4
    forward 6
    up 7
    forward 5
    down 2
    down 3
    down 8
    up 2
    down 2
    up 1
    up 7
    down 1
    down 6
    down 9
    down 7
    forward 8
    up 8
    up 9
    down 9
    forward 6
    forward 6
    down 6
    forward 3
    forward 4
    forward 6
    forward 7
    down 5
    down 8
    forward 7
    forward 9
    down 4
    up 9
    forward 8
    down 6
    down 1
    down 2
    forward 4
    down 5
    up 1
    forward 1
    down 6
    forward 4
    forward 2
    forward 6
    down 8
    up 2
    up 8
    forward 3
    down 6
    forward 7
    down 1
    forward 1
    forward 8
    down 8
    down 8
    down 2
    forward 8
    down 4
    up 8
    down 6
    forward 2
    down 5
    up 3
    up 1
    down 2
    forward 4
    up 7
    forward 2
    up 9
    forward 1
    down 5
    forward 3
    up 9
    up 4
    down 4
    up 6
    down 1
    forward 9
    up 5
    forward 5
    up 4
    down 6
    forward 2
    up 6
    forward 5
    forward 5
    down 4
    up 7
    forward 2
    down 9
    down 8
    down 7
    forward 4
    forward 7
    down 4
    forward 9
    up 2
    forward 1
    up 4
    down 8
    forward 9
    up 2
    up 5
    down 7
    up 6
    forward 7
    up 1
    forward 3
    down 3
    down 2
    down 6
    down 6
    forward 3
    down 2
    down 2
    down 5
    down 4
    down 6
    down 2
    forward 7
    down 6
    forward 4
    down 5
    down 7
    down 3
    forward 2
    forward 8
    forward 1
    up 6
    down 8
    down 4
    up 5
    forward 5
    down 7
    forward 5
    forward 8
    forward 7
    down 8
    down 5
    down 3
    up 2
    forward 8
    forward 7
    down 6
    up 1
    down 9
    down 3
    down 7
    down 2
    forward 9
    forward 2
    forward 4
    forward 8
    forward 3
    forward 2
    down 3
    down 8
    up 7
    down 7
    down 1
    forward 8
    down 2
    up 4
    forward 9
    down 6
    forward 5
    forward 8
    forward 1
    forward 4
    down 6
    down 1
    down 6
    forward 1
    forward 7
    down 3
    down 3
    down 1
    up 2
    down 6
    down 5
    down 6
    forward 5
    forward 2
    forward 1
    forward 8
    up 5
    down 2
    down 9
    down 9
    down 5
    down 3
    up 6
    forward 2
    down 7
    down 7
    down 2
    down 2
    up 6
    forward 8
    forward 6
    forward 4
    down 1
    forward 7
    down 5
    up 9
    forward 5
    down 4
    down 9
    up 7
    forward 1
    up 3
    up 7
    forward 6
    forward 8
    up 6
    up 3
    forward 4
    up 3
    down 1
    forward 8
    forward 1
    up 3
    forward 7
    up 2
    forward 6
    forward 2
    down 2
    down 9
    down 9
    forward 1
    forward 8
    down 5
    forward 4
    forward 5
    forward 5
    down 9
    forward 8
    up 8
    down 6
    down 2
    down 5
    down 6
    up 9
    up 8
    down 4
    down 9
    up 4
    forward 6
    forward 1
    forward 3
    up 4
    up 3
    forward 3
    forward 8
    down 1
    down 2
    down 2
    forward 9
    forward 4
    up 2
    forward 7
    down 4
    up 1
    forward 2
    forward 9
    forward 8
    down 4
    down 7
    up 7
    down 1
    forward 3
    down 7
    down 8
    up 2
    forward 2
    forward 8
    forward 4
    forward 4
    down 5
    forward 8
    down 2
    up 7
    down 1
    up 9
    up 5
    down 3
    forward 6
    forward 2
    forward 6
    down 5
    down 6
    forward 9
    up 5
    up 6
    down 4
    down 5
    up 4
    down 6
    down 2
    down 5
    down 4
    forward 7
    forward 6
    down 8
    forward 2
    down 5
    down 7
    down 2
    forward 8
    forward 6
    down 3
    forward 4
    up 6
    down 9
    down 3
    forward 3
    forward 3
    down 9
    up 1
    up 3
    forward 3
    forward 6
    forward 1
    forward 4
    forward 3
    forward 3
    forward 3
    down 6
    down 8
    forward 3
    down 5
    forward 8
    forward 3
    down 4
    up 3
    up 1
    down 7
    forward 7
    up 6
    forward 7
    down 4
    down 7
    up 9
    down 9
    forward 8
    down 5
    down 2
    forward 9
    down 3
    forward 4
    forward 4
    forward 4
    forward 6
    down 1
    up 5
    forward 8
    down 6
    forward 5
    up 5
    up 5
    down 2
    down 9
    down 7
    up 3
    up 7
    up 6
    forward 2
    forward 6
    up 9
    forward 5
    forward 2
    up 4
    down 4
    down 5
    forward 7
    down 5
    down 7
    forward 3
    down 6
    down 1
    forward 6
    up 5
    up 6
    up 3
    down 9
    up 7
    forward 9
    down 3
    forward 4
    up 2
    forward 9
    down 2
    up 2
    up 5
    forward 1
    down 8
    down 1
    down 8
    up 2
    forward 9
    forward 1
    up 2
    down 1
    up 1
    forward 2
    down 7
    forward 9
    up 1
    forward 8
    down 5
    down 5
    down 9
    forward 5
    down 7
    down 1
    forward 9
    down 5
    forward 9
    forward 9
    down 6
    down 5
    down 6
    forward 3
    down 4
    up 8
    down 6
    up 4
    down 9
    up 1
    up 2
    up 8
    forward 6
    down 7
    down 4
    down 9
    down 3
    forward 9
    down 5
    forward 6
    down 6
    forward 7
    down 9
    forward 2
    forward 2
    down 8
    down 7
    forward 5
    down 8
    forward 6
    down 1
    forward 6
    forward 7
    forward 3
    forward 2
    forward 1
    forward 1
    down 9
    forward 7
    up 9
    down 5
    forward 6
    down 4
    down 2
    forward 4
    forward 3
    forward 9
    down 8
    down 2
    forward 4
    down 8
    down 6
    forward 9
    down 7
    forward 1
    up 1
    forward 3
    down 5
    down 8
    up 6
    forward 9
    forward 4
    down 2
    forward 4
    up 2
    forward 9
    down 5
    down 1
    down 9
    forward 5
    down 9
    forward 1
    down 9
    forward 8
    down 5
    forward 9
    forward 8
    up 8
    down 7
    up 9
    down 5
    up 9
    forward 7
    forward 7
    forward 1
    up 2
    up 2
    forward 6
    up 5
    up 5
    down 7
    forward 2
    up 8
    forward 8
    down 8
    forward 7
    forward 8
    down 7
    down 8
    forward 4
    forward 2
    down 8
    up 4
    down 1
    up 7
    forward 4
    forward 3
    down 6
    up 6
    forward 6
    down 1
    forward 3
    down 6
    forward 6
    up 3
    up 1
    up 5
    down 1
    up 9
    down 8
    forward 7
    forward 9
    up 2
    forward 2
    forward 6
    up 2
    up 3
    forward 1
    forward 2
    forward 8
    down 6
    forward 5
    down 8
    forward 2
    up 1
    down 1
    down 3
    forward 9
    down 6
    down 1
    down 6
    down 6
    down 6
    down 1
    up 6
    down 6
    forward 6
    down 5
    down 2
    up 5
    forward 7
    down 3
    forward 9
    forward 2
    down 2
    down 1
    down 9
    up 3
    down 2
    forward 2
    forward 6
    down 4
    up 7
    up 3
    up 9
    forward 3
    forward 8
    forward 9
    forward 4
    forward 2
    up 7
    up 5
    down 9
    forward 1
    up 1
    down 7
    forward 5
    forward 5
    down 3
    forward 1
    forward 6
    up 8
    down 4
    down 2
    up 2
    forward 2
    forward 1
    down 4
    up 9
    down 1
    down 4
    down 6
    forward 3
    forward 7
    down 4
    up 8
    down 6
    forward 7
    forward 4
    up 2
    down 8
    forward 5
    down 1
    forward 7
    down 1
    up 6
    down 9
    forward 3
    forward 1
    up 2
    up 4
    forward 8
    forward 1
    up 8
    forward 3
    forward 7
    up 9
    up 6
    up 9
    down 9
    down 5
    forward 8
    forward 7
    down 5
    forward 7
    forward 6
    forward 8
    up 4
    forward 6
    down 6
    up 8
    down 7
    up 8
    forward 6
    forward 7
    down 4
    forward 4
    up 2
    forward 9
    down 6
    down 2
    down 8
    forward 6
    up 2
    up 8
    forward 2
    down 3
    forward 7
    forward 6
    down 9
    up 1
    forward 1
    down 8
    down 8
    forward 4
    forward 4
    up 6
    down 8
    up 8
    forward 1
    forward 1
    down 6
    up 3
    up 7
    forward 1
    forward 1
    up 2
    forward 2
    down 4
    up 8
    forward 1
    up 7
    down 2
    forward 2
    down 1
    forward 1
    down 4
    forward 4
    forward 8
    up 5
    down 6
    up 8
    forward 4
    down 3
    up 6
    forward 3
    forward 4
    down 5
    forward 1
    forward 6
    forward 4
    forward 1
    down 7
    down 4
    forward 2
    down 6
    forward 3
    down 5
    forward 3
    forward 4
    forward 4
    forward 9
    down 5
    forward 7
    forward 7
    forward 9
    down 1
    up 6
    forward 1
    down 9
    forward 3
    down 7
    up 8
    up 6
    down 7
    forward 2
    down 9
    forward 9
    forward 6
    down 9
    forward 6
    down 8
    forward 1
    up 6
    down 1
    forward 6
    down 9
    forward 6
    forward 7
    forward 3
    forward 8
    forward 5
    forward 8
    down 9
    down 2
    forward 3
    down 3
    up 4
    down 4
    down 3
    forward 3
    forward 3
    down 5
    forward 5
    forward 1
    down 9
    down 3
    up 7
    forward 9
    up 1
    down 1
    down 6
    up 1
    forward 2"""

    data = []
    for x in inp.split('\n'):
        [a, b] = x.split()
        data.append((a, int(b)))

    return data
    # return [(x.split(" ")[0], int(x.split(" ")[1])) for x in inp.split("\n")]

def part1(data):

    x = 0
    y = 0

    for (n, d) in data:
        if n == 'forward':
            x += d
        elif n == 'down':
            y += d
        elif n == 'up':
            y -= d
        else:
            print('what', n, d)

    print("Part 1:", x*y)

def part2(data):

    aim = 0
    x = 0
    y = 0

    for (n, d) in data:
        if n == 'forward':
            x += d
            y += aim * d
        elif n == 'down':
            aim += d
        elif n == 'up':
            aim -= d
        else:
            print('what', n, d)

    print("Part 2:", x*y)

if __name__ == '__main__':
    data = parse()
    part1(data)
    part2(data)