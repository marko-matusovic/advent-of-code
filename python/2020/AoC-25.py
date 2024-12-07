arr = '2959251;4542595'

def part1(door, card):
    a = 1
    dls = 0
    while a != door:
        a *= 7
        a %= 20201227
        dls += 1
    print()

    a = 1
    cls = 0
    while a != card:
        a *= 7
        a %= 20201227
        cls += 1

    a = 1
    for i in range(dls):
        a *= card
        a %= 20201227
    res = a

    a = 1
    for i in range(cls):
        a *= door
        a %= 20201227
    if a == res:
        return (res, dls, cls)

if __name__ == '__main__':
    door = int(arr.split(';')[0])
    card = int(arr.split(';')[1])

    (private, dls, cls) = part1(door, card)
    print("Part 1", private)





