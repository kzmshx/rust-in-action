fn main() {
    // needle: 針, haystack: 干し草の山
    let _needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        // match では、1個の型に関してすべての選択肢が明示的に処理されることが保証される
        // 可能な値のすべてについて分岐を提供しなければコンパイルエラーとなる
        // match には、フォールスルー（fall through）は存在しない
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}
