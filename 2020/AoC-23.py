arr = '789465123'
# arr = '389125467' # P1: 67384529

def part1(circ, rep):
    cur = 0
    num = len(circ)
    for i in range(rep) :
        # print('\r{}%'.format(100*i/rep),end='')
        if num != len(circ):
            print("Error, different length")
            return

        # print('move:',i+1)
        # print(circ[:20])
        curEl = circ[cur]
        # print('i:',cur, '\tn:',curEl)

        elms = circ[cur+1:cur+4]
        del circ[cur+1:cur+4]
        if num < cur+4:
            elms += circ[0:cur+4-num]
            del circ[0:cur+4-num]

        # print(circ[:20])
        # print(elms)

        n = curEl-1
        while n not in circ:
            n = (n-1) % (num+1)

        ins = circ.index(n)
        # print('n:',n,'\tins:',ins)
        circ.insert(ins+1, elms[0])
        circ.insert(ins+2, elms[1])
        circ.insert(ins+3, elms[2])

        cur = (circ.index(curEl)+1)%num

    print()
    # print(circ[circ.index(1)-1:circ.index(1)+2])
    print(''.join([str(x) for x in circ[circ.index(1):]+circ[:circ.index(1)]]))


def part2(circ, rep):
    cur = 0
    num = len(circ)
    for i in range(rep) :
        # print('\r{}%'.format(100*i/rep),end='')
        if num != len(circ):
            print("Error, different length")
            return

        print('move:',i+1)
        print(circ[:20])
        curEl = circ[cur]
        print('i:',cur, '\tn:',curEl)

        elms = circ[cur+1:cur+4]
        del circ[cur+1:cur+4]
        if num < cur+4:
            elms += circ[0:cur+4-num]
            del circ[0:cur+4-num]

        print(circ[:20])
        print(elms)

        n = curEl-1
        while n not in circ:
            n = (n-1) % (num+1)

        ins = circ.index(n)
        print('n:',n,'\tins:',ins)
        circ.insert(ins+1, elms[0])
        circ.insert(ins+2, elms[1])
        circ.insert(ins+3, elms[2])

        cur = (circ.index(curEl)+1)%num

    print()
    print(circ[circ.index(1)-2:circ.index(1)+1])
    print(''.join([str(x) for x in circ[circ.index(1):]+circ[:circ.index(1)]]))

if __name__ == '__main__':
    circ = [int(x) for x in arr]

    print('Part 1')
    part1(circ, 100)

    # print('Part 2')
    # circ = [int(x) for x in arr]
    # for i in range(len(circ),1000000+1):
    #     circ.append(i)
    # part2(circ, 10000000)

# P1
# 35692784 too low

# P2
# 149245887792 too high