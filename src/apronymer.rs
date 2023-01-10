use crate::permutator;
//use permutator::{Combination, Permutation}; //external crate; remove when reimplemented

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Fragment {
    tag: usize,
    position: usize,
}

pub struct Apronymer<'a> {
    tags: Vec<String>,
    permutations: Vec<Vec<&'a Fragment>>,
    fragment_length: usize,
    apronym_length: usize,
    fragment_sets: Vec<Vec<Fragment>>,
}

impl<'a> Apronymer<'a> {
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
            let mut set: Vec<Fragment> = Vec::new();
            
            let mut counter = combination.clone();

            let mut position;

            for tag_index in 0..self.tags.len() {  
                position = counter % self.fragment_length;
                counter = counter / self.fragment_length;
                
                set.push(Fragment {tag: tag_index, position: position,})
            }

            self.fragment_sets.push(set);
        }
    }

    pub fn permutate(&mut self) {
        permutator::permutate(&mut vec!["A", "B", "C"]);
/*         for fragments in self.fragment_sets {
            fragments.combination(self.apronym_length).for_each(|mut c| {
                c.permutation().for_each(|p| {
                    self.permutations.push(p);
                });
            }); 
        }*/
    }

/*     pub fn get_permutations(&mut self) -> Vec<Fragment> {
        return self.permutations;
    } */
}