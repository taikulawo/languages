use url::Url;

fn main() {
    let u: Url = "https://$backend/$uri?name=$alpn".parse().unwrap();
    println!("{:?}, {:?} {:?}", u.host(), u.path(), u.query());
    let u: Url = "../main.rs".parse().unwrap();
    println!("{:?}, {:?} {:?}", u.host(), u.path(), u.query());
    println!("Hello, world!");
}
