use crate::fs::FileHandler;
use crate::graphs::Graph;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

impl FileHandler for Graph {
    fn save(&self) {
        let res = [self.left_to_right.clone(), self.right_to_left.clone()];
        let uuid = Uuid::new_v4();
        let name = format!("{}.txt", uuid);
        let mut file = File::create(&name).unwrap();
        let contents = serde_json::to_string(&res).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
}