fn main() {
    // 行前の改行をエスケープで削除
    let penguin_data = "\
common name,length (cm)
Little penguin,33
Yellow-eyed penguin,65
Fiordland penguin, 60
Invalid,data
";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        // ヘッダがある前提でスキップ
        // 空行はスキップ
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // Vec<_> のアンダースコア _ とすることで、要素型を Rust に推論させる
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();

        // この if の中は --release フラグを付けて Cargo を実行するとスキップされる
        if cfg!(debug_assertions) {
            // eprintln! は標準エラーに出力する
            // 第1引数の文字列リテラルが、出力を制御する「埋め込みミニ言語」として機能する
            // {:?} はデフォルト表現
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // Ok(length) = fields[1].parse は field[1] の f32 としての解析を試み、成功したら、その値を length に代入する
        // if let 構文は条件により処理したデータをローカル変数に渡す処理を完結に表現できる
        // if let Ok(length) とすることで Err(E) が返されるパターンをスキップすることができる
        if let Ok(length) = fields[1].parse::<f32>() {
            // println! は標準出力に出力する
            // 第1引数の文字列リテラルが、出力を制御する「埋め込みミニ言語」として機能する
            // {} はプログラマー定義で「値を文字列として表現するメソッドを使う」ことを Rust に支持する
            println!("{}, {}cm", name, length);
        }
    }
}
