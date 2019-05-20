use std::env;
use std::fs;
use std::fs::File;

use std::io;
use std::io::{Read, Write};
use std::path::{PathBuf, Path};

use super::types::Blob;
use super::error::TgitError;

pub struct FileService {
  pub root_dir: PathBuf,
  pub tgit_dir: PathBuf,
  pub object_dir: PathBuf,
}

impl FileService {
  pub fn new() -> Result<FileService, TgitError> {
    let root_dir = FileService::find_root()?;
    let tgit_dir = root_dir.join(".tgit").to_path_buf();
    let object_dir = tgit_dir.join("objects").to_path_buf();
    Ok(
      FileService {
        root_dir,
        tgit_dir,
        object_dir,
      }
    )
  }
  fn find_root() -> Result<PathBuf, TgitError> {
    let mut current_dir = env::current_dir()?;
    loop {
      if FileService::is_tgit(&current_dir) {
        return Ok(current_dir);
      }
      if !current_dir.pop() {
        return Err(TgitError::NoDirectory);
      }
    }
    Ok(current_dir)
  }

  fn is_tgit<P>(path: P) -> bool
  where 
       P: Sized + AsRef<Path>  
  
  {
   path.as_ref().join(".tgit").exists()
  }

  pub fn get_head_ref(&self) -> io::Result<PathBuf> {
    let mut head_file = File::open(self.root_dir.join("tgit/HEAD"))?;
    let mut ref_path = String::new();
    head_file.read_to_string(&mut ref_path)?;
    let mut ref_path = ref_path.split_off(6);
    Ok(self.tgit_dir.join(ref_path))
  }

  pub fn write_blob(&self, blob: &Blob) -> io::Result<()> {
    self.write_object(&blob.hash, &blob.data)
  }

  pub fn write_object(&self, hash: &str, data: &Vec<u8>) -> io::Result<()> {
    let blob_dir = self.object_dir.join(&hash[..2]);
    if !blob_dir.exists() {
      fs::create_dir(&blob_dir)?;
    }

    let blob_filename = blob_dir.join(&hash[2..]);
    let mut blob_f = File::create(&blob_filename)?;
    blob_f.write_all(data)?;
    
    Ok(())
  }
}

