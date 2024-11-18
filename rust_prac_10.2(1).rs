struct Array<T, const N: usize> {
    data: [T; N],
}

fn main() {
    let array1 = Array {
        data: [1, 2, 3],
    };
    
    let array2 = Array {
        data: [1.0, 2.0, 3.0],
    };

   
    let array3 = Array {
        data: [1, 2],
    };

    println!("Success!");
}
