import numpy as np
import heapq


def parse():
    raw = open('input/15', 'r').read()

    data = [[int(x) for x in row] for row in raw.split('\n')]

    return data


def part1(data):

    costs = data

    found = []
    for i in range(len(costs)):
        found.append([])
        for j in range(len(costs[i])):
            found[i].append(('', np.inf))

    queue = [(0, (0, 0))] # (cost, (x, y))

    while True:

        (cost, (x, y)) = heapq.heappop(queue)

        if found[x][y][1] < cost:
            continue

        if (x,y) == (len(costs)-1, len(costs[0])-1):
            break

        if x > 1 and cost + costs[x - 1][y] < found[x - 1][y][1]:
            found[x - 1][y] = ((x, y), cost + costs[x - 1][y])
            heapq.heappush(queue, (found[x - 1][y][1], (x - 1, y)))
        if x < len(costs[0]) - 1 and cost + costs[x + 1][y] < found[x + 1][y][1]:
            found[x + 1][y] = ((x, y), cost + costs[x + 1][y])
            heapq.heappush(queue, (found[x + 1][y][1], (x + 1, y)))
        if y > 1 and cost + costs[x][y - 1] < found[x][y - 1][1]:
            found[x][y - 1] = ((x, y), cost + costs[x][y - 1])
            heapq.heappush(queue, (found[x][y - 1][1], (x, y - 1)))
        if y < len(costs) - 1 and cost + costs[x][y + 1] < found[x][y + 1][1]:
            found[x][y + 1] = ((x, y), cost + costs[x][y + 1])
            heapq.heappush(queue, (found[x][y + 1][1], (x, y + 1)))

    print("Part 1:", found[len(costs)-1][len(costs[0])-1][1])


def part2(data):

    costs = []

    for i in range(5 * len(data)):
        costs.append([])
        for j in range(5 * len(data[0])):
            costs[i].append((int(i/len(data))+int(j/len(data[0]))+data[i%len(data[0])][j%len(data[0])]-1)%9+1)

    # print(costs)

    found = []
    for i in range(len(costs)):
        found.append([])
        for j in range(len(costs[i])):
            found[i].append(('', np.inf))

    queue = [(0, (0, 0))] # (cost, (x, y))

    while True:

        (cost, (x, y)) = heapq.heappop(queue)

        if found[x][y][1] < cost:
            continue

        if (x,y) == (len(costs)-1, len(costs[0])-1):
            break

        if x > 1 and cost + costs[x - 1][y] < found[x - 1][y][1]:
            found[x - 1][y] = ((x, y), cost + costs[x - 1][y])
            heapq.heappush(queue, (found[x - 1][y][1], (x - 1, y)))
        if x < len(costs[0]) - 1 and cost + costs[x + 1][y] < found[x + 1][y][1]:
            found[x + 1][y] = ((x, y), cost + costs[x + 1][y])
            heapq.heappush(queue, (found[x + 1][y][1], (x + 1, y)))
        if y > 1 and cost + costs[x][y - 1] < found[x][y - 1][1]:
            found[x][y - 1] = ((x, y), cost + costs[x][y - 1])
            heapq.heappush(queue, (found[x][y - 1][1], (x, y - 1)))
        if y < len(costs) - 1 and cost + costs[x][y + 1] < found[x][y + 1][1]:
            found[x][y + 1] = ((x, y), cost + costs[x][y + 1])
            heapq.heappush(queue, (found[x][y + 1][1], (x, y + 1)))

    print("Part 2:", found[len(costs)-1][len(costs[0])-1][1])


if __name__ == '__main__':
    part1(parse())
    part2(parse())
