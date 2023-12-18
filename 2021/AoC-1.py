def parse():
    inp = open('input/1.data', 'r').read()
    
    return [int(x) for x in inp.split()]

def part1(data):
    count = 0
    for i in range(1, len(data)):
        if(data[i-1] < data[i]):
            count += 1
    print("Part 1: {}".format(count))

def part2(data):
    count = 0
    # avgs = []
    # for i in range(2, len(data)):
    #     avgs.append((data[i-2]+data[i-1]+data[i])/3)
    # for i in range(1, len(avgs)):
    for i in range(3, len(data)):
        if(data[i-3] < data[i]):
            count += 1

    print("Part 2: {}".format(count))

if __name__ == '__main__':
    data = parse()
    part1(data)
    part2(data)



