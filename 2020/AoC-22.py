arr = '''Player 1:\n17\n19\n30\n45\n25\n48\n8\n6\n39\n36\n28\n5\n47\n26\n46\n20\n18\n13\n7\n49\n34\n23\n43\n22\n4\n\nPlayer 2:\n44\n10\n27\n9\n14\n15\n24\n16\n3\n33\n21\n29\n11\n38\n1\n31\n50\n41\n40\n32\n42\n35\n37\n2\n12'''
# arr = '''Player 1:\n9\n2\n6\n3\n1\n\nPlayer 2:\n5\n8\n4\n7\n10'''


def part1(p1, p2):
    while len(p1) != 0 and len(p2) != 0:
        a = p1.pop()
        b = p2.pop()
        if a > b:
            p1.insert(0, a)
            p1.insert(0, b)
        elif b > a:
            p2.insert(0, b)
            p2.insert(0, a)
        else:
            print('Why is ', a, 'and', b, 'the same!!!!')

    count = 0
    res = p1
    if len(p1) == 0:
        res = p2
    for i, n in enumerate(res):
        count += (i + 1) * n

    print('Part 1:', count)

import time

def part2(p1, p2):
    (w, q1, q2) = play(p1, p2, '')

    res = None
    if w == 1:
        res = q1
    elif w == 2:
        res = q2
    else:
        print("what's wrong now?")

    count = 0
    for i, n in enumerate(res):
        count += (i + 1) * n

    # print(w, q1, q2)

    print('Part 2:', count)

def play(p1, p2, rec):
    # i = 0
    # time.sleep(0.1)
    history = set([])
    while len(p1) != 0 and len(p2) != 0:
        # print(rec,'round',i,'hist:',len(history))
        #
        # print(rec, 'p1:', p1)
        # print(rec, 'p2:', p2)

        if hash(p1, p2) in history:
            return (1, p1, p2)
        history.add(hash(p1, p2))

        a = p1.pop()
        b = p2.pop()

        # print(rec,'p1:',a)
        # print(rec,'p2:',b)

        w = 0
        if a <= len(p1) and b <= len(p2):
            (w, q1, q2) = play(p1[-a:].copy(), p2[-b:].copy(), rec+'\t')

        if w == 1 or (w==0 and a > b):
            p1.insert(0, a)
            p1.insert(0, b)
        elif w == 2 or (w==0 and b > a):
            p2.insert(0, b)
            p2.insert(0, a)
        else:
            print('Why is ', a, 'and', b, 'the same!!!!')
        # i += 1

    if len(p1) == 0:
        return (2, p1, p2)
    elif len(p2) == 0:
        return (1, p1, p2)
    else:
        print('how are both len 0')
        return None


def hash(a, b):
    return (';'.join([str(x) for x in a])) + ';-;' + (';'.join([str(x) for x in b]))


if __name__ == '__main__':
    (p1, p2) = arr.split('\n\n')
    p1 = [int(x) for x in p1.split('\n')[1:][::-1]]
    p2 = [int(x) for x in p2.split('\n')[1:][::-1]]

    # part1(p1, p2)
    part2(p1, p2)


# P2
# 33393 too high
# 35770 too high