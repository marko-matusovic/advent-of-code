import numpy as np

def parse():
    data = open('input/5.d', 'r').read().split("\n")

    lines = []

    for dat in data:
        [pos1, pos2] = dat.split(" -> ")
        [x1, y1] = pos1.split(",")
        [x2, y2] = pos2.split(",")
        lines.append((int(x1), int(y1), int(x2), int(y2)))

    return lines

def part1(data):

    dim = 1000
    map = np.zeros(dim*dim).reshape([dim,dim])

    for line in data:
        (x1,y1,x2,y2) = line
        if x1==x2:
            for y in range(min(y1, y2), max(y1, y2)+1):
                map[x1][y] += 1
        elif y1==y2:
            for x in range(min(x1, x2), max(x1, x2)+1):
                map[x][y1] += 1
        else:
            continue

    summ = 0
    for row in map:
        for n in row:
            if n>=2:
                summ += 1

    print("Part 1:", summ)

def getlist(a, b):
    if a<b:
        return [x for x in range(a, b+1)]
    elif a>b:
        return [x for x in range(a, b-1, -1)]
    else:
        return [a]

def part2(data):

    dim = 1000
    map = np.zeros(dim*dim).reshape([dim,dim])

    for line in data:
        (x1,y1,x2,y2) = line
        if x1==x2:
            for y in range(min(y1, y2), max(y1, y2)+1):
                map[x1][y] += 1
        elif y1==y2:
            for x in range(min(x1, x2), max(x1, x2)+1):
                map[x][y1] += 1
        else:
            xx = getlist(x1, x2)
            yy = getlist(y1, y2)

            for i in range(len(xx)):
                map[xx[i]][yy[i]] += 1

    summ = 0
    for row in map:
        for n in row:
            if n>=2:
                summ += 1

    print("Part 2:", summ)

if __name__ == '__main__':
    data = parse()
    part1(data)
    part2(data)