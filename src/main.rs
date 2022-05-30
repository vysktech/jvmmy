mod attr;
mod class;
mod method;
mod constant_pool;
mod field;
mod util;

use crate::class::ClassFile;
use crate::class::loading::ClassFileLoader;

use std::fs;
use std::fs::read;

fn main() {
    let dir = "/home/vysk/dev/jvm/jvmmy";
    let filename = "HelloWorld.class";
    let path = format!("{}/{}", dir, filename);
    let contents = read(path).expect("Failed to read file");

    let mut loader = ClassFileLoader::new(contents);
    let class_file = loader.load();
}
