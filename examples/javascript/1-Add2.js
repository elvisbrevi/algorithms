function addUpTo(num) {
  return (num * (num + 1)) / 2;
}

let t1 = performance.now();
console.log(addUpTo2(1000000000));
let t2 = performance.now();
console.log(`Time elapsed: ${(t1 - t2) / 1000} seconds`);
