//! Shared by build.rs and the RTIC cfg macro; tuples are (system feature, chip).
//! Directory names are the feature suffix with hyphens replaced by underscores.
pub const SYSTEMS: &[(&str, &str)] = &[
    ("system-nucleo-f401re", "f401"),
    ("system-default-f401", "f401"),
    ("system-default-f405", "f405"),
    ("system-default-f411", "f411"),
];
