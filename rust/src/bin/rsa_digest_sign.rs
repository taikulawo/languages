use openssl::{
    hash::MessageDigest,
    md::Md,
    md_ctx::MdCtx,
    pkey::{PKey, Private},
    pkey_ctx::PkeyCtx,
    rsa::{Padding, Rsa},
    sign::Verifier,
};
const PKEY: &'static [u8] = include_bytes!("../../certificates/www.example.org.key.pem");
fn sign(message: &[u8], pkey: &PKey<Private>) {
    let md = Md::sha256();
    let mut ctx = MdCtx::new().unwrap();
    ctx.digest_init(md).unwrap();
    ctx.digest_update(message).unwrap();
    let mut buf = vec![0; 1024];
    let n = ctx.digest_final(&mut buf).unwrap();
    let digest_hashed = &buf[..n];
    let mut pkey_ctx = PkeyCtx::new(&pkey).unwrap();
    pkey_ctx.sign_init().unwrap();
    pkey_ctx.set_rsa_padding(Padding::PKCS1).unwrap();
    pkey_ctx.set_signature_md(md).unwrap();
    let mut signature = vec![];
    pkey_ctx.sign_to_vec(digest_hashed, &mut signature).unwrap();
    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).unwrap();
    verifier.update(message).unwrap();
    assert!(matches!(verifier.verify(&signature), Ok(true)));
}
fn main() {
    let b = "hello,world";
    let rsa = Rsa::private_key_from_pem(PKEY).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();
    sign(b.as_bytes(), &pkey);
}
