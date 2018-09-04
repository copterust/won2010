extern crate won2010;

fn main() {
    let cal = &mut won2010::Cal::new(0.98);
    println!("{:?}", cal);
    let res = cal.step(&[
        [0.9, 0.07, 0.01],
        [0.07, 0.9, 0.01],
        [0.01, 0.9, 0.07],
        [0.01, 0.07, 0.9],
        [0.07, 0.01, 0.9],
        [0.9, 0.01, 0.07],
    ]);
    println!("{}", res);
    println!("{:?}", cal);
}
