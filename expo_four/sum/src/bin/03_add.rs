//Rhs es el valor a la derecha del signo +
trait MyAdd<Rhs> {
    type Output;

    fn my_add(self, rhs: Rhs) -> Self::Output;
}

impl MyAdd<f64> for i32 {
    type Output = f64;

    fn my_add(self, rhs: f64) -> f64 {
        self as f64 + rhs
    }
}

impl MyAdd<i32> for f64 {
    type Output = f64;

    fn my_add(self, rhs: i32) -> f64 {
        self + rhs as f64
    }
}

impl MyAdd<i32> for i32 {
    type Output = i32;

    fn my_add(self, rhs: i32) -> i32 {
        self + rhs
    }
}

impl MyAdd<f64> for f64 {
    type Output = f64;

    fn my_add(self, rhs: f64) -> f64 {
        self + rhs
    }
}

fn add_gen_custom<T, U>(a: T, b: U) -> T::Output
where
    T: MyAdd<U>,
{
    a.my_add(b)
}

fn main() {
    let x = add_gen_custom(1, 2.5); // i32 + f64
    println!("{}", x); // Output: 3.5

    let y = add_gen_custom(4.2, 7); // f64 + i32
    println!("{}", y); // Output: 11.2

    let z = add_gen_custom(10, 20); // i32 + i32
    println!("{}", z); // Output: 30
}
