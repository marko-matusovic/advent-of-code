arr = [int(x) for x in '20,0,1,11,6,3'.split(',')]

def part1():
    list = arr.copy()
    occur = {}
    for i,a in enumerate(arr):
        occur[a] = [i+1]

    i = 7
    while i<=2020 :
        last = occur[list[-1]][-2:]
        if len(last)<2:
            list.append(0)
            occur[0].append(i)
        else:
            num = last[1] - last[0]
            list.append(num)
            if num not in occur:
                occur[num] = []
            occur[num].append(i)
        i += 1

    print('Part 1:', list[2020-1])

def part2():
    list = arr.copy()
    occur = {}
    for i, a in enumerate(arr):
        occur[a] = [i + 1]

    i = 7
    while i <= 30000000:
        print('\r')
        last = occur[list[-1]][-2:]
        if len(last) < 2:
            list.append(0)
            occur[0].append(i)
        else:
            num = last[1] - last[0]
            list.append(num)
            if num not in occur:
                occur[num] = []
            occur[num].append(i)
        i += 1

    print('Part 2:',list[30000000-1])

if __name__ == '__main__':
    part1()
    part2()