fn main() {
    let mut t = (String::from("hello"), String::from("world"));

    // Modify this line only, don't use `_s`
    t.0 = String::from("new_hello");

    println!("{:?}", t);
}
