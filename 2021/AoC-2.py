def parse():
    inp = open('input/2.data', 'r').read()

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