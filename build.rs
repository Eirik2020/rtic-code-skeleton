use std::collections::BTreeSet;
use std::{env, fs, path::PathBuf};

use catalog::{Chip, Pin};
use quote::{format_ident, quote};
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

fn selected_system(chip: Chip) -> Option<catalog::SystemConfig> {
    let selected: Vec<_> = catalog::all_systems()
        .into_iter()
        .filter(|system| CfgReducer::feature_enabled(system.cargo_feature))
        .collect();
    assert!(selected.len() <= 1, "select at most one system feature");
    selected.into_iter().next().map(|system| {
        assert_eq!(system.chip(), chip, "system/chip mismatch");
        system
    })
}

/// A system selects an exact linker/probe record from the chip catalog.
/// Those records are build metadata, not peripheral capability data.
fn memory_layout(chip: Chip, system: Option<&catalog::SystemConfig>) -> String {
    let part = if let Some(system) = system {
        assert_eq!(system.chip(), chip, "system/chip mismatch");
        // Generated-body is a synthetic fixture, not the selected system firmware.
        assert!(
            !CfgReducer::feature_enabled("generated-body"),
            "system features cannot be combined with the generated-body fixture"
        );
        system.part
    } else {
        chip.config().default_part
    };
    if let Some(system) = system {
        let embed = read_config("Embed.toml");
        let profile = system.embed_profile;
        let general = embed
            .get(profile)
            .and_then(|profile| profile.get("general"))
            .unwrap_or_else(|| panic!("missing Embed.toml profile '{profile}'"));
        assert_eq!(
            config_str(general, "chip"),
            part.probe_chip,
            "system probe profile does not match the selected MCU part"
        );
    }
    let mut memory = format!(
        "MEMORY\n{{\n  FLASH : ORIGIN = 0x08000000, LENGTH = {}K\n  RAM : ORIGIN = 0x20000000, LENGTH = {}K\n",
        part.flash_kib, part.ram_kib
    );
    if let Some(ccm_kib) = part.ccm_kib {
        memory.push_str(&format!("  CCM : ORIGIN = 0x10000000, LENGTH = {ccm_kib}K\n"));
    }
    memory.push_str("}\n");
    memory
}

