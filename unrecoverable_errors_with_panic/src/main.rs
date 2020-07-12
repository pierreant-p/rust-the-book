fn main() {
    let v = vec![1, 2, 3];
    v[99]; // this will cause a panic

    panic!("crash and burn");
}
