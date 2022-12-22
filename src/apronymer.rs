pub struct Fragment {
    tag: usize,
    position: usize,
}

pub struct Apronymer {
    tags: Vec<&str>,
    permutation: Vec<Fragment>,
    fragment_length: i32,
    apronym_length: usize,
    fragment_sets: Vec<Vec<Fragment>>,
}

impl Apronymer {
    pub fn initialize(&mut self, mut tags: Vec<&str>, fragment_size: usize) {
        self.fragment_sets = Vec<Vec<Fragment>>::new();

        for fragement_index in 0..fragment_size {
            let mut fragments = Vec<Fragment>::new();
            
            for tag_index in 0..tags.len() {       
                fragments.add(Fragement {tag: tag_index, position: fragement_index,})
            }
            self.fragment_sets.add(fragments);
        }
    }
}