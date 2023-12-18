arr = open('input/12.data', 'r').read().split('\n')

import numpy as np

if __name__ == '__main__':
    sx = 0
    sy = 0
    wx = 10
    wy = 1
    for a in arr:
        if a[0] == 'N':
            wy += int(a[1:])
        elif a[0] == 'S' :
            wy -= int(a[1:])
        elif a[0] == 'E':
            wx += int(a[1:])
        elif a[0] == 'W':
            wx -= int(a[1:])
        elif a[0] == 'F' :
            sx += wx * int(a[1:])
            sy += wy * int(a[1:])
        elif a[0] == 'R':
            print(a)
            print('\t', wx, wy)
            dis = np.sqrt(np.square(wx) + np.square(wy))
            ang = np.arctan(wy/(wx+0.00000001)) - int(a[1:])/180*np.pi
            if wx < 0 :
                ang += np.pi
            wx = int(np.round(dis * np.cos(ang)))
            wy = int(np.round(dis * np.sin(ang)))
            print('\t', wx, wy)
        elif a[0] == 'L':
            print(a)
            print('\t', wx, wy)
            dis = np.sqrt(np.square(wx) + np.square(wy))
            ang = np.arctan(wy/(wx+0.00000001)) + int(a[1:])/180*np.pi
            if wx < 0 :
                ang += np.pi
            wx = int(np.round(dis * np.cos(ang)))
            wy = int(np.round(dis * np.sin(ang)))
            print('\t', wx, wy)
        else:
            print('error', a)
            break
        print(a, wx, wy, sx, sy)

    print('Part 2:', sx, sy, abs(sx)+abs(sy))

