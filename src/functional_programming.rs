pub fn run() {
    // 文字を出力するのは数学的な関数ではない
    println!("Hello Functional Programming");
    println!("double: {}", double(10));
}

// 数値を二倍にする関数
// 数学的な関数と言える
fn double(n: i32) -> i32 {
    n * 2
}
