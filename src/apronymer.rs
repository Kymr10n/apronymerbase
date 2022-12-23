#[derive(Debug)]
pub struct Fragment {
    tag: usize,
    position: usize,
}

pub struct Apronymer {
    tags: Vec<String>,
    permutation: Vec<Fragment>,
    fragment_length: i32,
    apronym_length: usize,
    fragment_sets: Vec<Vec<Fragment>>,
}

impl Apronymer {
    pub fn new() -> Self {
        return Apronymer {
            tags: Vec::new(),
            permutation: Vec::new(),
            fragment_length: 0,
            apronym_length: 0,
            fragment_sets: Vec::new(),
        };
    }

    pub fn initialize(&mut self, mut tags: Vec<&str>, fragment_size: usize) {
        for fragement_index in 0..fragment_size {
            let mut fragments: Vec<Fragment> = Vec::new();
            
            for tag_index in 0..tags.len() {       
                fragments.push(Fragment {tag: tag_index, position: fragement_index,})
            }
            self.fragment_sets.push(fragments);
        }
    }
}