fn main() {
    let i: f64 = 31415.926e4f64;

    assert_eq!(i, 314159260.0);
    println!("{i}");

    let mut arr: [u8; 5] = [0, 1, 2, 3, 4];
    let mut v1: Vec<u32> = vec![0, 1, 2, 3, 4];

    v1.remove(2);
    println!("{:?}", v1);

    {
        let fe = arr.iter_mut().nth(0);
        if let Some(e) = fe {
            *e = 6;
        }
    }

    let slice_1 = &mut arr[0..3];
    let fe = slice_1.iter_mut().nth(2);
    if let Some(e) = fe {
        *e = 7
    }

    println!("{:?}", arr);

    println!(
        "in the room thewomen come and go,
    Singin of mount abora"
    );
    println!(
        "It was a bright, cold day in April, and \
        there were four of us \
        more or less."
    );

    let method = b"GET";
    println!("{:?}", method);
}
