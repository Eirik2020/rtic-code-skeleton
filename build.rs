use std::{env, fs, path::PathBuf};

#[path = "src/systems/catalog.rs"]
mod systems_catalog;

use syn::visit_mut::VisitMut;
use syn::{Attribute, Field, Item, Lit, Meta, parse_file, visit_mut};

struct CfgReducer;

impl CfgReducer {
    fn feature_enabled(name: &str) -> bool {
        env::var_os(format!(
            "CARGO_FEATURE_{}",
            name.to_uppercase().replace('-', "_")
        ))
        .is_some()
    }

    fn meta_enabled(meta: &Meta) -> bool {
        match meta {
            Meta::NameValue(value) if value.path.is_ident("feature") => {
                matches!(&value.value, syn::Expr::Lit(expr) if matches!(&expr.lit, Lit::Str(feature) if Self::feature_enabled(&feature.value())))
            }
            Meta::List(list) if list.path.is_ident("all") => list
                .parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )
                .map(|items| items.iter().all(Self::meta_enabled))
                .unwrap_or(false),
            Meta::List(list) if list.path.is_ident("any") => list
                .parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )
                .map(|items| items.iter().any(Self::meta_enabled))
                .unwrap_or(false),
            Meta::List(list) if list.path.is_ident("not") => list
                .parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )
                .ok()
                .and_then(|items| items.first().map(|item| !Self::meta_enabled(item)))
                .unwrap_or(false),
            _ => false,
        }
    }

    fn cfg_enabled(attr: &Attribute) -> bool {
        match &attr.meta {
            Meta::List(list) => list
                .parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )
                .ok()
                .and_then(|items| items.first().map(Self::meta_enabled))
                .unwrap_or(true),
            _ => true,
        }
    }

    fn enabled(attrs: &[Attribute]) -> bool {
        attrs
            .iter()
            .filter(|attr| attr.path().is_ident("cfg"))
            .all(Self::cfg_enabled)
    }

    fn clean_field(field: &mut Field) {
        field.attrs.retain(|attr| !attr.path().is_ident("cfg"));
    }
}

impl visit_mut::VisitMut for CfgReducer {
    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        match &mut item.fields {
            syn::Fields::Named(fields) => {
                fields.named = std::mem::take(&mut fields.named)
                    .into_iter()
                    .filter(|field| Self::enabled(&field.attrs))
                    .map(|mut field| {
                        Self::clean_field(&mut field);
                        field
                    })
                    .collect();
            }
            syn::Fields::Unnamed(fields) => {
                fields.unnamed = std::mem::take(&mut fields.unnamed)
                    .into_iter()
                    .filter(|field| Self::enabled(&field.attrs))
                    .map(|mut field| {
                        Self::clean_field(&mut field);
                        field
                    })
                    .collect();
            }
            syn::Fields::Unit => {}
        }
        visit_mut::visit_item_struct_mut(self, item);
    }

    fn visit_expr_struct_mut(&mut self, expr: &mut syn::ExprStruct) {
        expr.fields = std::mem::take(&mut expr.fields)
            .into_iter()
            .filter(|field| Self::enabled(&field.attrs))
            .map(|mut field| {
                field.attrs.retain(|attr| !attr.path().is_ident("cfg"));
                field
            })
            .collect();
        visit_mut::visit_expr_struct_mut(self, expr);
    }
}

fn has_disabled_cfg(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path().is_ident("cfg") && !CfgReducer::cfg_enabled(attr))
}

