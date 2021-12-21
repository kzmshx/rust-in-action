fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < (b as i32) {
        println!("10は100より小さい。");
    }

    // トレイトをローカルスコープにインポートする
    // トレイトはメソッドのコレクション
    // オブジェクト指向プログラミング言語における抽象クラスまたはインターフェースのようなもの
    // 関数型プログラミング言語における型クラスのようなもの
    use std::convert::TryInto;

    // ローカルスコープにトレイトが存在するときだけ、型のメソッドを呼び出せる
    // ただし、暗黙のプレリュードによって一般的な演算メソッドはインポートなしで呼び出せる
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("10は100より小さい。");
    }
}
