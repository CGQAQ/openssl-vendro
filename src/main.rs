use std::fs;

use openssl::pkcs12::Pkcs12;
use openssl::pkcs12::ParsedPkcs12_2;
use openssl::x509::X509;
use openssl::pkey::PKey;
use openssl::nid::Nid;

use openssl::version::version;

const CERT: &str = include_str!("../cert.pem");
const CERT_KEY: &str = include_str!("../cert-key.pem");

fn main() {
    println!("version: {}", version());
    
    let cert = X509::from_pem(CERT.as_bytes()).unwrap();
    let cert_key = PKey::private_key_from_pem(CERT_KEY.as_bytes()).unwrap();

    let p12 = Pkcs12::builder()
        .pkey(&cert_key)
        .key_algorithm(Nid::AES_256_CBC)
        .cert(&cert)
        .cert_algorithm(Nid::AES_256_CBC)
        .build2("123")
        .unwrap();


    let der = p12.to_der().unwrap();

    fs::write("cert.p12", der).unwrap();
}
