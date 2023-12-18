def edge(edge, rot, flip):
    rot = rot % 4
    res = edge[rot*9 : rot*9+10]
    if flip :
        return res[::-1]
    return res

def xor(a, b):
    return (a or b) and not (a and b)

def rotate(arr, n):
    for i in range(n%4):
        s = len(arr)
        brr = [[[] for j in range(s)] for i in range(s)]
        for x in range(s):
            for y in range(s):
                brr[x][y] = arr[s-y-1][x]
        arr = brr
    return arr

def flip(arr):
    return arr[::-1]

def transform(arr, rot, flp):
    arr = rotate(arr, rot)
    if flp:
        arr = flip(arr)
    return arr

def part1(puzzles):

    turns = [(0,False), (1,False), (2,False), (3,False), (0,True), (1,True), (2,True), (3,True)]

    neighs = {}
    for p in puzzles.keys():
        neighs[p] = set([])

    for p in puzzles.keys():
        pbor = puzzles[p]
        for q in puzzles.keys():
            if p==q :
                continue
            qbor = puzzles[q]
            for ti in turns:
                for tj in turns:
                    if edge(pbor, ti[0], ti[1]) == edge(qbor, tj[0], tj[1]):
                        neighs[p].add(q)
                        neighs[q].add(p)

    res = 1
    for n in neighs.keys():
        if len(neighs[n]) == 2:
            res *= n

    print('Part 1:', res)

def part2(puzzles, pieces):

    turns = [(0,False), (1,False), (2,False), (3,False), (0,True), (1,True), (2,True), (3,True)]

    neighs = {}
    for p in puzzles.keys():
        neighs[p] = {}

    for p in puzzles.keys():
        pbor = puzzles[p]
        for q in puzzles.keys():
            if p==q :
                continue
            qbor = puzzles[q]
            for ti in turns[:4]: # no flip -> avoid duplicates
                for tj in turns:
                    if edge(pbor, ti[0], ti[1]) == edge(qbor, tj[0], tj[1])[::-1]:
                        neighs[p][q] = (ti, tj)

    corner = None
    for n in neighs.keys():
        if len(neighs[n])==2:
            corner = n
            break

    # for m in neighs[corner].keys():
    #     print(neighs[corner][m])

    size = 12

    image = [[[] for i in range(size)] for j in range(size)]
    image[0][0] = (corner, 2, False)

    # first row
    for i in range(1,size):
        (prevId, prevRot, prevFlip) = image[0][i-1]
        (nextId, nextRot, nextFlip) = (None, None, None)
        for m in neighs[prevId].keys():
            if (neighs[prevId][m][0][0] + prevRot)%4 == 1 :
                nextId = m
                nextRot = 3 - neighs[prevId][m][1][0]
                nextFlip = xor(prevFlip, neighs[prevId][m][1][1])
                # nextFlip = neighs[prevId][m][1][1]
                # print(prevFlip, neighs[prevId][m][1][1], nextFlip)
                break
        if nextId == None:
            print('No match found in first row at cell:', i)
        image[0][i] = (nextId, nextRot, nextFlip)

    # next rows
    for i in range(1,12):
        # first cell
        (prevId, prevRot, prevFlip) = image[i-1][0]
        (nextId, nextRot, nextFlip) = (None, None, None)
        for m in neighs[prevId].keys():
            if (neighs[prevId][m][0][0] + prevRot + 2*prevFlip) % 4 == 2: # match north
                nextId = m
                nextFlip = xor(prevFlip, neighs[prevId][m][1][1])
                nextRot = (4 - neighs[prevId][m][1][0] + 2*nextFlip) % 4
                break
        if nextId == None:
            print('No match found in row:', i, 'at first cell')
        image[i][0] = (nextId, nextRot, nextFlip)
        # next cells
        for j in range(1, 12):
            (prevId, prevRot, prevFlip) = image[i][j - 1]
            (nextId, nextRot, nextFlip) = (None, None, None)
            for m in neighs[prevId].keys():
                if (neighs[prevId][m][0][0] + prevRot) % 4 == 1: # match west & ignore north?
                    nextId = m
                    nextRot = 3 - neighs[prevId][m][1][0]
                    nextFlip = xor(prevFlip, neighs[prevId][m][1][1])
                    break
            if nextId == None:
                print('No match found in row:', i, 'at cell:', j)
            image[i][j] = (nextId, nextRot, nextFlip)



    # # print placement of puzzles
    # for y in image:
    #     for x in y:
    #         print('{:17}'.format(x.__str__()), end='')
    #     print()

    # # show merged puzzles
    # for y in image:
    #     for line in range(10):
    #         for (id, rot, flip) in y:
    #             transformed = transform(pieces[id], rot, flip)
    #             print(''.join(transformed[line]),end='|')
    #         print()
    #     print('-'*12*11)


    photo = []
    for y in image:
        for line in range(1,9):
            photo.append([])
            for (id, rot, cflip) in y:
                transformed = transform(pieces[id], rot, cflip)
                for ch in transformed[line][1:-1]:
                    photo[-1].append(ch)

    monster = '..................#.\n#....##....##....###\n.#..#..#..#..#..#...'
    pos = []
    for i,y in enumerate(monster.split('\n')):
        for j,x in enumerate(y):
            if x == '#':
                pos.append((i,j))
    h = 3
    w = 20

    for k in range(4):
        for y,line in enumerate(photo):
            if len(photo)<=y+h:
                break
            for x,cell in enumerate(line):
                if len(line)<=x+w:
                    break
                found = True
                for p in pos:
                    y2 = y+p[0]
                    x2 = x+p[1]
                    if photo[y2][x2] == '.':
                        found = False
                        break
                if found:
                    for p in pos:
                        y2 = y + p[0]
                        x2 = x + p[1]
                        photo[y2][x2] = 'O'
        photo = rotate(photo, 1)
    photo = flip(photo)
    for k in range(4):
        for y,line in enumerate(photo):
            if len(photo)<=y+h:
                break
            for x,cell in enumerate(line):
                if len(line)<=x+w:
                    break
                found = True
                for p in pos:
                    y2 = y+p[0]
                    x2 = x+p[1]
                    if photo[y2][x2] == '.':
                        found = False
                        break
                if found:
                    for p in pos:
                        y2 = y + p[0]
                        x2 = x + p[1]
                        photo[y2][x2] = 'O'
        photo = rotate(photo, 1)

    count1 = 0
    count2 = 0
    for y in photo:
        for x in y:
            if x == '#':
                count1 += 1
            elif x == 'O':
                count2 += 1

    print('\n'.join([''.join(x) for x in photo]))
    print('Part 2:', count1)

if __name__ == '__main__':
    arr = open('input/20.data', 'r').read().split('\n\n')

    puzzles = {}
    pieces = {}
    for ar in arr:
        ar = ar.split('\n')

        border = []
        for i in range(10):
            border.append(ar[1][i])
        for i in range(9):
            border.append(ar[2+i][9])
        for i in range(9):
            border.append(ar[10][8-i])
        for i in range(9):
            border.append(ar[9-i][0])

        num = int(ar[0].split(' ')[1][:-1])
        puzzles[num] = ''.join(border)
        pieces[num] = [[x for x in a] for a in ar[1:]]
    # part1(puzzles)
    part2(puzzles, pieces)


# 2567 too high