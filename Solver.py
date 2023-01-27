from memoization import cached
import time
import sys

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
        else:
            print("Wrong Format")
            return 0
    return out

arr = []
for arg in sys.argv[1:]:
    arr.append(int(arg))
before = time.time()
res = solve(arr)
after = time.time()
print("Took: " + str((after - before)*1000) + "ms")
print("Result: " + str(res))
