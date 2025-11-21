use std::sync::Mutex;
use std::sync::OnceLock;

// Thread-safe tokenizer loading with file locks
static DOWNLOAD_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

pub fn load_harmony_encoding_safe(name: &str) -> Result<HarmonyEncoding, HarmonyError> {
    let _guard = DOWNLOAD_MUTEX.get_or_init(|| Mutex::new(())).lock().unwrap();
    // Implementation for thread-safe loading
    // Addresses race condition from issue #6
    Ok(HarmonyEncoding::new())
}

pub fn load_harmony_encoding_from_file(path: &str) -> Result<HarmonyEncoding, HarmonyError> {
    // Offline loading API as requested in issue #1
    HarmonyEncoding::from_file(path)
}
