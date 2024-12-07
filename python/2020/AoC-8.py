arr = open('input/8.d', 'r').read().split('\n')

def run1():
    i = 0
    exec = []
    accu = 0
    while i not in exec:
        exec.append(i)
        [ins, num] = arr[i].split(' ')
        if ins == 'acc':
            accu += int(num)
            i += 1
        elif ins == 'jmp':
            i += int(num)
        elif ins == 'nop':
            i += 1
        else :
            print('error')
            break
    print(accu)

def run2(i, exec, accu, swap):
    if i in exec:
        return (False, accu)
    if len(arr) <= i:
        return (True, accu)

    exec.append(i)
    [ins, num] = arr[i].split(' ')
    if ins == 'acc':
        return run2(i+1, exec, accu+int(num), swap)
    if swap :
        if ins == 'jmp':
            retA = run2(i + int(num), exec, accu, True)
            if retA[0] :
                return retA
            retB = run2(i + 1, exec, accu, False)
            if retB[0] :
                return retB
            return (False, 0)
        elif ins == 'nop':
            retA = run2(i + 1, exec, accu, True)
            if retA[0]:
                return retA
            retB = run2(i + int(num), exec, accu, False)
            if retB[0] :
                return retB
            return (False, 0)
    else:
        if ins == 'jmp':
            return run2(i+int(num), exec, accu, False)
        elif ins == 'nop':
            return run2(i+1, exec, accu, False)
        else:
            print("Error unknown insctruction:", arr[i])
            exit()


if __name__ == '__main__':
    run1()

    (cond, accu) = run2(0, [], 0, True)
    if cond :
        print('Part 2:', accu)
    else :
        'What happened?'