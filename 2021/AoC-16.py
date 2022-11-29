def parse():
    raw = '''40541D900AEDC01A88002191FE2F45D1006A2FC2388D278D4653E3910020F2E2F3E24C007ECD7ABA6A200E6E8017F92C934CFA0E5290B569CE0F4BA5180213D963C00DC40010A87905A0900021B0D624C34600906725FFCF597491C6008C01B0004223342488A200F4378C9198401B87311A0C0803E600FC4887F14CC01C8AF16A2010021D1260DC7530042C012957193779F96AD9B36100907A00980021513E3943600043225C1A8EB2C3040043CC3B1802B400D3CA4B8D3292E37C30600B325A541D979606E384B524C06008E802515A638A73A226009CDA5D8026200D473851150401E8BF16E2ACDFB7DCD4F5C02897A5288D299D89CA6AA672AD5118804F592FC5BE8037000042217C64876000874728550D4C0149F29D00524ACCD2566795A0D880432BEAC79995C86483A6F3B9F6833397DEA03E401004F28CD894B9C48A34BC371CF7AA840155E002012E21260923DC4C248035299ECEB0AC4DFC0179B864865CF8802F9A005E264C25372ABAC8DEA706009F005C32B7FCF1BF91CADFF3C6FE4B3FB073005A6F93B633B12E0054A124BEE9C570004B245126F6E11E5C0199BDEDCE589275C10027E97BE7EF330F126DF3817354FFC82671BB5402510C803788DFA009CAFB14ECDFE57D8A766F0001A74F924AC99678864725F253FD134400F9B5D3004A46489A00A4BEAD8F7F1F7497C39A0020F357618C71648032BB004E4BBC4292EF1167274F1AA0078902262B0D4718229C8608A5226528F86008CFA6E802F275E2248C65F3610066274CEA9A86794E58AA5E5BDE73F34945E2008D27D2278EE30C489B3D20336D00C2F002DF480AC820287D8096F700288082C001DE1400C50035005AA2013E5400B10028C009600A74001EF2004F8400C92B172801F0F4C0139B8E19A8017D96A510A7E698800EAC9294A6E985783A400AE4A2945E9170'''

    # raw = 'D2FE28' # Lit 2021
    # raw = '38006F45291200' # Op I0
    # raw = 'EE00D40C823060' # Op I1

    # raw = 'A0016C880162017C3686B18A3D4780' # sum = 31

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