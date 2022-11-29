def parse():
    raw = '''end-ry
jf-jb
jf-IO
jb-hz
jo-LM
hw-end
hw-LM
hz-ry
WI-start
LM-start
kd-jf
xi-WI
hw-jb
hz-jf
LM-jb
jb-xi
ry-jf
WI-jb
end-hz
jo-start
WI-jo
xi-ry
xi-LM
xi-hw
jo-xi
WI-jf'''

#     raw = '''fs-end
# he-DX
# fs-he
# start-DX
# pj-DX
# end-zg
# zg-sl
# zg-pj
# pj-he
# RW-he
# fs-DX
# pj-RW
# zg-RW
# start-pj
# he-WI
# zg-he
# pj-fs
# start-RW'''

    map = {}
    for row in raw.split('\n'):
        [a, b] = row.split('-')
        if a not in map:
            map[a] = []
        if b not in map:
            map[b] = []
        map[a].append(b)
        map[b].append(a)

    return map

def is_small_cave(cave):
    return cave.lower() == cave

def explore1(map, paths, path):
    # print(path)
    if path[-1] == 'end':
        paths.append(path.copy())
        return
    # print('\t->', map[path[-1]])
    for edge in map[path[-1]]:
        if is_small_cave(edge) and edge in path:
            continue
        copy = path.copy()
        copy.append(edge)
        explore1(map, paths, copy)

def part1(data):

    paths = []
    path = ['start']

    explore1(data, paths, path)

    print("Part 1:", len(paths))


def two_small_caves(path):
    data = path.copy()
    data.sort()
    prev = data[0]
    for cur in data[1:]:
        if prev == cur and is_small_cave(cur):
            return True
        prev = cur
    return False

def explore2(map, paths, path):
    # print(path)
    if path[-1] == 'end':
        paths.append(path.copy())
        return
    # print('\t->', map[path[-1]])
    for edge in map[path[-1]]:
        if edge == 'start':
            continue
        if is_small_cave(edge) and edge in path and two_small_caves(path):
            continue
        copy = path.copy()
        copy.append(edge)
        explore2(map, paths, copy)

def part2(data):

    paths = []
    path = ['start']

    explore2(data, paths, path)

    print("Part 2:", len(paths))
    # 30402 too low

if __name__ == '__main__':
    part1(parse())
    part2(parse())