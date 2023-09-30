package main

import (
	"fmt"
	"os"
	"strconv"
	"time"
)

var combCache = make(map[string]uint)

func comb(arr []uint) uint {
	if len(arr) == 0 {
		return 0
	}
	if len(arr) == 1 && arr[0] == 1 {
		return 1
	}
	str := fmt.Sprint(arr)
	cached, isCached := combCache[str]
	if isCached {
		return cached
	}
	var out uint = 0

	for i := 0; i < len(arr); i++ {
		if arr[i] == 0 {
			continue
		}
		var diff uint
		if i == len(arr)-1 {
			diff = arr[i]
		} else if arr[i+1] > arr[i] {
			continue
		} else {
			diff = arr[i] - arr[i+1]
		}
		arr[i] -= 1

		j := len(arr)
		for ; arr[j-1] == 0; j-- {
			if j-1 == 0 {
				break
			}
		}

		res := comb(arr[:j])
		out += res * diff

		arr[i] += 1
	}

	combCache[str] = out
	return out
}

func main() {
	args := os.Args[1:]
	arr := make([]uint, len(args))
	for i := 0; i < len(args); i++ {
		n, e := strconv.Atoi(args[i])
		if e != nil {
			fmt.Print("Error parsing input!")
		}
		arr[i] = uint(n)
	}
	start := time.Now()
	res := comb(arr)
	len := time.Now().Sub(start)
	fmt.Printf("Took: %d µs\n", len.Microseconds())
	fmt.Printf("Got: %d\n", res)
}
