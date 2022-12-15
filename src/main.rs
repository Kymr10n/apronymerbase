//use itertools::Itertools;
// use permutation::permutation::Permutation;

#[derive(Debug)]
pub struct Fragment {
    word_index: usize,
    length: usize,
}

fn main() {
    let mut length: usize = 4;

    let mut fraction: usize = 3;
    
    let mut tags: Vec<&str> = Vec::new();

    tags.push("virtual");
    tags.push("desktop");
    tags.push("environmet");
    tags.push("remote");
    tags.push("reliable");
    tags.push("secure");
    tags.push("inovative");
    tags.push("streaming");
    tags.push("platform");
    tags.push("workstation");
    tags.push("flexible");
    tags.push("scalable");

    let mut fragments: Vec<Fragment> = Vec::new();

    for index in 0..tags.len()  {
        let fragement = Fragment {
            word_index: index,
            length: 1,  
        };

        fragments.push(fragement);
    }

    use permutator::{Combination, Permutation};

    let mut permutations: Vec<Vec<&Fragment>> = Vec::new();
    
    let mut counter = 1;
    
    fragments.combination(length).for_each(|mut c| {
        c.permutation().for_each(|p| {
            println!("k-permutation@{}={:?}", counter, p);
            
            permutations.push(p);
            
            counter += 1;
        });
    });

    println!("k-permutation@{}", counter);

    //let mut fragments: Vec<Fragment> = Vec::new();

    // let vec = vec!['a','b','c','d'];
    // let permutation = Permutation::from_vec([0,2,3,1]);
    // assert_eq!(permutation.apply_slice(&vec), vec!['a','c','d','b']);

    //let perms = tags.permutations();
    /*
    let mut permutations: Vec<Vec<&&str>> = Vec::new();

    for permutation in tags.iter().permutations(word_index) {
        permutations.push(permutation);
        //permutation.first().unwrap().do_something(vpair.last().unwrap());
    }
    */
}
