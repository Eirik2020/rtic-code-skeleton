//! Chip cfg reduction before RTIC expansion, preserving the input token spans.
//!
//! The selected chip is an explicit argument, never the proc-macro host's
//! environment. This keeps rustc and rust-analyzer expansion deterministic.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, Item, Meta, Stmt, Token, punctuated::Punctuated, visit_mut::VisitMut};

const CHIPS: [&str; 3] = ["f401", "f405", "f411"];
#[path = "../../../src/systems/catalog.rs"]
mod systems_catalog;
use systems_catalog::SYSTEMS;

struct Selection {
    chip: syn::Ident,
    system: Option<syn::LitStr>,
}

impl syn::parse::Parse for Selection {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let chip = input.parse()?;
        let system = if input.is_empty() {
            None
        } else {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        };
        Ok(Self { chip, system })
    }
}

struct Reducer {
    chip: String,
    system: Option<String>,
    error: Option<syn::Error>,
}

impl Reducer {
    fn predicate(&self, meta: &Meta) -> syn::Result<bool> {
        match meta {
            Meta::NameValue(value) if value.path.is_ident("feature") => {
                if let Expr::Lit(expr) = &value.value {
                    if let syn::Lit::Str(feature) = &expr.lit {
                        let name = feature.value();
                        if CHIPS.contains(&name.as_str()) {
                            return Ok(name == self.chip);
                        }
                        if SYSTEMS.iter().any(|(system, _)| *system == name) {
                            return Ok(self.system.as_deref() == Some(name.as_str()));
                        }
                    }
                }
            }
            Meta::List(list) => {
                let name = &list.path;
                if name.is_ident("all") || name.is_ident("any") || name.is_ident("not") {
                    let args =
                        list.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                    // Validate every operand, including ones short-circuiting would skip.
                    let values = args
                        .iter()
                        .map(|meta| self.predicate(meta))
                        .collect::<syn::Result<Vec<_>>>()?;
                    if name.is_ident("all") {
                        return Ok(values.iter().all(|value| *value));
                    }
                    if name.is_ident("any") {
                        return Ok(values.iter().any(|value| *value));
                    }
                    if values.len() == 1 {
                        return Ok(!values[0]);
                    }
                    return Err(syn::Error::new_spanned(
                        meta,
                        "not() requires exactly one predicate",
                    ));
                }
            }
            _ => {}
        }
        Err(syn::Error::new_spanned(
            meta,
            "RTIC preprocessing supports registered chip/system features and all/any/not; register new features in the reducer before using them here",
        ))
    }

    fn record(&mut self, error: syn::Error) {
        if let Some(existing) = &mut self.error {
            existing.combine(error);
        } else {
            self.error = Some(error);
        }
    }

    fn attributes(&mut self, attrs: &mut Vec<Attribute>) -> bool {
        let mut enabled = true;
        attrs.retain(|attr| {
            if attr.path().is_ident("cfg") {
                match attr.parse_args::<Meta>().and_then(|meta| self.predicate(&meta)) {
                    Ok(value) => enabled &= value,
                    Err(error) => self.record(error),
                }
                // RTIC must not copy even a true cfg onto generated expressions.
                false
            } else {
                if attr.path().is_ident("cfg_attr") {
                    self.record(syn::Error::new_spanned(attr,
                        "cfg_attr inside the RTIC body is not supported by this reducer; use cfg-gated items instead"));
                }
                true
            }
        });
        enabled
    }

    fn item_enabled(&mut self, item: &mut Item) -> bool {
        let attrs = match item {
            Item::Const(item) => &mut item.attrs,
            Item::Enum(item) => &mut item.attrs,
            Item::ExternCrate(item) => &mut item.attrs,
            Item::Fn(item) => &mut item.attrs,
            Item::ForeignMod(item) => &mut item.attrs,
            Item::Impl(item) => &mut item.attrs,
            Item::Macro(item) => &mut item.attrs,
            Item::Mod(item) => &mut item.attrs,
            Item::Static(item) => &mut item.attrs,
            Item::Struct(item) => &mut item.attrs,
            Item::Trait(item) => &mut item.attrs,
            Item::TraitAlias(item) => &mut item.attrs,
            Item::Type(item) => &mut item.attrs,
            Item::Union(item) => &mut item.attrs,
            Item::Use(item) => &mut item.attrs,
            _ => return true,
        };
        self.attributes(attrs)
    }

