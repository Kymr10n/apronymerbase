mod permutator {
   pub fn permutate(source: Vec<& T>) {
      let mut length = source.len;
      let mut iterator: Vec<u32> = Vec::new();
      //let mut iterator: Vec<usize> = vec![0; length+1];

      for i in 0..length {
         iterator.push(i);
      }

      let mut iterator: Vec<usize> = vec![0; length+1];

      let mut index = 1;

      while (index < length) {
         let mut j;

         p[index] -= 1;

         match(index % 2 = 0) {
            true => j = p[index],
            false => j = 0
         }

         swap(source[j], source[index]);

         index = 1;

         while (iterator[index] = 0) {
            iterator[index] = index;
            index += 1;
         }
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