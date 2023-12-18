arr = open('input/9.data', 'r').read().split('\n')

def canSum(nums, n):
    for i, a in enumerate(nums):
        for j, b in enumerate(nums[i + 1:]):
            if a+b == n :
                return True
    return False

def sums(arr):
    tot = 0
    for a in arr:
        tot += a
    return tot

def minIn(arr):
    minN = arr[0]
    for a in arr:
        if a<minN :
            minN = a
    return minN


def maxIn(arr):
    maxN = arr[0]
    for a in arr:
        if maxN<a :
            maxN = a
    return maxN

if __name__ == '__main__':
    p = 25
    nums = []
    for n in arr[:p]:
        nums.append(int(n))

    i = p
    while i < len(arr):
        n = int(arr[i])
        if not canSum(nums, n):
            print('Part 1:',n)
            break
        nums.append(n)
        del nums[0]
        i += 1

    # PART 2

    # i - index
    # n - number
    nums=[]
    for num in arr[:i]:
        nums.append(int(num))

    for i in range(len(nums)):
        for j in range(i+1, len(nums)):
            if sums(nums[i:j]) == n:
                minN = minIn(nums[i:j])
                maxN = maxIn(nums[i:j])
                print('Part 2:', i, j, minN, maxN, minN + maxN)



