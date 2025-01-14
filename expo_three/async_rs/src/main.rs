fn main() {
    let aux: i64 = 0;
    let aux_vec: Vec<i32> = vec![1, 2, 3];

    println!("aux_dir     : {:p}", &aux);
    println!("heaped_dir  : {:p}", &aux_vec[1]);
}