fn reduce_body(source: &str) -> String {
    let mut file = parse_file(source).expect("parse app body");
    // The editable skeleton is now a real module. Only the legacy negative
    // fixture needs its contents as text; generated-body fixtures stay flat.
    if file.items.len() == 1 {
        if let Item::Mod(module) = &mut file.items[0] {
            if module.ident == "app" {
                file.items = module.content.take().expect("inline app module").1;
            }
        }
    }
    CfgReducer.visit_file_mut(&mut file);
    file.items.retain(|item| match item {
        Item::Const(item) => !has_disabled_cfg(&item.attrs),
        Item::Enum(item) => !has_disabled_cfg(&item.attrs),
        Item::ExternCrate(item) => !has_disabled_cfg(&item.attrs),
        Item::Fn(item) => !has_disabled_cfg(&item.attrs),
        Item::ForeignMod(item) => !has_disabled_cfg(&item.attrs),
        Item::Impl(item) => !has_disabled_cfg(&item.attrs),
        Item::Macro(item) => !has_disabled_cfg(&item.attrs),
        Item::Mod(item) => !has_disabled_cfg(&item.attrs),
        Item::Static(item) => !has_disabled_cfg(&item.attrs),
        Item::Struct(item) => !has_disabled_cfg(&item.attrs),
        Item::Trait(item) => !has_disabled_cfg(&item.attrs),
        Item::TraitAlias(item) => !has_disabled_cfg(&item.attrs),
        Item::Type(item) => !has_disabled_cfg(&item.attrs),
        Item::Union(item) => !has_disabled_cfg(&item.attrs),
        Item::Use(item) => !has_disabled_cfg(&item.attrs),
        Item::Verbatim(_) => true,
        _ => true,
    });
    prettyplease::unparse(&file)
}

fn read_config(path: &str) -> toml::Value {
    println!("cargo:rerun-if-changed={path}");
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read {path}: {error}"))
        .parse()
        .unwrap_or_else(|error| panic!("parse {path}: {error}"))
}

fn config_str<'a>(config: &'a toml::Value, key: &str) -> &'a str {
    config
        .get(key)
        .and_then(toml::Value::as_str)
        .unwrap_or_else(|| panic!("configuration requires string '{key}'"))
}

/// A system selects an exact part from the chip's reusable part catalog.
/// Chip-only builds use the catalog's explicit default part for compile tests.
fn memory_layout(chip: &str) -> String {
    let config = read_config(&format!("src/chips/{chip}/config.toml"));
    assert_eq!(config_str(&config, "chip"), chip, "chip config mismatch");
    assert_eq!(
        config_str(&config, "cargo_feature"),
        chip,
        "chip feature mismatch"
    );
    let selected_systems: Vec<_> = systems_catalog::SYSTEMS
        .iter()
        .filter(|(feature, _)| CfgReducer::feature_enabled(feature))
        .collect();
    assert!(
        selected_systems.len() <= 1,
        "select at most one system feature"
    );
    let system = selected_systems.first().map(|&&(feature, expected_chip)| {
        assert_eq!(expected_chip, chip, "system/chip mismatch");
        let directory = feature.strip_prefix("system-").unwrap().replace('-', "_");
        let system = read_config(&format!("src/systems/{directory}/config.toml"));
        assert_eq!(
            config_str(&system, "cargo_feature"),
            feature,
            "system feature mismatch"
        );
        system
    });
    let part_name = if let Some(system) = &system {
        assert_eq!(config_str(system, "chip"), chip, "system/chip mismatch");
        // Generated-body is a synthetic fixture, not the selected system firmware.
        assert!(
            !CfgReducer::feature_enabled("generated-body"),
            "system features cannot be combined with the generated-body fixture"
        );
        config_str(system, "part")
    } else {
        config_str(&config, "default_part")
    };
    let part = config
        .get("parts")
        .and_then(|parts| parts.get(part_name))
        .unwrap_or_else(|| panic!("part '{part_name}' is not registered for chip '{chip}'"));
    if let Some(system) = &system {
        let embed = read_config("Embed.toml");
        let profile = config_str(system, "embed_profile");
        let general = embed
            .get(profile)
            .and_then(|profile| profile.get("general"))
            .unwrap_or_else(|| panic!("missing Embed.toml profile '{profile}'"));
        assert_eq!(
            config_str(general, "chip"),
            config_str(part, "probe_chip"),
            "system probe profile does not match the selected MCU part"
        );
    }
    let size = |key| {
        part.get(key)
            .and_then(toml::Value::as_integer)
            .filter(|size| *size > 0)
            .unwrap_or_else(|| panic!("part '{part_name}' requires a positive '{key}'"))
    };
    let mut memory = format!(
        "MEMORY\n{{\n  FLASH : ORIGIN = 0x08000000, LENGTH = {}K\n  RAM : ORIGIN = 0x20000000, LENGTH = {}K\n",
        size("flash_kib"),
        size("ram_kib")
    );
    if part.get("ccm_kib").is_some() {
        memory.push_str(&format!(
            "  CCM : ORIGIN = 0x10000000, LENGTH = {}K\n",
            size("ccm_kib")
        ));
    }
    memory.push_str("}\n");
    memory
}