/// Turns the active system's clock selection into a real `rcc::Config`,
/// validated against the chip's maximum sysclk. Board-less systems and
/// chip-only builds have no clock selection and keep today's bare
/// `Config::hsi()` unchanged.
/// A system with a board is assumed to run from that board's own clock setup
/// (`crate::boards::active::clock_config()`, hand-written — this project's
/// domain is flight controllers, which always have HSE; there's no boardless
/// real hardware to model HSI-vs-HSE selection for). A system with no board
/// gets bare `Config::hsi()`.
fn clock_config(chip: Chip, system: Option<&catalog::SystemConfig>) -> String {
    let max_sysclk_hz = chip.config().max_sysclk_hz;

    let body = match system.and_then(|system| system.clock) {
        None => quote! { ::stm32f4xx_hal::rcc::Config::hsi() },
        Some(clock) => {
            let target_hz = clock.target_hz;
            assert!(
                target_hz <= max_sysclk_hz,
                "clock.target_hz {target_hz} exceeds chip '{chip}' max sysclk {max_sysclk_hz}"
            );
            let base = if system.is_some_and(|system| system.board.is_some()) {
                quote! { crate::boards::active::clock_config() }
            } else {
                quote! { ::stm32f4xx_hal::rcc::Config::hsi() }
            };
            quote! { #base.sysclk(::fugit::HertzU32::from_raw(#target_hz)) }
        }
    };

    let generated = quote! {
        // Generated from the active system's clock selection (or the chip
        // default when none is declared). Do not edit.
        pub fn config() -> ::stm32f4xx_hal::rcc::Config {
            #body
        }
    };
    let file = parse_file(&generated.to_string()).expect("parse generated clock module");
    prettyplease::unparse(&file)
}

fn cfg_values<'a>(values: impl IntoIterator<Item = &'a str>) -> String {
    values
        .into_iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Emits `board_peripheral`/`board_pin`/`board_dma` cfg flags from the active
/// system's board declaration. Peripheral construction itself lives in
/// `src/hardware/` and `src/boards/`, compiled for the embedded target — this
/// only needs to know *which* names to gate on, not how to build anything.
fn configure_board(system: Option<&catalog::SystemConfig>) {
    let mut peripheral_cfgs = BTreeSet::from([
        "usart1", "usart2", "usart6", "tim3", "tim4", "tim5",
    ]);
    let mut pin_cfgs = BTreeSet::from([
        "led1", "led2", "led3", "serial1", "serial2", "serial3", "pwm1", "pwm2",
    ]);
    let mut dma_cfgs = BTreeSet::new();

    if let Some(board) = system.and_then(|system| system.board.as_ref()) {
        for peripheral in &board.peripherals {
            let name = peripheral.cfg_name();
            peripheral_cfgs.insert(name);
            println!("cargo:rustc-cfg=board_peripheral=\"{name}\"");
        }
        for pin in &board.pins {
            let name = pin.cfg_name();
            pin_cfgs.insert(name);
            println!("cargo:rustc-cfg=board_pin=\"{name}\"");
        }
        for dma in &board.dma {
            let name = dma.cfg_name();
            dma_cfgs.insert(name);
            println!("cargo:rustc-cfg=board_dma=\"{name}\"");
        }
    }

    println!(
        "cargo:rustc-check-cfg=cfg(board_peripheral, values({}))",
        cfg_values(peripheral_cfgs)
    );
    println!(
        "cargo:rustc-check-cfg=cfg(board_pin, values({}))",
        cfg_values(pin_cfgs)
    );
    if !dma_cfgs.is_empty() {
        println!(
            "cargo:rustc-check-cfg=cfg(board_dma, values({}))",
            cfg_values(dma_cfgs)
        );
    }

    if CfgReducer::feature_enabled("sw-report") {
        let system = system
            .unwrap_or_else(|| panic!("feature 'sw-report' requires the Nucleo board prototype"));
        assert_eq!(
            system.cargo_feature, "system-nucleo-f401re",
            "feature 'sw-report' currently supports only system-nucleo-f401re"
        );
        assert!(
            system
                .board
                .as_ref()
                .is_some_and(|board| board.has_peripheral("tim3")),
            "feature 'sw-report' requires board peripheral 'tim3'"
        );
    }
}

/// Generates one function per named serial port (`serial1_protocol()`, ...),
/// returning the active system's default protocol route for it, or `None`
/// if unrouted. Validates each route names a pin the active board actually
/// declares, and that no pin is routed twice — the same "reject an
/// impossible mapping" requirement as
/// `docs/architecture/hardware-software-routing.md`, applied at build time
/// since there is no persistent/runtime routing store yet.
fn routing_config(system: Option<&catalog::SystemConfig>) -> String {
    let ports = [Pin::Serial1, Pin::Serial2, Pin::Serial3];
    let routes = system.map(|system| system.default_serial_routes).unwrap_or(&[]);

    let mut seen = BTreeSet::new();
    for route in routes {
        assert!(
            seen.insert(route.pin.cfg_name()),
            "default_serial_routes assigns more than one protocol to '{}'",
            route.pin.cfg_name()
        );
        let board = system
            .and_then(|system| system.board.as_ref())
            .unwrap_or_else(|| panic!("default_serial_routes requires an active board"));
        assert!(
            board.has_pin(route.pin.cfg_name()),
            "default_serial_routes assigns a protocol to '{}', which board '{}' does not provide",
            route.pin.cfg_name(),
            board.name
        );
    }

    let functions = ports.iter().map(|pin| {
        let fn_name = format_ident!("{}_protocol", pin.cfg_name());
        match routes.iter().find(|route| route.pin == *pin) {
            Some(route) => {
                let variant = format_ident!("{}", route.protocol.variant_name());
                quote! {
                    pub fn #fn_name() -> Option<crate::software::serial_protocol::SerialProtocol> {
                        Some(crate::software::serial_protocol::SerialProtocol::#variant)
                    }
                }
            }
            None => quote! {
                pub fn #fn_name() -> Option<crate::software::serial_protocol::SerialProtocol> {
                    None
                }
            },
        }
    });

    let generated = quote! {
        // Generated from the active system's default_serial_routes. Do not edit.
        #(#functions)*
    };
    let file = parse_file(&generated.to_string()).expect("parse generated routing module");
    prettyplease::unparse(&file)
}

fn main() {
    println!("cargo:rerun-if-changed=src/app_body_skeleton.rs");
    for chip in Chip::ALL {
        println!("cargo:rerun-if-changed=src/chips/{chip}/app_head.rs");
        println!("cargo:rerun-if-changed=src/chips/{chip}/app_body.rs");
    }
    println!("cargo:rerun-if-changed=src/invalid_interrupt.rs");
    for feature in [
        "CARGO_FEATURE_F401",
        "CARGO_FEATURE_F405",
        "CARGO_FEATURE_F411",
        "CARGO_FEATURE_SYSTEM_NUCLEO_F401RE",
        "CARGO_FEATURE_SYSTEM_DEFAULT_F401",
        "CARGO_FEATURE_SYSTEM_DEFAULT_F405",
        "CARGO_FEATURE_SYSTEM_DEFAULT_F411",
        "CARGO_FEATURE_SW_REPORT",
        "CARGO_FEATURE_GENERATED_BODY",
        "CARGO_FEATURE_INVALID_INTERRUPT",
    ] {
        println!("cargo:rerun-if-env-changed={feature}");
    }

    let chips: Vec<Chip> = Chip::ALL
        .into_iter()
        .filter(|chip| {
            let env_name = format!("CARGO_FEATURE_{}", chip.cargo_feature().to_uppercase());
            env::var_os(env_name).is_some()
        })
        .collect();
    assert_eq!(chips.len(), 1, "exactly one chip feature must be selected");
    let chip = chips[0];
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let system = selected_system(chip);
    fs::write(
        out_dir.join("memory.x"),
        memory_layout(chip, system.as_ref()),
    )
    .expect("write memory.x");
    configure_board(system.as_ref());
    fs::write(
        out_dir.join("clock_config.rs"),
        clock_config(chip, system.as_ref()),
    )
    .expect("write clock_config.rs");
    fs::write(out_dir.join("routing.rs"), routing_config(system.as_ref()))
        .expect("write routing.rs");
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
        "src/app_body_skeleton.rs".to_string()
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
