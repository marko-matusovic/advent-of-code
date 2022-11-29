def parse():
    data = '''7612648217
7617237672
2853871836
7214367135
1533365614
6258172862
5377675583
5613268278
8381134465
3445428733'''

    ## example
#     data = '''5483143223
# 2745854711
# 5264556173
# 6141336146
# 6357385478
# 4167524645
# 2176841721
# 6882881134
# 4846848554
# 5283751526'''

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
