if __name__=='__main__':
    a = open('input/1.data', 'r').read().split()

    arr = [int(x) for x in a]

    arr.sort()

    for a in arr:
        for b in arr:
            if(a+b>2020):
                break
            for c in arr:
                if(a+b+c>2020):
                    break
                if(a+b+c==2020):
                    print(a*b*c)
                    break
