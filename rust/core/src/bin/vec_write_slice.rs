fn main() {
    let mut v = Vec::with_capacity(10);
    let cap = v.capacity();
    let mut v1 = &mut v[..cap];
    v1[0] = 1;
}
