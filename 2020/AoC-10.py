arr = open('input/10.data', 'r').read().split('\n')

arr = [int(x) for x in arr]
arr.append(0)
arr.sort()
arr.append(arr[-1] + 3)


def app1():
    prev = arr[0]
    count = [0, 0, 0]
    for a in arr[1:]:
        count[a - prev - 1] += 1
        # print('{:3} {:3} {:1} {:3}'.format(prev, a, a-prev, count[a-prev-1]))
        prev = a

    print('Part 1:', count[0] * count[2])


def app2():
    brr = [[arr[0]]]
    prev = arr[0]
    for a in arr[1:]:
        if a - prev == 3:
            brr.append([])
        brr[-1].append(a)
        prev = a

    total = 1
    for b in brr:
        total *= comb(b)
    print('Part 2:', total)


def comb(arr):
    if len(arr) <= 2:
        return 1

    print('from', arr)

    lis = [[arr[0]]]
    for a in arr[1:]:
        for l in lis:
            if 0 < a-l[-1] <= 3:
                lis.append(l+[a])
                print(l+[a])

    count = 0
    i = len(lis)-1
    while lis[i][-1]==arr[-1]:
        count += 1
        i -= 1

    print(count)
    return count


if __name__ == '__main__':
    app1()
    app2()
