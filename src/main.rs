use std::fs;
use std::str;
use trie_rs::TrieBuilder;

mod permutator;

mod apronymer;
use apronymer::Apronymer;

pub struct Fragment {
    tag: usize,
    position: usize,
}

fn main() {
    let fragment_length: usize = 3;
    let apronym_length: usize = 5;

    let mut tags: Vec<String> = Vec::new();

    tags.push(String::from("virtual"));
    tags.push(String::from("desktop"));
    tags.push(String::from("environmet"));
    tags.push(String::from("remote"));
    tags.push(String::from("reliable"));
    tags.push(String::from("secure"));
    tags.push(String::from("inovative"));
    tags.push(String::from("streaming"));
    tags.push(String::from("platform"));
    tags.push(String::from("workstation"));
    tags.push(String::from("flexible"));
    tags.push(String::from("scalable"));

    let mut apronymer: Apronymer = Apronymer::new(tags.clone(), apronym_length, fragment_length);

    apronymer.initialize();
    apronymer.permutate();

    //parse dictionary from file into vector
    let dictionary: Vec<&str> = include_str!("../static/dictionary.txt").split("\n").collect();
    
    let mut builder: TrieBuilder<u8> = TrieBuilder::new();

    for word in dictionary {
        builder.push(&word);
    }

    let trie = builder.build();

    /* for permutation in permutations {
        let mut apronym = String::new();
        let mut usedtags = String::new();

        for fragment in apronymer.get_permutations() {
            let s1 = String::from(&tags[fragment.tag]);

            apronym += &s1[..fragment.position];
            usedtags += "-";
            usedtags += &tags[fragment.tag];
        }

        if trie.exact_match(&apronym) {
            println!("{}={}", apronym, usedtags);
        }
    } */
}
