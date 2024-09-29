use std::fs::File;

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    XChaCha20Poly1305, XNonce
};



fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[+]Usage: {} <filename>", args[0]);
        return;
    }

    let file = File::open(&args[1]);

    if file.is_err() {
        println!("[+]Error opening file: {:?}", file.err().unwrap());
        return;
    }

    //convert the file to a byte array
    let file_bytes = std::fs::read(&args[1]);
    if file_bytes.is_err() {
        println!("[+]Error reading file: {:?}", file_bytes.err().unwrap());
        return;
    }

    let file_bytes = file_bytes.unwrap();

    let key = XChaCha20Poly1305::generate_key(&mut OsRng);
    let cipher = XChaCha20Poly1305::new(&key);
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng); // 192-bit nonce for safety

    let encrypt_file = cipher.encrypt(&nonce, file_bytes.as_ref());

    match encrypt_file {
        Ok(encrypted) => {
            let mut file = File::create("Do_not_cry").unwrap();
            std::io::Write::write_all(&mut file, &encrypted).unwrap();
        }
        Err(e) => {
            println!("[+]Error encrypting file: {:?}", e);
        }
    }
    


}
