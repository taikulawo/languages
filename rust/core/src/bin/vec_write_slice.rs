fn main() {
    let mut v = vec![0, 10];
    let cap = v.capacity();
    let mut v1 = &mut v[..cap];
    v1[0] = 1;
    let mut v = Vec::with_capacity(10);
    let cap = v.capacity();
    let mut v1 = &mut v[..cap];
    v1[0] = 1;
}
