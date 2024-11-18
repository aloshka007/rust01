fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x // Повертаємо значення x
    };

    assert_eq!(v, 3);

    println!("Success!");
}
