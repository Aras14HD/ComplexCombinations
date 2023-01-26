let cache = {}
let solve = arr => {
  //console.log(arr)
  let key = JSON.stringify(arr);
  if (cache[key]) {
    //console.log("Cached!")
    return cache[key]
  }
  if (arr.length == 1 && arr[0] == 1) {
    return 1;
  }
  let out = 0;
  for (let i = 0; i < arr.length; i++) {
    if (arr[i] != 0) {
      let diff = 0;
      let copy = [...arr];
      copy.splice(i, 1, copy[i]-1);
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
  cache[key] = out
  return out
}


arr = process.argv.slice(2)
console.time("took")
res = solve(arr)
console.timeEnd("took")
console.log(res)
