use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    // move toma ownership de los objetos que mueve al closure
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    //println!("{:?}", numbers);

    let numbers = vec![1, 2, 3];

    let sum: i32 = thread::spawn(move || numbers.into_iter().sum())
        .join()
        .unwrap();

    println!("suma: {}", sum);
}
