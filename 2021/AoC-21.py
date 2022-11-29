def parse():
    raw = '''Player 1 starting position: 5
Player 2 starting position: 6'''

    return (5, 6)
    # return (4, 8)  # 1 -> 739785 # 2 -> 444356092776315


def part1(data):
    (a, b) = data

    rolls = 0
    sa = 0
    sb = 0
    turn = True
    while sa < 1000 and sb < 1000:
        move = 0
        for i in range(3):
            rolls += 1
            move += (rolls - 1) % 100 + 1
        if turn:
            a = (a + move - 1) % 10 + 1
            sa += a
        else:
            b = (b + move - 1) % 10 + 1
            sb += b
        turn = not turn

    print("Part 1:", min(sa, sb) * rolls)

    # 69696 too low


def part2(data):
    (a, b) = data

    game = {(a, b, 0, 0, True): 1}

    win_a = 0
    win_b = 0

    while len(game) > 0:
        g = list(game.keys())[0]
        count = game[g]
        del game[g]
        (a, b, sa, sb, p) = g
        if sa >= 21:
            win_a += count
            continue
        if sb >= 21:
            win_b += count
            continue
        for i in range(1, 4):
            for j in range(1, 4):
                for k in range(1, 4):
                    die = i + j + k
                    if p:
                        na = (a + die - 1) % 10 + 1
                        ng = (na, b, sa + na, sb, False)
                    else:
                        nb = (b + die - 1) % 10 + 1
                        ng = (a, nb, sa, sb + nb, True)
                    if not ng in game:
                        game[ng] = 0
                    game[ng] += count

    print("Part 2:", max(win_a, win_b))


if __name__ == '__main__':
    part1(parse())
    part2(parse())
