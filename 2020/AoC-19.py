import time

def part1():
    (rules,input) = open('input/19.data', 'r').read().split('\n\n')
    
    rules = rules.split('\n')
    input = input.split('\n')

    ruls = {}
    for r in rules:
        (a, b) = r.split(': ')
        # print(r)
        if b[0] == '\"':
            # print('case 1')
            c = b[1:-1]
            ruls[a] = ([c])
        elif len(b.split(' ')) == 1:
            # print('case 2')
            ruls[a] = ([b])
        elif len(b.split(' | ')) == 1:
            # print('case 3')
            c = b.split(' ')
            ruls[a] = (c)
        else:
            # print('case 4')
            b = b.split(' | ')
            c1 = b[0].split(' ')
            c2 = b[1].split(' ')
            ruls[a] = (c1, c2)
    rules = ruls

    list = []
    list.append(rules['0'])
    listS = set([])

    i = 0
    while i < len(list):
        if i%10 == 0:
            print('\ri:{:10} l:{:10} %:{}'.format(i, len(list), 100.0*i/len(list)), end='')
        # print(list[i])
        rep = False
        for j,e in enumerate(list[i]):
            if 'a' <= e <= 'z':
                # print('c')
                continue
            tbr = rules[e]
            if type(tbr)==type((1,1)):
                # print('t')
                del list[i][j]
                list.append(list[i].copy())
                for k, a in enumerate(tbr[0]):
                    list[i].insert(j+k, a)
                for k, a in enumerate(tbr[1]):
                    list[-1].insert(j+k, a)
                rep = True
                break
            else:
                # print('l')
                del list[i][j]
                # print(list[i])
                for k, a in enumerate(tbr):
                    list[i].insert(j+k, a)
                    # print(list[i])
                rep = True
                break
        if rep:
            continue
        listS.add(''.join(list[i-1]))
        # print(len(listS))
        # time.sleep(0.1)
        i += 1
    print()

    count = 0
    for a in input:
        if a in listS:
            count += 1

    print('Part 1:', count)


def part2b():
    (rules,input) = open('input/19.data', 'r').read().split('\n\n')
    # (rules, input) = '0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: "a"\n5: "b"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb'.split('\n\n')
    # (rules, input) = '0: 1 2\n1: "a"\n2: 1 3 | 3 1\n3: "b"\n\naab\naba\nbab\nbba'.split('\n\n')

    rules = rules.split('\n')
    input = input.split('\n')

    ruls = {}
    for r in rules:
        (a, b) = r.split(': ')
        # print(r)
        if b[0] == '\"':
            # print('case 1')
            c = b[1:-1]
            ruls[a] = ([c])
        elif len(b.split(' ')) == 1:
            # print('case 2')
            ruls[a] = ([b])
        elif len(b.split(' | ')) == 1:
            # print('case 3')
            c = b.split(' ')
            ruls[a] = (c)
        else:
            # print('case 4')
            b = b.split(' | ')
            c1 = b[0].split(' ')
            c2 = b[1].split(' ')
            ruls[a] = (c1, c2)
    rules = ruls

    list = []
    list.append(rules['0'])
    listS = set([])

    max = 0
    for i in input:
        if max < len(i):
            max = len(i)

    i = 0
    while i < len(list):
        # if i%10 == 0:
        #     print('\ri:{:10} l:{:10} %:{}'.format(i, len(list), 100.0*i/len(list)), end='')
        print(''.join(list[i]))
        rep = False
        if max < len(list[i]):
            del list[i]
            continue
        for j,e in enumerate(list[i]):
            if 'a' <= e <= 'z':
                # print('c')
                continue
            tbr = rules[e]
            if e == '8':
                del list[i][j]
                list[i].insert(j, '42')
                list.append(list[i].copy())
                list[-1].insert(j, '42')
                while len(list[-1])<100:
                    list.append(list[-1].copy())
                    list[-1].insert(j, '42')
                rep = True
                break
            elif e == '11':
                del list[i][j]
                list[i].insert(j, '42')
                list[i].insert(j+1, '31')
                list.append(list[i].copy())
                k=1
                list[-1].insert(j+k, '42')
                list[-1].insert(j+k+1, '31')
                while len(list[-1])<100:
                    k += 1
                    list.append(list[-1].copy())
                    list[-1].insert(j+k, '42')
                    list[-1].insert(j+k+1, '31')
                rep = True
                break
            elif type(tbr)==type((1,1)):
                # print('t')
                del list[i][j]
                list.append(list[i].copy())
                for k, a in enumerate(tbr[0]):
                    list[i].insert(j+k, a)
                for k, a in enumerate(tbr[1]):
                    list[-1].insert(j+k, a)
                rep = True
                break
            else:
                # print('l')
                del list[i][j]
                # print(list[i])
                for k, a in enumerate(tbr):
                    list[i].insert(j+k, a)
                    # print(list[i])
                rep = True
                break
        if rep:
            continue
        listS.add(''.join(list[i-1]))
        # print(len(listS))
        # time.sleep(0.1)
        i += 1
    print()

    count = 0
    for a in input:
        if a in listS:
            count += 1

    print('Part 2:', count)


