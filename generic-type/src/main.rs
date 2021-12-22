use std::ops::Add;
use std::time::Duration;

///
/// # 型 T に加算のサポートが必須であることを指示する
///
/// → 関数定義に、型変数とともにトレイト境界（trait bound）を入れる
///
/// # <T: std::ops::Add<Output = T>>
///
/// `T` は `std::ops::Add` を実装しており、引数 `i`, `j` および返り値の型が同じである必要がある
///
/// # トレイト
///
/// インターフェース、プロトコル、コントラクト（契約）に例えられる。
/// オブジェクト指向プログラミングにおける抽象基底クラスのようなもの。
/// 関数型プログラミングにおける Haskell の型クラスのようなもの。
///
/// Rust の演算子は、すべてトレイト付きで定義されている。
/// Rust の演算子は、トレイトのメソッド群の糖衣構文（syntactic sugar）である。
///
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{}", floats);
    println!("{}", ints);

    // std::time::Duration は std::fmt::Display トレイトを実装しないが、フォールバックで std::fmt::Debug を要求する
    println!("{:?}", durations);
}
