def setup():
    arr = '''LLLLLL.LLLLL.LLLL..LLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLL.LLLLL..LLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.L.LLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLL\n...L....L.LLL.L.L....LL..LL....L.L..L.....L....LLL..L....LL....LLLL.L.........L..LL..............\nLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL\nLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLL..LLLLLLL.LLLLLL..LLLLLLLLLLLL.LLLLLLLL..LLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLL\nLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLL...L.L..L.LL.L.L...L....L..L.LLL.L..L....L.L.L...LLL..L...LLL..L..LL.L.L.LL.LL..L...L......L.L\nLLLLLL.LLLL.LLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLL.LLLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLL..LLLLLLLLLLLLLL..LLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LL.LLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLL.LLLLLLL\n......L.L.L...LLLL.L....L...LL....LL.LL..L.L..L..L..LL.....L..LL...LLL...L.LL.L.L...L.L.......L..\nLLLLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLL..LLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLL.LLLLLLLLL\n..L...LL..L.LL..L.L.LL.LLLL......L...LLL..L.L.L..L...LL.LLL..L..L....L.L.LLL..L..L...LL...L.L..LL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.L.LLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLLLLL.LLLL.LL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLL\n..L.......L.LL..LL..L.L....LLL.L....LL...L.L...L.....L...LL..LL....LL.....LL.L....L..............\nLLLLLLLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLL.LL.LLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLL.LL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLL.......L...L..L.L.LL...LL...........LL.L...L.....LL.......LL....LLL.L.LLL..L.L.L.L...LL..LL....\nLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLL..LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nL....L.L.L...LL...LL.....L..L.LLLLLL.L...........L..LL.L....L.....LL..LL.L...L..LLL........L.LLL.\nLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL..LLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLL.L..................L..L..L..L..L...LL....LL.L......L..LL...LL...........LL....LL..L...L.......\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLL.LL.LLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.L.LLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLLL.LL.LLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLL..L.LLLLL\nL...L.LL.....L...L....L...L...L..L.L..L..L..LL.L.LL..L....L.L.L..L.L.L.LL......LL.L.L.LLLLL...L.L\nLLLL.L.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLL.LLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLL.L.LLLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL..LLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLLLLLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLL\n..L..L.....LLL.......LL....L...L..LLL..L.......L......L.L....L..L....LLL.LL.L.....LL...L..LLLL...\nLLLLLL.LLLLL.LLLLL.LL.LLLLLL.LLLLLLLLLLLLLLLL..LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLL.L.LLLL.LLLLLLL\nLLL.LL.LLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLL.LLL.LLL.LLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LL.LLLLLLLLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LL.LLLLLL..LLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLL.LLLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.L.LLLLLL.LLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLL.LLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL\n...LLLL.LL.L.L........LLL...L.LLL....L.L..LLL.L..L..L...L.L..L...LL..LL.L.LL..LLL.L....L....L.L..\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLL...LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLL.LLLLL.LLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLL.LLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLLLLL.LL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLL.LLLLLLLLLLL.LLLLLLLLL.LLLLLL.LLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLL..LLL.LLLLLLLLLLLLLLLLLLLL\n.....L.L.LL.....L.L.LL.L...L....LLL........LL.L....LLL..LLL.L..LL..L......L..L...L.L.L.....L.L...\nLLLLLLLLLLLLLLLLLL.LL.LLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLL.LLLLL..LLLLLLL\nLLLLLL.LLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL\nLLLLL..LLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLLLLLLLL.LLLLLLLLLLLLLL\nLLLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLL.L.LL.LL.LLLLLLLLLLLLLL\nLLLLLL.LLLLLLLLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLLLLLLLLLLL.LLLLL.LLLLLLLLLLLLLL\nLLLLLLLLLLLL.LLLLL.LLLLLLLLL.LL.LLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLLLL\nLLLLLL.LLLLL..LLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLLLLLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLLLLLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL\nLLLLLLLLLLLL.LLLL.LLLLLLLLLL.LLLLLLLLLLLLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLLLLLLLLLL\nLLLLLL.LLLLL.LLLLL.LLLLLLLLLLLLLLLLLLL.LLLLLLL.L.LLLLLLLLLLLLLLLLLLLLLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLLL.LL.LLLLLLLLLLLLLLLLLL.LLLLLLLLL.LLLLLLLLLLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLLLLLLLLL.LLLLLLL\nLLLLL.LLLLLL.LLLLL.LLLLLLLLL.LLLLLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLL.LLLLLLLL.LLLLL.LLLLLL.LLLLLLL'''

    arr = [[ch for ch in ar] for ar in arr.split('\n')]
    print(arr)
    return arr

def par1(arr):
    while True :
        # [print(''.join(ar)) for ar in arr]
        # print('='*100)
        change = False
        new = [ar.copy() for ar in arr]
        for i, ar in enumerate(arr):
            for j, a in enumerate(ar):
                if a == '.' :
                    continue
                occupy = 0
                rangX = range(max(0, i-1), min(len(arr), i+2))
                rangY = range(max(0, j - 1), min(len(ar), j + 2))
                # print(i, j, rangX, rangY)
                for x in rangX:
                    for y in rangY:
                        if x == i and y == j:
                            continue
                        # print(i, j, x, y)
                        if arr[x][y] == '#':
                            occupy += 1
                if occupy==0:
                    new[i][j] = '#'
                    if a == 'L':
                        change = True
                if 4 <= occupy:
                    new[i][j] = 'L'
                    if a== '#':
                        change = True
        if not change :
            break
        arr = new
    count = 0
    for ar in arr:
        for a in ar:
            if a == '#':
                count += 1

    print('Part 1:', count)

def par2(arr):
    while True :
        [print(''.join(ar)) for ar in arr]
        print('='*100)
        change = False
        new = [ar.copy() for ar in arr]
        for i, ar in enumerate(arr):
            for j, a in enumerate(ar):
                # print(i, j, a)
                if a == '.' :
                    continue
                occupy = 0
                rang = [-1, 0, 1]
                for x in rang:
                    for y in rang:
                        if x == y == 0:
                            continue
                        cx = i+x
                        cy = j+y
                        # print('\t', i, j, x, y)
                        while 0 <= cx < len(arr) and 0<= cy < len(arr[cx]) and arr[cx][cy]=='.':
                            # print('\t\t',cx, cy)
                            cx += x
                            cy += y
                        if 0 <= cx < len(arr) and 0<= cy < len(arr[cx]) and arr[cx][cy] == '#':
                            occupy += 1
                if occupy==0:
                    new[i][j] = '#'
                    if a == 'L':
                        change = True
                if 5 <= occupy:
                    new[i][j] = 'L'
                    if a== '#':
                        change = True
        if not change :
            break
        arr = new
    count = 0
    for ar in arr:
        for a in ar:
            if a == '#':
                count += 1

    print('Part 2:', count)

if __name__ == '__main__':
    par2(setup())