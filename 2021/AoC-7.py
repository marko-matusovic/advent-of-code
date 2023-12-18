import numpy as np

def parse():
    data = open('input/7.data', 'r').read()

    return np.array([int(n) for n in data.split(',')], np.int32)

def part1(data):
    goal = np.average(data)

    minfuel = 1e99

    for i in range(0, np.max(data)):
        fuel = np.sum(np.abs(data - i))
        if fuel < minfuel:
            goal = i
            minfuel = fuel

    print("Part 1:", goal, minfuel)

    # 458 too low
    # 350644 too high

def cost(dist):
    return (dist + 1) * (dist / 2.0)

def part2(data):
    goal = np.average(data)

    minfuel = 1e99

    for i in range(0, np.max(data)):
        fuel = np.sum(cost(np.abs(data - i)))
        if fuel < minfuel:
            goal = i
            minfuel = fuel

    print("Part 2:", goal, minfuel)

if __name__ == '__main__':
    data = parse()

    part1(data)
    part2(data)