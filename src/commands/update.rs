use std::path::PathBuf;
use crate::Mode;

pub struct UpdateCommand {
    pub defconfig: PathBuf,
    pub mode: Mode,
    pub verbose: bool
}

impl UpdateCommand {
    pub fn new(defconfig: PathBuf, mode: Mode, verbose: bool) -> Self {
        Self {
            defconfig: defconfig,
            mode: mode,
            verbose: verbose
        }
    }

    pub fn run(&self) {
        println!("update command launched!")
    }
}
