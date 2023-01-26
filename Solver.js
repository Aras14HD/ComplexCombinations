let solve = (arr) => {
  if (arr == [1])
    return 1;
  let out = 0;
  for (let i = 0; i < arr.length; i++) {
    if (arr[i] != 0) {
      let diff = 0;
      let copy = [] + arr;
      copy[i]--;
      console.log(copy[i])
      while (copy[copy.length-1] == 0) {
        copy.pop()
      }
      if (i == arr.length-1) {
        diff = arr[i];
      } else {
        diff = arr[i] - arr[i+1];
      }
      if (diff != 0)
        out += diff*solve(copy);
    } else {
      console.error("Wrong Format!")
      return 0
    }
  }
  return out
}


arr = process.argv.slice(2)
console.time("took")
res = solve(arr)
console.timeEnd("took")
console.log("res")
