use crate::fs::FileHandler;
use crate::graphs::Graph;

impl FileHandler for Graph {
    fn save(&self) {
        println!("Hello World!")
    }
}