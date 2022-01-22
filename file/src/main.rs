#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = f.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);

    read_length
}

fn main() {
    let mut f1 = File {
        name: String::from("f2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    open(&mut f1);
    let f1_length = read(&f1, &mut buffer);
    close(&mut f1);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f1);
    println!("{} is {} byte long", &f1.name, f1_length);
    println!("{}", text);

    let f2 = File::new("f1.txt");
    let f2_name = &f2.name;
    let f2_length = &f2.data.len();

    println!("{:?}", f2);
    println!("{} is {} byte long", f2_name, f2_length);
}
