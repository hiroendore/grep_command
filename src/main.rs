use std::env;
use std::io::BufRead;
use std::path::Path;

/// 標準入力から文字列を一行受け取る。
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}


/// ファイルパスを受け取って、ファイル全体を返す。
fn lines_from_file<P>(filename: P) -> Vec<String>
where
    P:AsRef<Path>,
{
    let file = std::fs::File::open(filename).expect("no such file");
    let buf = std::io::BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line")).collect()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args[1]);

    println!("一行適当に入力してENTER");
    let mut line: String = read();
    println!("{:?}", line);

    let filepath = "sample.txt";
    let text = lines_from_file(filepath);
    for linetext in text.into_iter() {
        if linetext == line {
            println!("{:?}", linetext);
        }
    }
}
