def parse():
    data = open('input/3.d', 'r').read().split('\n')

    return data

def bin2dec(bin):
    dec = 0

    for b in bin:
        dec *= 2
        if b=="1" :
            dec += 1

    return dec

def part1(data):

    freq = [0] * len(data[0])

    for dat in data:
        for (i, d) in enumerate(dat):
            freq[i] += int(d)*2 - 1

    gamma = 0
    epsylon = 0

    for f in freq:
        gamma *= 2
        epsylon *= 2
        if f<0 :
            gamma += 0
            epsylon += 1
        elif f>0 :
            gamma += 1
            epsylon += 0


    print("Part 1:")
    print("gamma:", gamma)
    print("epsylon:", epsylon)
    print("gamma*epsylon:", gamma * epsylon)


def part2(data):
    oxi = 0
    co2 = 0


    id = [x for x in range(len(data))]
    skip = []

    for j in range(len(data[0])):
        balance = 0

        for i in id:
            if i in skip:
                continue
            if data[i][j] == "1":
                balance += 1
            elif data[i][j] == "0":
                balance -= 1
            else:
                print("wtf")

        remove = "0" if balance >= 0 else "1"

        for i in id:
            if i in skip:
                continue
            if data[i][j] == remove:
                skip.append(i)

        # print("size:", len(id)-len(skip))
        # print("balance:", balance)
        # print("id:", id)
        # print("skip:", skip)

        if len(id) - len(skip) == 1:
            for i in id:
                if i in skip:
                    continue
                oxi = data[i]

        if oxi != 0:
            break

    id = [x for x in range(len(data))]
    skip = []

    for j in range(len(data[0])):
        balance = 0

        for i in id:
            if i in skip:
                continue
            if data[i][j] == "1":
                balance += 1
            elif data[i][j] == "0":
                balance -= 1
            else:
                print("wtf")

        remove = "1" if balance >= 0 else "0"

        for i in id:
            if i in skip:
                continue
            if data[i][j] == remove:
                skip.append(i)

        # print("size:", len(id)-len(skip))
        # print("balance:", balance)
        # print("id:", id)
        # print("skip:", skip)

        if len(id) - len(skip) == 1:
            for i in id:
                if i in skip:
                    continue
                co2 = data[i]

        if co2 != 0:
            break

    oxi = bin2dec(oxi)
    co2 = bin2dec(co2)

    print("Part 2:")
    print("oxi:", oxi)
    print("co2:", co2)
    print("oxi * co2:", oxi * co2 )

if __name__ == '__main__':
    data = parse()
    part1(data)
    part2(data)


    # 3708000 too low