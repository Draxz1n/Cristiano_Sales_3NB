
mod lib;

fn main() {
    let arr = [2, 3, 4];
    let product = unsafe { lib::multiply_array(arr.as_ptr(), arr.len()) };
    println!("Produto dos elementos: {}", product);
    assert_eq!(product, 24);
}
