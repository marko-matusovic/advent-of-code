[rules,myticket,alltickets] = open('input/16.data', 'r').read().split('\n\n')

rules = rules.split('\n')
myticket = myticket.split('\n')[1].split(',')
alltickets = [[int(num) for num in row.split(',')] for row in alltickets.split('\n')[1:]]

def part1():
    ruls = []
    for r in rules:
        [a,b] = r.split(': ')[1].split(' or ')
        [a1, a2] = a.split('-')
        [b1, b2] = b.split('-')
        ruls.append([range(int(a1),int(a2)+1),range(int(b1),int(b2)+1)])

    error = 0
    for tick in alltickets:
        for num in tick:
            match = False
            for r in ruls:
                if num in r[0] or num in r[1]:
                    match = True
                    break
            if not match:
                error += num

    print('Part 1', error)


def part2():
    ruls = []
    for r in rules:
        [a,b] = r.split(': ')[1].split(' or ')
        [a1, a2] = a.split('-')
        [b1, b2] = b.split('-')
        ruls.append([range(int(a1),int(a2)+1),range(int(b1),int(b2)+1)])

    i = 0
    while i < len(alltickets):
        next = True
        for num in alltickets[i]:
            match = False
            for r in ruls:
                if num in r[0] or num in r[1]:
                    match = True
                    break
            if not match:
                del alltickets[i]
                next = False
                break
        if next:
            i += 1

    posCanRul = []
    for col in range(len(ruls)):
        posCanRul.append([])
        for rul in range(len(ruls)):
            rulCan = True
            for row in alltickets:
                if row[col] not in ruls[rul][0] and row[col] not in ruls[rul][1]:
                    rulCan = False
                    break
            if rulCan:
                posCanRul[col].append(rul)

    pos = {}
    # idp = {}
    # for i in range(20):
    #     for j,p in enumerate(posCanRul):
    #         if len(p)!=i:
    #             continue
    #         for id in p:
    #             if id in pos.values():
    #                 continue
    #             pos[j] = id
    #             idp[id] = j
    #
    # print(pos)

    # [print(i+1, p) for i,p in enumerate(posCanRul)]
    for i in range(1, 21):
        for j,p in enumerate(posCanRul):
            if len(p) == i:
                print(j+1, p)
    pos = {
        20: 0,
        17: 1,
        11: 2,
        14: 3,
        12: 4,
        7:  5,
        9:  6,
        16: 7,
        3:  8,
        6:  9,
        1:  10,
        13: 11,
        5:  12,
        15: 13,
        4:  14,
        2:  15,
        10: 16,
        8:  17,
        18: 18,
        19: 19,
    }

    count = 1
    count *= int(myticket[20-1])
    count *= int(myticket[17-1])
    count *= int(myticket[11-1])
    count *= int(myticket[14-1])
    count *= int(myticket[12-1])
    count *= int(myticket[7-1])

    print('Part 2', count)

if __name__ == '__main__':
    part1()
    part2()