def parse():
    data = open('input/11', 'r').read()

    return [[int(n) for n in row] for row in data.split('\n')]


def part1(data):
    flashes = 0
    for step in range(100):
        for (i, row) in enumerate(data):
            for (j, o) in enumerate(row):
                data[i][j] += 1

        flashed = True
        while (flashed):
            flashed = False
            for (i, row) in enumerate(data):
                for (j, o) in enumerate(row):
                    if o > 9:
                        flashes += 1
                        flashed = True
                        data[i][j] = -10
                        for ii in range(max(i-1, 0), 1+min(len(data)-1, i+1)):
                            for jj in range(max(j-1, 0), 1+min(len(row)-1, j+1)):
                                data[ii][jj] += 1
        for (i, row) in enumerate(data):
            for (j, o) in enumerate(row):
                if o < 0 :
                    data[i][j] = 0
        # print('after step', step, '\n', data)

    print("Part 1:", flashes)
    # 858 too low

def part2(data):
    step = 0
    while True:
        step += 1
        for (i, row) in enumerate(data):
            for (j, o) in enumerate(row):
                data[i][j] += 1

        flashes = 0
        flashed = True
        while (flashed):
            flashed = False
            for (i, row) in enumerate(data):
                for (j, o) in enumerate(row):
                    if o > 9:
                        flashes += 1
                        flashed = True
                        data[i][j] = -10
                        for ii in range(max(i-1, 0), 1+min(len(data)-1, i+1)):
                            for jj in range(max(j-1, 0), 1+min(len(row)-1, j+1)):
                                data[ii][jj] += 1

        for (i, row) in enumerate(data):
            for (j, o) in enumerate(row):
                if o < 0 :
                    data[i][j] = 0
        # print('after step', step, '\n' + str(data))
        if flashes == 100:
            break

    print("Part 2:", step)
    # 224 too low


if __name__ == '__main__':
    data = parse()
    # part1(data)
    part2(data)
