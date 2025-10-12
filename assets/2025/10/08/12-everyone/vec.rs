fn main() {
    let mut v = vec![1, 2, 3];
    let num = &v[2];
    v.push(4);
    println!("{}", *num);
}
