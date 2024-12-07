arr = open('input/4.d', 'r').read().split('\n\n')

allow = ['byr','iyr','eyr','hgt','hcl','ecl','pid']
corr = 0

for line in arr:
    count = 0
    allowC = allow.copy()
    for a in line.split():
        if a.split(':')[0] in allowC:
            count += 1
            allowC.remove(a.split(':')[0])
    if count >= 7:
        corr += 1

print("Part 1:", corr)


import re

corr = 0

for line in arr:
    dict = {}
    br = False
    for a in line.split():
        a = a.split(':')
        if a[0] not in dict :
            dict[a[0]] = a[1]
        else:
            br = True
            break
    if br :
        continue
    try:
        cond1 = 1920 <= int(dict['byr']) <= 2002
        cond2 = 2010 <= int(dict['iyr']) <= 2020
        cond3 = 2020 <= int(dict['eyr']) <= 2030
        if dict['hgt'][-2:]=='cm':
            cond4 = 150 <= int(dict['hgt'][:-2]) <= 193
        elif dict['hgt'][-2:]=='in':
            cond4 = 59 <= int(dict['hgt'][:-2]) <= 76
        else:
            cond4 = False
        cond5 = re.search("^#[a-f0-9]{6}$", dict['hcl']) != None
        cond6 = dict['ecl'] in ['amb','blu','brn','gry','grn','hzl','oth']
        cond7 = re.search('^[0-9]{9}$', dict['pid']) != None
        #print(dict, cond1, cond2, cond3, cond4, cond5, cond6, cond7)
        if cond1 and cond2 and cond3 and cond4 and cond5 and cond6 and cond7:
            corr += 1
    except:
        #do nothing
        doodoo = 1
    
print('Part 2: ', corr)
