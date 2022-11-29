def parse():
    raw = 'target area: x=240..292, y=-90..-57'
    # ehm
    data = (240, 292, -90, -57)
    # data = (20,30,-10,-5)

    return data

def hit(pos, target):
    (x, y) = pos
    (x1, x2, y1, y2) = target
    return x1 <= x <= x2 and y1 <= y <= y2

def path(dx, dy, t):
    x = 0
    y = 0
    path = []
    for i in range(t):
        path.append((x,y))
        x += dx
        y += dy
        dx = 0 if dx == 0 else dx - int(dx/abs(dx))
        dy -= 1
    return path

def part1(data):

    (x1,x2,y1,y2) = data

    y_0 = abs(y1)-1

    h_max = (y_0+1)*(y_0/2)

    print('Part 1:', h_max)

def part2(data):

    (x1,x2,y1,y2) = data

    count = 0
    for x in range(-300, 300):
        for y in range(-100, 100):
            print('\r',x,y,end='\t')
            for p in path(x, y, 300):
                if hit(p, data):
                    count += 1
                    break

    print("\rPart 2:", count)

    # 172 too low

if __name__ == '__main__':
    part1(parse())
    part2(parse())