fn main() {
    println!("cargo:rerun-if-changed=src/app_body_skeleton.rs");
    for chip in ["f401", "f405", "f411"] {
        println!("cargo:rerun-if-changed=src/chips/{chip}/app_head.rs");
        println!("cargo:rerun-if-changed=src/chips/{chip}/config.toml");
        println!("cargo:rerun-if-changed=src/chips/{chip}/app_body.rs");
    }
    println!("cargo:rerun-if-changed=src/invalid_interrupt.rs");
    for feature in [
        "CARGO_FEATURE_F401",
        "CARGO_FEATURE_F405",
        "CARGO_FEATURE_F411",
    ] {
        println!("cargo:rerun-if-env-changed={feature}");
    }

    let chips: Vec<&str> = [
        ("CARGO_FEATURE_F401", "f401"),
        ("CARGO_FEATURE_F405", "f405"),
        ("CARGO_FEATURE_F411", "f411"),
    ]
    .into_iter()
    .filter_map(|(env_name, chip)| env::var_os(env_name).map(|_| chip))
    .collect();
    assert_eq!(chips.len(), 1, "exactly one chip feature must be selected");
    let chip = chips
        .first()
        .copied()
        .unwrap_or_else(|| panic!("one chip feature must be selected"));
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("memory.x"), memory_layout(chip)).expect("write memory.x");
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Normal development compiles the source module through a span-preserving
    // attribute macro. Text composition is only for the existing test fixtures.
    if env::var_os("CARGO_FEATURE_GENERATED_BODY").is_none()
        && env::var_os("CARGO_FEATURE_INVALID_INTERRUPT").is_none()
    {
        return;
    }
    let body_path = if env::var_os("CARGO_FEATURE_GENERATED_BODY").is_some() {
        format!("src/chips/{chip}/app_body.rs")
    } else {
        "src/app_body_skeleton.rs".to_owned()
    };
    let body_source = fs::read_to_string(body_path).expect("read app body");
    let body = reduce_body(&body_source);
    let heads: Vec<String> = if env::var_os("CARGO_FEATURE_INVALID_INTERRUPT").is_some() {
        vec!["src/invalid_interrupt.rs".to_owned()]
    } else {
        chips
            .iter()
            .map(|chip| format!("src/chips/{chip}/app_head.rs"))
            .collect()
    };

    let indented_body: String = body
        .lines()
        .map(|line| {
            if line.is_empty() {
                "\n".to_owned()
            } else {
                format!("    {line}\n")
            }
        })
        .collect();

    let mut app = String::from(
        "// WARNING: Generated code. Do not edit this file; changes will be overwritten.\n\n",
    );
    for head in heads {
        let source = fs::read_to_string(head).expect("read app head");
        app.push_str(&source.replace("/* APP_BODY */", indented_body.trim_end()));
        app.push('\n');
    }

    let generated_name = format!("generated_app_{chip}.rs");
    fs::write(out_dir.join(&generated_name), &app).expect("write generated app");
    println!("cargo:rustc-env=RTIC_GENERATED_APP={generated_name}");

    // Stable, predictable copy for humans and rust-analyzer to actually find.
    // This copy is only for generated-body / invalid-interrupt fixtures.
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let preview_dir = manifest_dir.join("target/generated-preview");
    fs::create_dir_all(&preview_dir).expect("create preview dir");
    fs::write(preview_dir.join(&generated_name), &app).expect("write preview copy");
}
