use num::Complex;

fn main() {
    // Rust にはコンストラクタはないが、リテラル構文がある
    let a = Complex { re: 2.1, im: -1.2 };
    // 簡略にインスタンス化するための静的メソッド new を用意しているクレートが多い
    // 静的メソッドはある型で利用できるが、その型のインスタンスではない関数である
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}
