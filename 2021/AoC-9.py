def parse():
    data = open('input/9.data', 'r').read()

    return ([[int(x) for x in row] for row in data.split('\n')], data.splitlines())

def part1(data):

    summ = 0
    lows = []

    for (j, row) in enumerate(data):
        for (i, d) in enumerate(row):
            if 0 < i          and row[i-1] <= d:
                continue
            if i < len(row)-1 and row[i+1] <= d:
                continue
            if 0 < j          and data[j-1][i] <= d:
                continue
            if j < len(data)-1 and data[j+1][i] <= d:
                continue
            summ += d + 1
            # print(j, i, d+1)
            lows.append((i,j))

    print("Part 1:", summ)
    return lows

def part2(data, lows, lines):
    data = lines
    taken = [[False] * len(data[0]) for _ in range(len(data))]
    neighbors = []
    basins = []
    basin = -1
    while not all([all(line) for line in taken]):
        if len(neighbors) == 0:
            i, j = 0, 0
            while taken[i][j]:
                i += 1
                if i == len(data):
                    i = 0
                    j += 1
            basin += 1
            basins.append(0)
        else:
            i, j = neighbors.pop()

        if taken[i][j]:
            continue

        taken[i][j] = True

        if data[i][j] == "9":
            continue

        basins[basin] += 1

        if i != 0 and not taken[i - 1][j]:
            neighbors.append((i - 1, j))
        if i != len(data) - 1 and not taken[i + 1][j]:
            neighbors.append((i + 1, j))
        if j != 0 and not taken[i][j - 1]:
            neighbors.append((i, j - 1))
        if j != len(data[i]) - 1 and not taken[i][j + 1]:
            neighbors.append((i, j + 1))

    basins.sort(reverse=True)

    print("Part 2:", basins[0] * basins[1] * basins[2])

if __name__ == '__main__':
    (data, lines) = parse()
    lows = part1(data)
    part2(data, lows, lines)