use std::ops::Add;

fn add_gen_1<T>(a: T, b: T) -> T
where
    T: Add<Output = T>,
{
    a + b
}

fn main() {
    let a = add_gen_1(1, 2);
    let b = add_gen_1(2.1, 3.4);

    println!("{} -- {}", a, b);
}
