use std::fs;
use std::str;
use trie_rs::TrieBuilder;

pub struct Fragment {
    tag: usize,
    position: usize,
}

fn main() {
    let fragment_length: i32 = 3;
    let apronym_length: usize = 5;

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

    let tags_reference = &tags;

    let mut fragments: Vec<Fragment> = Vec::new();
    let mut index: usize = 0;

    for tag in &tags {
        fragments.push(Fragment {tag: index, position: 1});
        index += 1;
    }

    use permutator::{Combination, Permutation};

    let mut permutations: Vec<Vec<&Fragment>> = Vec::new();
    
    let mut counter = 1;
    
    fragments.combination(apronym_length).for_each(|mut c| {
        c.permutation().for_each(|p| {
            // println!("k-permutation@{}={:?}", counter, p);
            permutations.push(p);
            
            counter += 1;
        });
    });

    let words = fs::read_to_string("C:/Users/alexa/Projects/apronymerbase/static/words.txt")
        .expect("Should have been able to read the file");
    
    let mut builder: TrieBuilder<u8> = TrieBuilder::new();

    let dictionary: Vec<&str> = words.split("\n").collect();

    for word in dictionary {
        builder.push(&word);
    }

    let trie = builder.build();

    for permutation in permutations {
        let mut apronym = String::new();
        let mut usedtags = String::new();

        for fragment in permutation {
            
           
            let s1 = String::from(tags_reference[fragment.tag]);

            apronym += &s1[..fragment.position];
            usedtags += "-";
            usedtags += tags_reference[fragment.tag];
        }

        if trie.exact_match(&apronym) {
        // if trie.exact_match("Test") {
            println!("{}={}", apronym, usedtags);
        }
    }
}
