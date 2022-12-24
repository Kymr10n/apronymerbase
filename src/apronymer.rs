use permutator::{Combination, Permutation};

#[derive(Debug)]
pub struct Fragment {
    tag: usize,
    position: usize,
}

pub struct Apronymer<'a> {
    tags: Vec<String>,
    permutations: Vec<&'a Fragment>,
    fragment_length: usize,
    apronym_length: usize,
    fragment_sets: Vec<Vec<Fragment>>,
}

impl Apronymer<'a> {
    pub fn new(tags: Vec<String>, apronym_length: usize, fragment_length: usize) -> Self {
        return Apronymer {
            tags: tags,
            permutations: Vec::new(),
            fragment_length: fragment_length,
            apronym_length: apronym_length,
            fragment_sets: Vec::new(),
        };
    }

    pub fn initialize(&mut self) {
        for combination in 0..self.fragment_length.pow(self.tags.len() as u32) {
            let mut fragments: Vec<Fragment> = Vec::new();
            
            let mut helper = combination.clone();

            let mut position = 0;

            for tag_index in 0..self.tags.len() {  
                position = helper % self.fragment_length;
                helper = helper / self.fragment_length;
                
                fragments.push(Fragment {tag: tag_index, position: position,})
            }
            self.fragment_sets.push(fragments);
        }
    }

    pub fn permutate(&mut self) {
        for fragments in self.fragment_sets {
            fragments.combination(self.apronym_length).for_each(|mut c| {
                c.permutation().for_each(|p| {
                    self.permutations.push(p);
                });
            });
        }
    }

    pub fn get_permutations(&mut self) -> Vec<Fragment> {
        return self.permutations;
    }
}