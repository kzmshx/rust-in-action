fn main() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    // haystack に含まれる各要素への参照を反復処理する
    for item in &haystack {
        // *item は item の参照先を返す
        if *item == needle {
            println!("{}", item);
        }
    }
}
