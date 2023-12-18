import numpy as np

def parse():
    data = open('input/4', 'r').read().split("\n\n")

    nums = [int(x) for x in data[0].split(",")]

    boards = [[[int(n) for n in row.split()] for row in board.split("\n")] for board in data[1:]]

    for i in range(len(boards)):
        boards[i] = np.array(boards[i])

    return (nums, boards)


def check4win(board, draw):
    for row in board:
        win = True
        for n in row:
            if n not in draw:
                win = False
                break
        if win:
            return True

    for collumn in board.T:
        win = True
        for n in collumn:
            if n not in draw:
                win = False
                break
        if win:
            return True

    return False

def finScore(board, draw):
    summ = 0
    for row in board:
        for n in row:
            if n not in draw:
                summ += n
    return summ * draw[-1]


def part1(data):

    (nums, boards) = data

    draw = []

    win = False

    for n in nums:
        draw.append(n)
        for b in boards:
            if check4win(b, draw):
                win = True
                winBoard = b
                break
        if win:
            break

    print("Part 1:", finScore(winBoard, draw))

def part2(data):
    (nums, boards) = data

    draw = []

    won = []

    for n in nums:
        draw.append(n)
        for (i, b) in enumerate(boards):
            if i in won:
                continue
            if check4win(b, draw):
                lastWin = finScore(b, draw)
                won.append(i)

    print("Part 2:", lastWin)

if __name__ == '__main__':
    data = parse()

    part1(data)
    part2(data)