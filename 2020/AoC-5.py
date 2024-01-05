arr = open('input/5.d', 'r').read().split()

maxId = 0

def getId(c, r):
    return c*8+r;

def getSeat(s):
    col = 0
    for i in range(7):
        if s[6-i]=='B' :
            col += 2**i
    row = 0
    for i in range(3):
        if s[9-i]=='R' :
            row += 2**i

    return getId(col, row)

brr = []
maxS = 0
for a in arr:
    n = getSeat(a)
    brr.append(n)
    if maxS < n:
        maxS = n

print('Part 1:',maxS)


brr.sort()

for i in range (1000):
    if i-1 in brr and i not in brr and i+1 in brr:
        print('Part 2:', i)
        break
