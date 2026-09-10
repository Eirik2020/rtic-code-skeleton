use std::{env, fs, path::PathBuf};

use quote::ToTokens;
use syn::visit_mut::VisitMut;
use syn::{parse_file, visit_mut, Attribute, Field, Item, Lit, Meta};

struct CfgReducer;

impl CfgReducer {
    fn feature_enabled(name: &str) -> bool {
        env::var_os(format!("CARGO_FEATURE_{}", name.to_uppercase())).is_some()
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
    file.into_token_stream().to_string()
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
    let chip = chips
        .first()
        .copied()
        .unwrap_or_else(|| panic!("one chip feature must be selected"));
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

    let mut app = String::from(
        "// WARNING: Generated code. Do not edit this file; changes will be overwritten.\n\n",
    );
    for head in heads {
        let source = fs::read_to_string(head).expect("read app head");
        app.push_str(&source.replace("/* APP_BODY */", &body));
        app.push('\n');
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let generated_name = format!("generated_app_{chip}.rs");
    fs::write(out_dir.join(&generated_name), app).expect("write generated app");
    println!("cargo:rustc-env=RTIC_GENERATED_APP={generated_name}");
}
