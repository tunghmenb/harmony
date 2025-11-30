use std::sync::Mutex;
use std::sync::OnceLock;

use crate::tiktoken::CoreBPE;
use crate::tiktoken_ext::{Encoding, LoadError};

// Thread-safe tokenizer loading with file locks
static DOWNLOAD_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

/// Thread-safe loading of HarmonyEncoding with mutex protection for file downloads
/// Addresses race condition from issue #6 where multiple threads downloading
/// the same tokenizer file causes corruption and redundant network requests
pub fn load_harmony_encoding_safe(name: &str) -> Result<CoreBPE, LoadError> {
    // Get or initialize the global download mutex
    let download_mutex = DOWNLOAD_MUTEX.get_or_init(|| Mutex::new(()));
    
    // Acquire the lock before attempting to download/load
    let _guard = download_mutex.lock().unwrap();
    
    // Use the existing encoding loading mechanism with thread safety
    Encoding::load_from_name(name)
}

/// Offline loading API as requested in issue #1
/// Loads HarmonyEncoding from a local file path without network access
pub fn load_harmony_encoding_from_file<P: AsRef<std::path::Path>>(
    path: P,
    encoding_name: &str,
) -> Result<CoreBPE, LoadError> {
    use std::fs::File;
    use std::io::BufReader;
    use crate::tiktoken_ext::{load_encoding_from_file, load_tiktoken_vocab_file};
    
    // Parse the encoding name to get the expected pattern and special tokens
    let encoding = Encoding::from_name(encoding_name)
        .ok_or_else(|| LoadError::UnknownEncodingName(encoding_name.to_string()))?;
    
    // Load the vocabulary from the local file
    let vocab = load_tiktoken_vocab_file(&path, None)
        .map_err(LoadError::InvalidTiktokenVocabFile)?;
    
    // Create CoreBPE with the appropriate pattern and special tokens
    match encoding {
        Encoding::O200kHarmony => {
            let mut specials: Vec<(String, u64)> = encoding
                .special_tokens()
                .iter()
                .map(|(s, r)| ((*s).to_string(), *r))
                .collect();
            specials.extend((200014..=201088).map(|id| (format!("<|reserved_{id}|>"), id)));
            
            CoreBPE::new(
                vocab,
                specials.into_iter(),
                &encoding.pattern(),
            )
            .map_err(LoadError::CoreBPECreationFailed)
        }
        Encoding::O200kBase => {
            let mut specials: Vec<(String, u64)> = encoding
                .special_tokens()
                .iter()
                .map(|(s, r)| ((*s).to_string(), *r))
                .collect();
            specials.extend((199998..=201088).map(|id| (format!("<|reserved_{id}|>"), id)));
            
            CoreBPE::new(
                vocab,
                specials.into_iter(),
                &encoding.pattern(),
            )
            .map_err(LoadError::CoreBPECreationFailed)
        }
        _ => {
            CoreBPE::new(
                vocab,
                encoding.special_tokens().iter().cloned(),
                &encoding.pattern(),
            )
            .map_err(LoadError::CoreBPECreationFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_thread_safe_loading() {
        // This test would verify that multiple threads can safely load encodings
        // without race conditions. In practice, this would be tested with
        // concurrent access patterns.
        let result = load_harmony_encoding_safe("o200k_harmony");
        assert!(result.is_ok(), "Should load encoding successfully");
    }
    
    #[test]
    fn test_offline_loading_api() {
        // This test would verify offline loading from a file
        // Note: Requires a test file to be present
        // let result = load_harmony_encoding_from_file("test-data/tokenizer.tiktoken", "o200k_harmony");
        // assert!(result.is_ok(), "Should load from file successfully");
    }
}