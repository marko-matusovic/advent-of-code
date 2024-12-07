arr = open('input/21.d', 'r').read()

def part1(cookbook):
    ati = {}
    for (ingredients, alergens) in cookbook:
        for a in alergens:
            if a not in ati:
                ati[a] = []
            ati[a].append(set(ingredients))

    # for a in ati:
    #     print(a, ati[a])

    pati = {}

    for a in ati:
        res = ati[a][0].copy()
        for i in ati[a][1:]:
            res = res & i
        pati[a] = res.copy()

    res = set([])
    for a in pati:
        for i in pati[a]:
            res.add(i)

    count = 0
    for (ingredients, alergens) in cookbook:
        for i in ingredients:
            if i not in res:
                count += 1

    print('Part 1:', count)

def part2(cookbook):
    ati = {}
    for (ingredients, alergens) in cookbook:
        for a in alergens:
            if a not in ati:
                ati[a] = []
            ati[a].append(set(ingredients))

    pati = {}

    for a in ati:
        res = ati[a][0].copy()
        for i in ati[a][1:]:
            res = res & i
        pati[a] = res.copy()

    for a in pati:
        print(a, pati[a])

    print('='*20)

    for i in range(len(res)):
        for a in pati:
            if len(pati[a])==1:
                el = pati[a].pop()
                for b in pati:
                    if el in pati[b]:
                        pati[b].remove(el)
                pati[a].add(el)
                break

    for a in pati:
        print(a, pati[a])

    print('Part 2:', 'vcckp,hjz,nhvprqb,jhtfzk,mgkhhc,qbgbmc,bzcrknb,zmh')

if __name__ == '__main__':

    cookbook = []
    for a in arr.split('\n'):
        ingredients = a.split(' (contains ')[0].split(' ')
        alergens = a.split(' (contains ')[1][:-1].split(', ')
        cookbook.append((ingredients, alergens))

    part1(cookbook)
    part2(cookbook)

# 192 too low



















