def eval(num1, op, num2):
    if op == '+':
        return int(num1) + int(num2)
    elif op == '*':
        return int(num1) * int(num2)
    elif op == None:
        if num1 == None:
            return num2
        elif num2 == None:
            return num1
    else:
        print('error: expected \'+\' or \'*\' but got \'{}\''.format(op))

def part1(arr):
    sum = 0

    for exp in arr:
        numstack = []
        opstack = []
        curnum = None
        curop = None
        for e in exp.split(' '):
            if e=='+' or e=='*':
                curop = e
            elif e == '(':
                numstack.append(curnum)
                opstack.append(curop)
                curnum = None
                curop = None
            elif e == ')':
                curnum = eval(numstack.pop(), opstack.pop(), curnum)
                curop = None
            else: # number
                curnum = eval(curnum, curop, e)
                curop = None
        sum += curnum

    return sum

def addBrack(arr, l, r):
    if l+2>=r:
        return (arr, r)
    arr.insert(l, '(')
    arr.insert(r+1, ')')
    return (arr, r+2)

def part2(arr):

    brr = []
    for exp in arr:
        exps = exp.split(' ')

        i = 0
        last = 0
        stack = []
        while i<len(exps):
            # for j,s in enumerate(stack):
            #     print('  ' * s + str(j))
            # print('  '*last + 'L')
            # print('  '*i + 'i')
            # print(' '.join(exps))
            e = exps[i]
            if e == '+':
                1 # do nothing just move forward
            elif e == '*':
                (exps, i) = addBrack(exps, last, i)
                last = i+1
            elif e == '(':
                stack.append(last)
                last = i + 1
            elif e == ')':
                (exps, i) = addBrack(exps, last, i)
                last = stack.pop()
            i += 1

        (exps, i) = addBrack(exps, last, i)

        # print(exp)
        # print(' '.join(exps))
        # print()
        brr.append(' '.join(exps))

    arr = brr

    return part1(arr)

if __name__ == '__main__':
    arr = open('input/18.data', 'r').read()
    arr = arr.split('\n')
    brr = []
    for a in arr:
        exp = a.replace('(', '( ')
        exp = exp.replace(')', ' )')
        brr.append(exp)
    arr = brr
    # arr = ['1 + ( 2 * 3 ) + ( 4 * ( 5 + 6 ) )'] # = 51
    # arr = ['2 * 3 + ( 4 * 5 )'] # = 46
    # arr = ['5 + ( 8 * 3 + 9 + 3 * 4 * 3 )'] # = 1445
    print('Part 1:', part1(arr))
    print('Part 2:', part2(arr))