arr = open('input/7.d', 'r').read().split('\n')


def main():
    class Bag:
        def __init__(self, col):
           self.parents = []
           self.children = {}
           self.col = col

        def addParent(self, p):
            self.parents.append(p)

        def addChild(self, child, n):
            self.children[child] = n
            child.addParent(self)

    global bags
    bags = {}
    for a in arr:
        parts = a.split(' bags contain ')
        if parts[0] not in bags:
            bags[parts[0]] = Bag(parts[0])
        parent = bags[parts[0]]
        if parts[1] == 'no other bags.':
            continue
        for ch in parts[1].split(', '):
            ws = ch.split(' ')
            count = int(ws[0])
            color = ' '.join(ws[1:-1])
            if color not in bags:
                bags[color] = Bag(color)
            parent.addChild(bags[color], count)

    target = 'shiny gold'
    canhol = []
    queue = [bags[target]]

    while len(queue) > 0 :
        cur = queue[0]
        del queue[0]
        for p in cur.parents:
            if p not in canhol:
                queue.append(p)
                canhol.append(p)

    print('Part 1:', len(set(canhol)))


    def count(bag):
        c = 1
        for b in bag.children:
            c += bag.children[b] * count(b)
        return c

    print('Part 2:', count(bags['shiny gold']))

if __name__ == '__main__':
    main()
