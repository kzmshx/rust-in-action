fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);
    println!("{}", res);
}

// <'a, 'b> は add_with_lifetimes をスコープとする2つのランタイム変数 'a, 'b を宣言している
// i: &'a i32 はライフタイム変数 'a を i のライフタイムに束縛する
//      つまり、パラメータ i はライフタイム a を持つ i32 型への参照である
// j: &'b i32 はライフタイム変数 'b を i のライフタイムに束縛する
//      つまり、パラメータ j はライフタイム b を持つ i32 型への参照である
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}
