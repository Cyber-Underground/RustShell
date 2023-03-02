use std::io::{Write, Read, self};
use std::fs::File;
use anyhow::{anyhow, Result};
use chacha20poly1305::{aead::{stream, NewAead}, XChaCha20Poly1305,};

pub fn encrypt(
    key: &[u8; 32],
    nonce: &[u8; 19],
) -> Result<(), anyhow::Error> {
    print!("    File to encrypt: ");
    io::stdout().flush().unwrap();
    let mut source_file_path = String::new();
    io::stdin().read_line(&mut source_file_path)?;
    let source_file_path = source_file_path.trim();

    let mut dist_file_path = String::from(source_file_path);
    dist_file_path.push_str(".r");

    let aead = XChaCha20Poly1305::new(key.as_ref().into());
    let mut stream_encryptor = stream::EncryptorBE32::from_aead(aead, nonce.as_ref().into());

    const BUFFER_LEN: usize = 500;
    let mut buffer = [0u8; BUFFER_LEN];

    let mut source_file = File::open(source_file_path)?;
    let mut dist_file = File::create(&dist_file_path)?;

    loop {
        let read_count = source_file.read(&mut buffer)?;

        if read_count == BUFFER_LEN {
            let ciphertext = stream_encryptor
                .encrypt_next(buffer.as_slice())
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
        } else {
            let ciphertext = stream_encryptor
                .encrypt_last(&buffer[..read_count])
                .map_err(|err| anyhow!("Encrypting large file: {}", err))?;
            dist_file.write(&ciphertext)?;
            break;
        }
    }

    std::fs::remove_file(source_file_path)?;

    Ok(())
}

pub fn decrypt(key: &[u8; 32], nonce: &[u8; 19]) -> Result<(), anyhow::Error> {
  print!("    File to decrypt: ");
  io::stdout().flush().unwrap();
  let mut encrypted_file_path = String::new();
  io::stdin().read_line(&mut encrypted_file_path)?;
  let encrypted_file_path = encrypted_file_path.trim();

  let mut dist_file_path = encrypted_file_path.to_string();
  if dist_file_path.ends_with(".r") {
    dist_file_path.pop();
  }

  let aead = XChaCha20Poly1305::new(key.as_ref().into());
  let mut stream_decryptor = stream::DecryptorBE32::from_aead(aead, nonce.as_ref().into());

  const BUFFER_LEN: usize = 500 + 16;
  let mut buffer = [0u8; BUFFER_LEN];

  let mut encrypted_file = File::open(encrypted_file_path)?;
  let mut dist_file = File::create(&dist_file_path)?;

  loop {
    let read_count = encrypted_file.read(&mut buffer)?;

    if read_count == BUFFER_LEN {
      let plaintext = stream_decryptor
        .decrypt_next(buffer.as_slice())
        .map_err(|err| {
          std::fs::remove_file(&dist_file_path).unwrap();
          anyhow!("Decrypting large file: {}", err)
        })?;
      dist_file.write(&plaintext)?;
    } else if read_count == 0 {
        break;
    } else {
      let plaintext = stream_decryptor
        .decrypt_last(&buffer[..read_count])
        .map_err(|err| {
          std::fs::remove_file(&dist_file_path).unwrap();
          anyhow!("Decrypting large file: {}", err)
        })?;
      dist_file.write(&plaintext)?;
      break;
    }
  }

  std::fs::remove_file(encrypted_file_path).unwrap();
  Ok(())
}
