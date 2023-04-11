let n = 1;
let iter = 0;
let prev_n = 0;
let prev_n_2 = 0;
while(true) {
  iter += 1;
  const new_n = n / 2;
  if(n === new_n) {
    console.log(`iter: ${iter}, n: ${n}, prev_n: ${prev_n_2}`);
    break;
  } else {
    prev_n_2 = prev_n;
    prev_n = n;
    n = new_n;
  }
}