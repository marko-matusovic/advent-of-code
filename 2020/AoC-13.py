arr = '''1002462
37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,601,x,x,x,x,x,x,x,x,x,x,x,19,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,29,x,443,x,x,x,x,x,x,x,x,x,x,x,x,13'''.split('\n')

# arr = '''0
# 17,x,13,19'''.split()
#
# arr = '''0
# 1789,37,47,1889'''.split()

def part1():
    est = int(arr[0])
    buses = []
    for x in arr[1].split(','):
        if x != 'x':
            buses.append(int(x))
    minB = 0
    minV = (buses[0] - (est % buses[0])) % buses[0]
    for b in buses[1:]:
        dep = (b - (est % b)) % b
        if dep < minV:
            minV = dep
            minB = b
    print('Part 1:', minV*minB)

def part2():
    # est = int(arr[0])
    buses = {}
    for i,x in enumerate(arr[1].split(',')):
        if x != 'x':
            buses[i] = int(x)

    print(buses)

    t = 0
    looking = True
    maxC = 0
    addT = 1
    while looking:
        looking = False
        c = 0
        for n in buses:
            b = buses[n]
            if (t+n) % b != 0:
                looking = True
                break
            else:
                c += 1
        if maxC == c:
            addT = t-prevT
            prevT = t
        elif maxC < c:
            prevT = t
            maxC = c
        print(t, c, addT)
        t += addT

    print('Part 2:', t-addT)

if __name__ == '__main__':
    part2()
