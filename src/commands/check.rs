use std::path::PathBuf;

pub struct CheckCommand {
    pub defconfig: PathBuf,
    pub verbose: bool
}

impl CheckCommand {
    pub fn new(defconfig: PathBuf, verbose: bool) -> Self {
        Self {
            defconfig: defconfig,
            verbose: verbose
        }
    }

    pub fn run(&self) {
        println!("check command launched!")
    }
}
