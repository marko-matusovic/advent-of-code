arr = open('input/2.data', 'r').read().split('\n')

arr = [a.split() for a in arr]
c = 0
for a in arr:
    nin,nax = a[0].split('-')
    nin = int(nin)
    nax = int(nax)
    char = a[1][:-1]
    pwd = a[2]
    i=0
    count = 0
    while(i!=-1):
        i = pwd.find(char, i)
        if(i==-1):
            break
        i += 1
        count += 1
    if(nin <= count <= nax):
        c += 1
        #print(nin, nax, char, pwd)

print('part 1', c)


c = 0
for a in arr:
    nin,nax = a[0].split('-')
    nin = int(nin)-1
    nax = int(nax)-1
    char = a[1][:-1]
    pwd = a[2]

    if(nax<len(pwd)):
        if(((pwd[nin]==char and pwd[nax]!=char) or (pwd[nin]!=char and pwd[nax]==char))):
            c += 1
            #print(nin+1, nax+1, char, pwd)
        
print('part 2', c)
