arr = open('input/3.d', 'r').read().split('\n')

dxy = [(1,1),(3,1),(5,1),(7,1),(1,2)]

for (dx, dy) in dxy:
    x = 0
    y = 0
    count = 0
    while y < len(arr):
        r = arr[y]
        if(r[x%(len(r))]=='#'):
            count +=1
        x += dx
        y += dy

    print('dx:',dx,'dy',dy,'count:',count)
