pub mod backend;
pub mod events;
mod handlers;
pub mod mir_loc;

use std::env;

/// List of functions we want hooked for the lifetime analyis runtime.
pub const HOOK_FUNCTIONS: &[&str] = &["malloc", "free", "calloc", "realloc", "reallocarray"];

pub use self::mir_loc::{DefPathHash, Metadata, MirLoc, MirLocId};

pub use self::handlers::*;

pub fn initialize() {
    let span_filename = env::var("METADATA_FILE")
        .expect("Instrumentation requires the METADATA_FILE environment variable be set");
    mir_loc::set_file(&span_filename);
    backend::init();
}

pub fn finalize() {
    backend::finalize();
}
