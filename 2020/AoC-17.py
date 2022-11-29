def part1(arr):
    # arr = [[[True]]]
    for iteration in range(6):
        # print('='*50)
        # print('i =',iteration)
        #
        # for k,z in enumerate(arr):
        #     print('z =',k)
        #     for y in z:
        #         for x in y:
        #             print('#' if x else '.', end='')
        #             # print(x,end=' ')
        #         print()

        brr = []
        brr.append([[False] * (2 + len(arr[0][0]))] * (2 + len(arr[0])))
        for z in arr:
            brr.append([])
            brr[-1].append([False] * (2 + len(arr[0][0])))
            for y in z:
                brr[-1].append([])
                brr[-1][-1].append(False)
                for x in y:
                    brr[-1][-1].append(x)
                brr[-1][-1].append(False)
            brr[-1].append([False] * (2 + len(arr[0][0])))
        brr.append([[False] * (2 + len(arr[0][0]))] * (2 + len(arr[0])))

        arr = brr

        brr = []
        for z in arr:
            brr.append([])
            for y in z:
                brr[-1].append([])
                for x in y:
                    brr[-1][-1].append(x)

        for k, z in enumerate(brr):
            for j, y in enumerate(z):
                for i, x in enumerate(y):
                    nc = 0
                    for kk in range(-1, 2):
                        kpos = k + kk
                        if kpos < 0 or len(arr) <= kpos:
                            continue
                        for jj in range(-1, 2):
                            jpos = j + jj
                            if jpos < 0 or len(arr[0]) <= jpos:
                                continue
                            for ii in range(-1, 2):
                                ipos = i + ii
                                if ipos < 0 or len(arr[0][0]) <= ipos or kk == jj == ii == 0:
                                    continue
                                # print(k,j,i,kpos,jpos,ipos)
                                if arr[kpos][jpos][ipos] == True:
                                    nc += 1
                    # brr[k][j][i] = nc
                    if nc == 3:
                        brr[k][j][i] = True
                    elif nc == 2:
                        1  # do nothing
                    else:
                        brr[k][j][i] = False
        arr = brr

    count = 0
    for ar in arr:
        for a in ar:
            for i in a:
                if i:
                    count += 1
    print('Part1:', count)

def part2(arr):
    arr = [arr]
    for iteration in range(6):

        brr = []
        brr.append([[[False] * (2 + len(arr[0][0][0]))] * (2 + len(arr[0][0]))] * (2 + len(arr[0])))
        for w in arr:
            brr.append([])
            brr[-1].append([[False] * (2 + len(arr[0][0][0]))] * (2 + len(arr[0][0])))
            for z in w:
                brr[-1].append([])
                brr[-1][-1].append([False] * (2 + len(arr[0][0][0])))
                for y in z:
                    brr[-1][-1].append([])
                    brr[-1][-1][-1].append(False)
                    for x in y:
                        brr[-1][-1][-1].append(x)
                    brr[-1][-1][-1].append(False)
                brr[-1][-1].append([False] * (2 + len(arr[0][0][0])))
            brr[-1].append([[False] * (2 + len(arr[0][0][0]))] * (2 + len(arr[0][0])))
        brr.append([[[False] * (2 + len(arr[0][0][0]))] * (2 + len(arr[0][0]))] * (2 + len(arr[0])))
        arr = brr

        brr = []
        for w in arr:
            brr.append([])
            for z in w:
                brr[-1].append([])
                for y in z:
                    brr[-1][-1].append([])
                    for x in y:
                        brr[-1][-1][-1].append(x)

        for l, w in enumerate(brr):
            for k, z in enumerate(w):
                for j, y in enumerate(z):
                    for i, x in enumerate(y):
                        nc = 0
                        for ll in range(-1, 2):
                            lpos = l + ll
                            if lpos < 0 or len(arr) <= lpos:
                                continue
                            for kk in range(-1, 2):
                                kpos = k + kk
                                if kpos < 0 or len(arr[0]) <= kpos:
                                    continue
                                for jj in range(-1, 2):
                                    jpos = j + jj
                                    if jpos < 0 or len(arr[0][0]) <= jpos:
                                        continue
                                    for ii in range(-1, 2):
                                        ipos = i + ii
                                        if ipos < 0 or len(arr[0][0][0]) <= ipos or ll == kk == jj == ii == 0:
                                            continue
                                        if arr[lpos][kpos][jpos][ipos] == True:
                                            nc += 1
                        # brr[k][j][i] = nc
                        if nc == 3:
                            brr[l][k][j][i] = True
                        elif nc == 2:
                            1  # do nothing
                        else:
                            brr[l][k][j][i] = False

        arr = brr

    count = 0
    for w in arr:
        for z in w:
            for y in z:
                for x in y:
                    if x:
                        count += 1
    print('Part2:', count)


if __name__ == '__main__':
    arr = '..#..#.#\n##.#..#.\n#....#..\n.#..####\n.....#..\n...##...\n.#.##..#\n.#.#.#.#'
    # arr = '.#.\n..#\n###'
    arr = [[[a == '#' for a in ar] for ar in arr.split('\n')]]
    part1(arr)
    part2(arr)
