///
/// # Rust の豊富な文字列型
///
/// String と &str はどちらも文字列を表現するが別々の型である。
///
/// ## `String`
///
/// - 他の言語における文字列型に近い。
///     - 連結（concatenation）や追加（append）、空白文字の除去などの一般的な演算をサポートする。
/// - `String` のテキスト格納には動的なメモリ割り当てが使われる。
/// - `String` は所有型（owned type）である。
/// - `String` は読み書きできる read-write データである。
///
/// ## `&str`
///
/// - 高性能の代わりに、機能は少ない。
///     - 拡張や収縮は不可能である。
///     - 文字コードの単なる配列を扱う感覚に近いが、UTF-8 としての有効性は保証される。
/// - `str` は通常 `&str` （文字列スライス）として登場する。
///     - これは `str` データの参照と長さを含む型である。
///     - `&str` のテキスト格納には動的なメモリ割り当てが使われない。
/// - `&str` は借用型（borrowed type）である。
/// - `&str` は読み出し専用の read-only データである。
///
/// ## その他
///
/// - `char`
///     - 1文字を4バイトにエンコードするキャラクタ型
///     - `&str` や `String` は1文字を UTF-8 としてエンコードするが、`char` は UCS-4/UTF-32 と等価である
/// - `u8`
///     - 生（未加工）バイトデータのスライス
///     - バイナリデータのストリーム処理で使われる
/// - `Vec<u8>`
///     - 生バイトのベクター
///     - `u8` データを消費するときに使われることが多い
/// - `std::ffi::OSString`
///     - プラットフォームネイティブな文字列
///     - 振る舞いは `String` に近い
///     - UTF-8 でエンコードされる保証はない
///     - ゼロバイト（0x00）を含まない保証はない
/// - `std::path::Path`
///     - ファイルシステムのパスを扱うための専用の文字列型
///
fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books.
What do we seek through millions of pages?";

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}
