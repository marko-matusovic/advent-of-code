def parse():
    raw = open('input/16.data', 'r').read()

    bin = hex_2_bin(raw)

    (packet, rest) = parse_packet(bin)

    if '1' in rest:
        print("Unsucessfull parsing, rest: \"" + rest + "\"")

    print(packet)

    return packet

def hex_2_bin(hex):
    convert = {
        '0': '0000',
        '1': '0001',
        '2': '0010',
        '3': '0011',
        '4': '0100',
        '5': '0101',
        '6': '0110',
        '7': '0111',
        '8': '1000',
        '9': '1001',
        'A': '1010',
        'B': '1011',
        'C': '1100',
        'D': '1101',
        'E': '1110',
        'F': '1111'
    }

    bin = ''

    for ch in hex:
        bin += convert[ch]

    return bin

def bin_2_dec(bin):
    res = 0
    for b in bin:
        res *= 2
        res += int(b)
    return res

def parse_packet(bin):

    ver = bin_2_dec(bin[0:3])
    typ = bin_2_dec(bin[3:6])
    rest = bin[6:]

    if typ == 4 : # Literal
        cont = True
        val = ''
        while cont:
            cont = rest[0] == '1'
            val += rest[1:5]
            rest = rest[5:]
        load = bin_2_dec(val)

    elif rest[0] == '0' : # operand + length

        length = bin_2_dec(rest[1:16])
        rest = rest[16:]
        start_length = len(rest)

        load = []

        while not start_length-len(rest) >= length:
            (packet, rest) = parse_packet(rest)
            load.append(packet)

    else: # operand + count
        count = bin_2_dec(rest[1:12])
        rest = rest[12:]
        load = []
        for i in range(count):
            (packet, rest) = parse_packet(rest)
            load.append(packet)

    return ((ver, typ, load), rest)

def count_vers(packet):
    (ver, typ, load) = packet
    sum = 0
    if typ != 4:
        for p in load:
            sum += count_vers(p)
    return ver + sum

def eval_packet(packet):
    (ver, typ, load) = packet
    if typ == 4:
        return load

    subs = [eval_packet(l) for l in load]

    if typ == 0: # Sum
        return sum(subs)
    elif typ == 1: # Product
        res = 1
        for s in subs:
            res *= s
        return res
    elif typ == 2: # Min
        return min(subs)
    elif typ == 3: # Max
        return max(subs)
    elif typ == 5:
        return 1 if subs[0] > subs[1] else 0
    elif typ == 6:
        return 1 if subs[0] < subs[1] else 0
    elif typ == 7:
        return 1 if subs[0] == subs[1] else 0
    else:
        print('error')
        pass

def part1(packet):

    count = count_vers(packet)

    print("Part 1:", count)

def part2(data):

    val = eval_packet(data)

    print("Part 2:", val)

if __name__ == '__main__':
    part1(parse())
    part2(parse())