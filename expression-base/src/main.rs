fn main() {
    // Rust は式ベースのプログラミング言語である
    // 式ベースの言語では、すべての式（Expression）が値を返し、ほとんどすべてのものが式である

    // if は式である
    let n1 = 123456;
    let description = if is_even(n1) { "Even" } else { "Odd" };
    println!("{} は {}", n1, description);

    // match は式である
    let n2 = 654321;
    let description = match is_even(n2) {
        true => "Even",
        false => "Odd",
    };
    println!("{} は {}", n2, description);

    // break は式である
    let n3 = loop {
        break 123;
    };
    println!("{}", n3);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
