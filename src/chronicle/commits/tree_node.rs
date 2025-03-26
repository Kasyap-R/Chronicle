use std::collections::HashMap;

pub struct TreeNode {
    pub files: HashMap<String, String>,
    pub subdirs: HashMap<String, TreeNode>,
}

impl TreeNode {
    pub fn new() -> Self {
        TreeNode {
            files: HashMap::new(),
            subdirs: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file_name: String, file_hash: String) {
        self.files.insert(file_name, file_hash);
    }

    pub fn get_or_add_dir(&mut self, dir_name: String) -> &mut TreeNode {
        self.subdirs
            .entry(dir_name.clone())
            .or_insert_with(|| TreeNode::new())
    }
}
