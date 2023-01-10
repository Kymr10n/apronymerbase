use std::mem;

pub fn permutate<T>(source: &mut Vec<T>) {
   let length = source.len();
   let mut iterator: Vec<usize> = Vec::new();
      //let mut iterator: Vec<usize> = vec![0; length+1];

   for i in 0..length {
      iterator.push(i);
   }

   let mut iterator: Vec<usize> = vec![0; length+1];

   let mut i = 1;

   while i < length {
      let j;

      iterator[i] -= 1;

      match i % 2 == 0 {
         true => j = iterator[i],
         false => j = 0
      }
/* 
      let helper: T;
      helper = source[i];
      source[i] = source[j];
      source[j] = helper; */

      //mem::swap(&mut source[j], &mut source[i]);

      i = 1;

      while iterator[i] == 0 {
         iterator[i] = i;
         i += 1;
      }
   }
}

/*
let a[] represent an arbitrary list of objects to permute
   let N equal the length of a[]
   create an integer array p[] of size N+1 to control the iteration     
   initialize p[0] to 0, p[1] to 1, p[2] to 2, ..., p[N] to N
   initialize index variable i to 1
   while (i < N) do {
      decrement p[i] by 1
      if i is odd, then let j = p[i] otherwise let j = 0
      swap(a[j], a[i])
      let i = 1
      while (p[i] is equal to 0) do {
         let p[i] = i
         increment i by 1
      } // end while (p[i] is equal to 0)
   } // end while (i < N)
   */