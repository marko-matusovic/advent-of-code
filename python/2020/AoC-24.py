arr = open('input/24.d', 'r').read()

def hash(x,y,z):
    return '{}:{}:{}'.format(x, y, z)

def dehash(h):
    [x,y,z] = [int(x) for x in h.split(':')]
    return (x,y,z)

def part1(moves):
    tiles = set([])
    for move in moves:
        x=y=z = 0
        for m in move:
            if m == 'w':
                x -= 1
                y += 1
            elif m == 'e':
                x += 1
                y -= 1
            elif m == 'sw':
                x -= 1
                z += 1
            elif m == 'se':
                y -= 1
                z += 1
            elif m == 'nw':
                y += 1
                z -= 1
            elif m == 'ne':
                x += 1
                z -= 1
            else:
                print('Error, unknown direction',m)
                return
        tile = hash(x,y,z)
        if tile in tiles:
            tiles.remove(tile)
        else:
            tiles.add(tile)
    print('Part 1:', len(tiles))
    return tiles

import time

def part2(tiles):

    neighs = [(0,  1, -1), (-1, 0,  1), ( 1, -1, 0),
              (0, -1,  1), ( 1, 0, -1), (-1,  1, 0)]

    for day in range(100):
        # time.sleep(0.1)
        neighCount = {}
        for t in tiles:
            neighCount[t] = 0
        for tile in tiles:
            (x,y,z) = dehash(tile)
            # print(hash(x,y,z))
            for (dx,dy,dz) in neighs:
                n = hash(x+dx, y+dy, z+dz)
                # print('\t',hash(dx,dy,dz),n)
                if n not in neighCount:
                    neighCount[n] = 0
                neighCount[n] += 1
            # print(neighCount)

        for n in neighCount:
            if neighCount[n] == 2:
                tiles.add(n)
            elif neighCount[n] == 1:
                1 # no change
            else:
                if n in tiles:
                    tiles.remove(n)

        print('day:',day+1,'\tcount:',len(tiles))


if __name__ == '__main__':
    moves = []
    for ar in arr.split('\n'):
        i = 0
        moves.append([])
        while i<len(ar):
            moves[-1].append(ar[i])
            i += 1
            if moves[-1][-1]=='e' or moves[-1][-1]=='w':
                continue
            moves[-1][-1] += ar[i]
            i += 1

    tiles = part1(moves)

    part2(tiles)


# P2
# 4492 too high





