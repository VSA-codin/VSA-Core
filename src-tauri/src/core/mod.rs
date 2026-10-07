pub mod module_registry;
pub mod status;

pub use module_registry::{ModuleDescriptor, ModuleRegistry};
pub use status::CoreStatus;

pub mod diagnostics;
pub mod settings;
