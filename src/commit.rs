use std::collections::BTreeMap;
use std::io::Write;
use crypto::sha1::Sha1;
use crypto::digest::Digest;
use regex::Regex;

use super::error::TgitError;
use super::index::Index;

pub struct Commit {
  pub hash: Option<String>,
  pub data: Option<Vec<u8>>,
  pub parent: Option<String>,
  pub files: BTreeMap<String, String>,
}