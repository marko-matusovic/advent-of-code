arr = open('input/14.d', 'r').read().split('\n')


def app1():
    mem = {}
    mask = 0
    for a in arr:
        a = a.split(' = ')
        if a[0]=='mask':
            mask = a[1]
        elif a[0][:3] == 'mem':
            mem[int(a[0][4:-1])] = applyMask(int(a[1]), mask)

    sum = 0
    for i in mem:
        sum += mem[i]

    print('Part 1:', sum)

def app2():
    mem = {}
    mask = 0
    for a in arr:
        a = a.split(' = ')
        if a[0]=='mask':
            mask = a[1]
        elif a[0][:3] == 'mem':
            add = int(a[0][4:-1])
            val = int(a[1])
            lis = applyMask2(add, mask)
            for l in lis:
                mem[l] = val

    sum = 0
    for i in mem:
        sum += mem[i]

    print('Part 2:', sum)

def applyMask(num, mask):
    out = 0
    for i in range(0, 36):
        b = 1 << i
        if mask[36-i-1] == 'X':
            if num & b > 0:
                out += b
        elif mask[36-i-1] == '1':
            out += b
    return out

def applyMask2(num, mask):
    out = 0
    xs = []
    for i in range(0, 36):
        b = 1 << i
        if mask[36-i-1] == 'X':
            xs.append(i)
        elif mask[36-i-1] == '1' or (mask[36-i-1] == '0' and num&b > 0):
            out += b
    out = [out]
    for x in xs:
        cur = out.copy()
        for c in cur:
            out.append(c + (1 << x))
    return out

if __name__ == '__main__':
    app1()
    app2()