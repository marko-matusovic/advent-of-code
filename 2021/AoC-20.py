def parse():
    raw = open('input/20.data', 'r').read()

    filter = raw.split('\n\n')[0]
    image = raw.split('\n\n')[1].split('\n')

    return (filter, image)

def bin_2_int(bin):
    res = 0
    for b in bin:
        res *= 2
        res += 1 if b == '1' or b == '#' else 0
    return res

def enlarge(img1, c):
    img2 = [c * (len(img1) + 2)]
    for line in img1:
        img2.append(c + line + c)
    img2.append(img2[0])
    return img2

def print_img(img):
    print("image:")
    for i in img:
        print(i)


def part1(data):
    (filter, img1) = data

    # print_img(img1)
    img2 = []
    img1 = enlarge(enlarge(img1, '.'), '.')
    for i in range(1, len(img1) - 1):
        img2.append('')
        for j in range(1, len(img1[i]) - 1):
            area = img1[i - 1][j - 1:j + 2] + img1[i][j - 1:j + 2] + img1[i + 1][j - 1:j + 2]
            num = bin_2_int(area)
            img2[-1] += filter[num]
    img1 = img2
    # print_img(img1)
    img2 = []
    img1 = enlarge(enlarge(img1, '#'), '#')
    for i in range(1, len(img1) - 1):
        img2.append('')
        for j in range(1, len(img1[i]) - 1):
            area = img1[i - 1][j - 1:j + 2] + img1[i][j - 1:j + 2] + img1[i + 1][j - 1:j + 2]
            num = bin_2_int(area)
            img2[-1] += filter[num]
    img1 = img2
    # print_img(img1)

    count = 0
    for line in img1:
        for c in line:
            count += 1 if c == '#' else 0

    print("Part 1:", count)

    # 104 too low

def part2(data):
    (filter, image) = data

    (filter, img1) = data

    for i in range(50):
        img2 = []
        c = '.' if i % 2 == 0 else '#'
        img1 = enlarge(enlarge(img1, c), c)
        for i in range(1, len(img1) - 1):
            img2.append('')
            for j in range(1, len(img1[i]) - 1):
                area = img1[i - 1][j - 1:j + 2] + img1[i][j - 1:j + 2] + img1[i + 1][j - 1:j + 2]
                num = bin_2_int(area)
                img2[-1] += filter[num]
        img1 = img2

    count = 0
    for line in img1:
        for c in line:
            count += 1 if c == '#' else 0

    print("Part 2:", count)

if __name__ == '__main__':
    part1(parse())
    part2(parse())