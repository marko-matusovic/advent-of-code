arr = open('input/6.data', 'r').read().split('\n\n')

totCount = 0
for g in arr:
    count = []
    for p in g.split():
        for a in p:
            count.append(a)
    totCount += len(set(count))

print('Part 1:', totCount)


totCount = 0
for g in arr:
    sett = []
    for a in g.split()[0]:
        sett.append(a)
    sett = set(sett)
    for p in g.split():
        sett2 = []
        for a in p:
            sett2.append(a)
        sett = sett.intersection(set(sett2))
    totCount += len(sett)

print('Part 2:', totCount)
