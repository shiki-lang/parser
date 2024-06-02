use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use swc_ecma_parser::{Parser, StringInput, Syntax};


fn parse(file: &str) {
}

fn main() {
    // ファイルパスを指定
    let test_path = "C:\\Users\\kishi\\.aa_myfile\\GitHub\\parser\\test\\test.shiki";
    let path = Path::new(test_path);

    // ファイルが存在しないあるいはshikiファイルでない場合はエラー
    if path.extension().unwrap() != "shiki" {
        panic!("{} is not a shiki file", path.display());
    }

    // ファイルを開く
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    // ファイルの内容を読み込む
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => print!("{} contains:\n{}", path.display(), s),
    }

    // ファイルをパースして構文木を作成
    parse(&s);
}