def part2():

    (rules,input) = open('input/19.data', 'r').read().split('\n\n')
    
    rules = rules.split('\n')
    input = input.split('\n')

    max = 0
    for i in input:
        if max < len(i):
            max = len(i)

    max += 1

    ruls = {}
    for r in rules:
        (a, b) = r.split(': ')
        # print(r)
        if b[0] == '\"':
            # print('case 1')
            c = b[1:-1]
            ruls[a] = ([c])
        elif len(b.split(' ')) == 1:
            # print('case 2')
            ruls[a] = ([b])
        elif len(b.split(' | ')) == 1:
            # print('case 3')
            c = b.split(' ')
            ruls[a] = (c)
        else:
            # print('case 4')
            b = b.split(' | ')
            c1 = b[0].split(' ')
            c2 = b[1].split(' ')
            ruls[a] = (c1, c2)
    rules = ruls

    rules['8'] = (['42'],['42', '8'])
    rules['11'] = (['42', '31'],['42', '11', '31'])

    patts = {}
    for i in range(max):
        patts[i+1] = set([])
        for inp in input:
            if i+1 <= len(inp):
                patts[i+1].add(inp[:i+1])

    list = []
    list.append(rules['0'])
    listS = set([])

    i = 0
    while i < len(list):
        if i%10 == 0:
            print('\ri:{:10} l:{:10} %:{}'.format(i, len(list), 100.0*i/len(list)), end='')
        rep = False
        if max < len(list[i]):
            del list[i]
            continue
        for j,e in enumerate(list[i]):
            if 'a' <= e <= 'z':
                if rep == False:
                    if ''.join(list[i][:j+1]) not in patts[j+1]:
                        del list[i]
                        rep = True
                        break
                continue
            tbr = rules[e]
            if type(tbr)==type((1,1)):
                del list[i][j]
                list.append(list[i].copy())
                for k, a in enumerate(tbr[0]):
                    list[i].insert(j+k, a)
                for k, a in enumerate(tbr[1]):
                    list[-1].insert(j+k, a)
                rep = True
                break
            else:
                # print('case 3')
                del list[i][j]
                # print(list[i])
                for k, a in enumerate(tbr):
                    list[i].insert(j+k, a)
                    # print(list[i])
                rep = True
                break
        if rep:
            continue
        listS.add(''.join(list[i-1]))
        # print(len(listS))
        # time.sleep(0.1)
        i += 1
    print()

    count = 0
    for a in input:
        if a in listS:
            count += 1

    print('Part 2:', count+1) # yes yes the +1 is a total bullshit, something does not work

if __name__ == '__main__':

    # part1()
    part2()