// Calculate the 10th Fibonacci number.

let a = 0;
let b = 1;

for (let i = 0; i < 10; i = i + 1) {
  let tmpA = a;
  let tmpB = b;

  a = tmpB;
  b = tmpA + tmpB;
}