    fn items(&mut self, items: &mut Vec<Item>) {
        items.retain_mut(|item| self.item_enabled(item));
        for item in items {
            self.visit_item_mut(item);
        }
    }
}

impl VisitMut for Reducer {
    fn visit_item_mod_mut(&mut self, module: &mut syn::ItemMod) {
        if let Some((_, items)) = &mut module.content {
            self.items(items);
        }
    }

    fn visit_fields_mut(&mut self, fields: &mut syn::Fields) {
        let fields = match fields {
            syn::Fields::Named(fields) => &mut fields.named,
            syn::Fields::Unnamed(fields) => &mut fields.unnamed,
            syn::Fields::Unit => return,
        };
        *fields = std::mem::take(fields)
            .into_iter()
            .filter_map(|mut field| self.attributes(&mut field.attrs).then_some(field))
            .collect();
        for field in fields {
            self.visit_field_mut(field);
        }
    }

    fn visit_expr_struct_mut(&mut self, expr: &mut syn::ExprStruct) {
        expr.fields = std::mem::take(&mut expr.fields)
            .into_iter()
            .filter_map(|mut field| self.attributes(&mut field.attrs).then_some(field))
            .collect();
        syn::visit_mut::visit_expr_struct_mut(self, expr);
    }

    fn visit_block_mut(&mut self, block: &mut syn::Block) {
        block.stmts.retain_mut(|stmt| match stmt {
            Stmt::Local(local) => self.attributes(&mut local.attrs),
            Stmt::Item(item) => self.item_enabled(item),
            Stmt::Macro(stmt) => self.attributes(&mut stmt.attrs),
            Stmt::Expr(_, _) => true, // The visitor below diagnoses unsupported cfg positions.
        });
        syn::visit_mut::visit_block_mut(self, block);
    }

    fn visit_attribute_mut(&mut self, attr: &mut Attribute) {
        // Fail closed for conditional syntax not handled above, rather than
        // silently producing a different RTIC resource/task graph.
        if attr.path().is_ident("cfg") || attr.path().is_ident("cfg_attr") {
            self.record(syn::Error::new_spanned(attr,
                "cfg at this position is not supported by the RTIC reducer; use a cfg-gated item or let statement"));
        }
    }
}

fn reduce(chip: &str, system: Option<&str>, module: &mut syn::ItemMod) -> syn::Result<()> {
    if !CHIPS.contains(&chip) {
        return Err(syn::Error::new_spanned(
            &module.ident,
            "expected f401, f405, or f411",
        ));
    }
    if let Some(system) = system {
        if !SYSTEMS.contains(&(system, chip)) {
            return Err(syn::Error::new_spanned(
                &module.ident,
                "unknown system or system/chip mismatch",
            ));
        }
    }
    let Some((_, items)) = &mut module.content else {
        return Err(syn::Error::new_spanned(
            module,
            "put for_chip on an inline mod app { ... }, inside the source file",
        ));
    };
    let mut reducer = Reducer {
        chip: chip.into(),
        system: system.map(str::to_owned),
        error: None,
    };
    reducer.items(items);
    reducer.error.map_or(Ok(()), Err)
}

