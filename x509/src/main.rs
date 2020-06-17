use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate openssl;
use openssl::x509::{
    X509,
    X509NameRef
};
use openssl::hash::MessageDigest;


fn print_name_ref(field: &str, content: &X509NameRef) {

    println!("{}:", field);
    for n in content.entries() {
        println!("  {}: {:?}", n.object(), String::from_utf8(n.data().as_slice().to_vec()).unwrap());
    }
}

fn print_x509(cert: &X509) -> Result<(), std::io::Error> {
    print_name_ref("subject_name", cert.subject_name());
    print_name_ref("issuer_name", cert.issuer_name());

    println!("Not before: {}", cert.not_before());
    println!("Not after: {}", cert.not_after());

    let before: String = String::from(format!("{}",cert.not_before()));
    println!("Before as str: {}", before);

    let digest = cert.digest(MessageDigest::sha256())?;
    println!("x509 fingerprint:\n{:X?}", digest);

    println!("serialnum: {:?}", cert.serial_number().to_bn().unwrap());

    Ok(())
}

fn read_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut content: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut content)?;
    Ok(content)
}


fn decode_x509(filename: &str) -> Result<X509, std::io::Error> {
    println!("Opening: {}", filename);    
    // Load the PEM file
    let pem_file = read_file(filename)?;
    let x509 = X509::from_pem(&pem_file)?;
    Ok(x509)
}


fn decode_chain(filename: &str) -> Result<Vec<X509>, std::io::Error> {
    println!("Opening CA chain: {}", filename);
    let pem_file = read_file(filename)?;
    let x509s = X509::stack_from_pem(&pem_file)?;
    Ok(x509s)
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let ca_chain = &args[1];
    let file = &args[2];

    let cert = decode_x509(&file)?;
    let chain = decode_chain(&ca_chain)?;

    println!("Chain:\n======");
    for c in &chain {
        print_x509(&c)?;
    }

    println!("\n\nCert:\n=====");
    print_x509(&cert)?;

    // Verify the chain
    println!("\n\nVerification");
    println!("Chain issued cert:  {:?}", chain[0].issued(cert.as_ref()));
    println!("Chain is valid:     {:?}", chain[1].issued(chain[0].as_ref()));
    println!("CA did not sign cert: {:?}", chain[1].issued(cert.as_ref()));

    Ok(())
}
