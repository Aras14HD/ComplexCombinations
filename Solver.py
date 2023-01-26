from memoization import cached
import time

@cached
def solve(arr):
    if arr == [1]:
        return 1
    out = 0
    i = len(arr)
    while i > 0:
        i -= 1
        if arr[i] != 0:
            copy = arr.copy()
            copy[i] -= 1
            while copy[len(copy)-1] == 0:
                copy.pop()
            if i == len(arr)-1:
                diff = arr[i]
            else:
                diff = arr[i] - arr[i+1]
            if diff != 0:
                out += diff*solve(copy)
    return out

arr = [9, 9, 9, 9]
before = time.time()
res = solve(arr)
after = time.time()
print("Took: " + str((after - before)*1000) + "ms")
print(res)