/// Place before RTIC, with an optional explicit system: `for_chip(f401, "system-nucleo-f401re")`.
#[proc_macro_attribute]
pub fn for_chip(args: TokenStream, input: TokenStream) -> TokenStream {
    let selection = syn::parse_macro_input!(args as Selection);
    let system = selection.system.as_ref().map(syn::LitStr::value);
    let mut module = syn::parse_macro_input!(input as syn::ItemMod);
    match reduce(&selection.chip.to_string(), system.as_deref(), &mut module) {
        Ok(()) => quote!(#module).into(),
        Err(error) => error.to_compile_error().into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduces_real_skeleton_for_every_chip() {
        let source = include_str!("../../../src/app_body_skeleton.rs");
        for chip in CHIPS {
            let mut module = syn::parse_str::<syn::ItemMod>(source).unwrap();
            reduce(chip, None, &mut module).unwrap();
            let tokens = quote!(#module).to_string();
            let items = &module.content.unwrap().1;
            let inits = items
                .iter()
                .filter(|item| matches!(item, Item::Fn(f) if f.sig.ident == "init"))
                .count();
            assert_eq!(inits, 1, "{chip}");
            assert_eq!(tokens.contains("f405_shared"), chip == "f405");
            assert_eq!(tokens.contains("f405_whole_task"), chip == "f405");
            assert_eq!(tokens.contains("f411_whole_control"), chip == "f411");
            assert!(
                !tokens.contains("fn blink"),
                "chip-only builds must not include system tasks"
            );
            assert!(!tokens.contains("Hardware"));
        }
    }

    #[test]
    fn rejects_unknown_predicates_instead_of_silently_dropping_code() {
        let mut module = syn::parse_quote! {
            mod app {
                #[cfg(any(feature = "f401", target_os = "none"))]
                fn task() {}
            }
        };
        assert!(reduce("f401", None, &mut module).is_err());
    }

    #[test]
    fn reduces_init_statements_and_removes_true_field_cfg() {
        let mut module = syn::parse_quote! {
            mod app {
                struct Shared { #[cfg(feature = "f405")] value: u32 }
                fn init() {
                    #[cfg(feature = "f405")] let value = 1;
                    #[cfg(feature = "f411")] let unavailable = absent();
                    Shared { #[cfg(feature = "f405")] value };
                }
            }
        };
        reduce("f405", None, &mut module).unwrap();
        let tokens = quote!(#module).to_string();
        assert!(!tokens.contains("cfg"));
        assert!(!tokens.contains("absent"));
        assert!(tokens.contains("let value"));
    }

    #[test]
    fn nucleo_selects_system_tasks_and_exactly_one_init() {
        let mut module =
            syn::parse_str::<syn::ItemMod>(include_str!("../../../src/app_body_skeleton.rs"))
                .unwrap();
        reduce("f401", Some("system-nucleo-f401re"), &mut module).unwrap();
        let tokens = quote!(#module).to_string();
        assert!(tokens.contains("fn blink"));
        assert!(tokens.contains("Hardware"));
        assert!(!tokens.contains("UART4"));
        assert_eq!(tokens.matches("fn init").count(), 1);
    }

    #[test]
    fn rejects_incompatible_or_unknown_system() {
        let mut module = syn::parse_quote! { mod app {} };
        assert!(reduce("f405", Some("system-nucleo-f401re"), &mut module).is_err());
        assert!(reduce("f401", Some("system-unknown"), &mut module).is_err());
    }

    #[test]
    fn default_systems_keep_chip_tasks_without_nucleo_wiring() {
        for &(system, chip) in SYSTEMS
            .iter()
            .filter(|(system, _)| system.starts_with("system-default-"))
        {
            let mut module =
                syn::parse_str::<syn::ItemMod>(include_str!("../../../src/app_body_skeleton.rs"))
                    .unwrap();
            reduce(chip, Some(system), &mut module).unwrap();
            let tokens = quote!(#module).to_string();
            assert_eq!(tokens.matches("fn init").count(), 1, "{system}");
            assert!(!tokens.contains("Hardware"), "{system}");
            assert!(!tokens.contains("fn blink"), "{system}");
            assert_eq!(tokens.contains("UART4"), chip == "f405", "{system}");
        }
    }
}
