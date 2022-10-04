use soft_float::F64;

fn main() {
    let a1 = F64::from(1);
    let b1 = F64::from(3);
    let c1 = a1 / b1;

    println!("{c1}");

    let a2 = F64::from(2);
    let b2 = F64::from(6);
    let c2 = a2 / b2;

    println!("{c2}");

    assert_eq!(c1, c2);

    println!("Hello, world!");
}
