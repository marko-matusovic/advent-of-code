arr = '''114
51
122
26
121
90
20
113
8
138
57
44
135
76
134
15
21
119
52
118
107
99
73
72
106
41
129
83
19
66
132
56
32
79
27
115
112
58
102
64
50
2
39
3
77
85
103
140
28
133
78
34
13
61
25
35
89
40
7
24
33
96
108
71
11
128
92
111
55
80
91
31
70
101
14
18
12
4
84
125
120
100
65
86
93
67
139
1
47
38'''.split('\n')

arr = [int(x) for x in arr]
arr.append(0)
arr.sort()
arr.append(arr[-1] + 3)


def app1():
    prev = arr[0]
    count = [0, 0, 0]
    for a in arr[1:]:
        count[a - prev - 1] += 1
        # print('{:3} {:3} {:1} {:3}'.format(prev, a, a-prev, count[a-prev-1]))
        prev = a

    print('Part 1:', count[0] * count[2])


def app2():
    brr = [[arr[0]]]
    prev = arr[0]
    for a in arr[1:]:
        if a - prev == 3:
            brr.append([])
        brr[-1].append(a)
        prev = a

    total = 1
    for b in brr:
        total *= comb(b)
    print('Part 2:', total)


def comb(arr):
    if len(arr) <= 2:
        return 1

    print('from', arr)

    lis = [[arr[0]]]
    for a in arr[1:]:
        for l in lis:
            if 0 < a-l[-1] <= 3:
                lis.append(l+[a])
                print(l+[a])

    count = 0
    i = len(lis)-1
    while lis[i][-1]==arr[-1]:
        count += 1
        i -= 1

    print(count)
    return count


if __name__ == '__main__':
    app1()
    app2()
