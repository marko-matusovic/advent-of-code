import numpy as np

TPos = tuple[int, int]
TCube = tuple[TPos, TPos, TPos]
TData = list[tuple[str, TCube]]


def parse() -> TData:
    raw = open('input/22.data', 'r').read()

    data = []

    for line in raw.split("\n"):
        (instruction, coords) = line.split(" ")
        (x, y, z) = coords.split(",")
        xi = tuple(int(i) for i in x[2:].split(".."))
        yi = tuple(int(i) for i in y[2:].split(".."))
        zi = tuple(int(i) for i in z[2:].split(".."))
        data.append((instruction, (xi, yi, zi)))

    return data


def part1(data: TData):
    print("Part 1:")

    cubes = np.full((101, 101, 101), False)

    for ins, cube in data:
        ((x0, x1), (y0, y1), (z0, z1)) = cube
        for x in range(max(x0, -50), min(x1, 50) + 1):
            for y in range(max(y0, -50), min(y1, 50) + 1):
                for z in range(max(z0, -50), min(z1, 50) + 1):
                    cubes[x - 50][y - 50][z - 50] = ins == "on"

    print(f"Answer: {cubes.sum()}")


def part2(data: TData):
    print("Part 2:")

    Xs = []
    Ys = []
    Zs = []

    for ins, cube in data:
        ((x0, x1), (y0, y1), (z0, z1)) = cube
        Xs.extend([x0, x1 + 1])
        Ys.extend([y0, y1 + 1])
        Zs.extend([z0, z1 + 1])

    Xs = sorted(list(set(Xs)))
    Ys = sorted(list(set(Ys)))
    Zs = sorted(list(set(Zs)))

    Xpos = {x: i for (i, x) in enumerate(Xs)}
    Ypos = {y: i for (i, y) in enumerate(Ys)}
    Zpos = {z: i for (i, z) in enumerate(Zs)}

    cubes = np.full((len(Xs) - 1, len(Ys) - 1, len(Zs) - 1), False)

    i = 0
    for ins, cube in data:
        print(f"ins: {i}\r")
        i += 1
        ((x0, x1), (y0, y1), (z0, z1)) = cube
        xi0 = Xpos[x0]
        xi1 = Xpos[x1 + 1]
        yi0 = Ypos[y0]
        yi1 = Ypos[y1 + 1]
        zi0 = Zpos[z0]
        zi1 = Zpos[z1 + 1]
        for x in range(xi0, xi1):
            for y in range(yi0, yi1):
                for z in range(zi0, zi1):
                    cubes[x][y][z] = ins == "on"

    sum = 0
    for xi, x in enumerate(Xs[:-1]):
        print(f"xi: {xi}\r")
        for yi, y in enumerate(Ys[:-1]):
            for zi, z in enumerate(Zs[:-1]):
                if cubes[xi][yi][zi]:
                    sum += (Xs[xi + 1] - x) * (Ys[yi + 1] - y) * (Zs[zi + 1] - z)

    print(f"Answer: {sum}")


if __name__ == "__main__":
    part1(parse())
    part2(parse())
