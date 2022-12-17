//use itertools::Itertools;
// use permutation::permutation::Permutation;

pub struct Fragment {
    tag: usize,
    position: i32,
}

fn main() {
    let fragment_length: i32 = 3;
    let apronym_length: usize = 4;

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

    let mut fragements: Vec<Fragment> = Vec::new();
    let mut index: usize = 0;

    for tag in tags {
        fragements.push(Fragment {tag: index, position: 1});
        index += 1;
    }

    use permutator::{Combination, Permutation};

    let mut permutations: Vec<Vec<&Fragment>> = Vec::new();
    
    let mut counter = 1;
    
    fragements.combination(apronym_length).for_each(|mut c| {
        c.permutation().for_each(|p| {
            // println!("k-permutation@{}={:?}", counter, p);
            permutations.push(p);
            
            counter += 1;
        });
    });

    for permutation in permutations {
        for fragment in permutation {
            println!("{}", tags[1]);
        }
        // for fragment in permutation {
        //     println!("{}", &tags[fragment.tag]);
        // }
        
    }

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
