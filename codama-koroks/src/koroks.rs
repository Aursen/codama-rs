use cargo_toml::Manifest;
use codama_nodes::Node;
use std::path::Path;

use crate::attributes::Attribute;
use crate::internals::ParsingResult;
use crate::korok_visitor::KorokVisitor;
use crate::stores::{CrateStore, ModuleStore, RootStore};

pub trait Korok<'a> {
    fn accept(&self, visitor: &mut dyn KorokVisitor);
}

#[derive(Debug)]
pub struct RootKorok<'a> {
    pub crates: Vec<CrateKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> RootKorok<'a> {
    pub fn parse(root_store: &'a RootStore) -> ParsingResult<Self> {
        Ok(Self {
            crates: root_store
                .crates
                .iter()
                .map(CrateKorok::parse)
                .collect::<ParsingResult<_>>()?,
            node: None,
        })
    }
}

impl Korok<'_> for RootKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_root(self);
    }
}

#[derive(Debug)]
pub struct CrateKorok<'a> {
    pub file: &'a syn::File,
    pub items: Vec<ItemKorok<'a>>,
    pub manifest: &'a Option<Manifest>,
    pub node: Option<Node>,
    pub path: &'a Path,
}

impl<'a> CrateKorok<'a> {
    pub fn parse(crate_store: &'a CrateStore) -> ParsingResult<Self> {
        Ok(Self {
            file: &crate_store.file,
            items: ItemKorok::parse_all(&crate_store.file.items, &crate_store.modules)?,
            manifest: &crate_store.manifest,
            node: None,
            path: &crate_store.path,
        })
    }
}

impl Korok<'_> for CrateKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_crate(self);
    }
}

#[derive(Debug)]
pub enum ItemKorok<'a> {
    FileModule(FileModuleKorok<'a>),
    Module(ModuleKorok<'a>),
    Struct(StructKorok<'a>),
    Enum(EnumKorok<'a>),
    Unsupported(UnsupportedItemKorok<'a>),
}

impl<'a> ItemKorok<'a> {
    pub fn parse(
        item: &'a syn::Item,
        modules: &'a Vec<ModuleStore>,
        item_index: usize,
    ) -> ParsingResult<Self> {
        match item {
            syn::Item::Mod(ast) if ast.content.is_none() => {
                let module = modules.iter().nth(item_index);
                match module {
                    Some(module) => Ok(ItemKorok::FileModule(FileModuleKorok::parse(ast, module)?)),
                    None => {
                        Err(syn::Error::new_spanned(ast, "Associated ModuleStore not found").into())
                    }
                }
            }
            syn::Item::Mod(ast) if ast.content.is_some() => {
                Ok(ItemKorok::Module(ModuleKorok::parse(ast, modules)?))
            }
            syn::Item::Struct(ast) => Ok(ItemKorok::Struct(StructKorok::parse(ast)?)),
            syn::Item::Enum(ast) => Ok(ItemKorok::Enum(EnumKorok::parse(ast)?)),
            _ => Ok(ItemKorok::Unsupported(UnsupportedItemKorok {
                ast: item,
                node: None,
            })),
        }
    }

    pub fn parse_all(
        items: &'a Vec<syn::Item>,
        modules: &'a Vec<ModuleStore>,
    ) -> ParsingResult<Vec<Self>> {
        items
            .iter()
            .enumerate()
            .map(|(item_index, item)| Self::parse(item, modules, item_index))
            .collect()
    }
}

impl Korok<'_> for ItemKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_item(self);
    }
}

#[derive(Debug)]
pub struct FileModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub file: &'a syn::File,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
    pub path: &'a Path,
}

impl<'a> FileModuleKorok<'a> {
    pub fn parse(ast: &'a syn::ItemMod, module: &'a ModuleStore) -> ParsingResult<Self> {
        if let Some(_) = ast.content {
            return Err(syn::Error::new_spanned(
                ast,
                "Module has content, it should be parsed with ModuleKorok",
            )
            .into());
        }

        Ok(Self {
            ast,
            file: &module.file,
            items: ItemKorok::parse_all(&module.file.items, &module.modules)?,
            path: &module.path,
            node: None,
        })
    }
}

impl Korok<'_> for FileModuleKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_file_module(self);
    }
}

#[derive(Debug)]
pub struct ModuleKorok<'a> {
    pub ast: &'a syn::ItemMod,
    pub items: Vec<ItemKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> ModuleKorok<'a> {
    pub fn parse(ast: &'a syn::ItemMod, modules: &'a Vec<ModuleStore>) -> ParsingResult<Self> {
        match &ast.content {
            Some(content) => Ok(Self {
                ast,
                items: ItemKorok::parse_all(&content.1, modules)?,
                node: None,
            }),
            None => Err(syn::Error::new_spanned(
                ast,
                "Module has no content, it should be parsed with FileModuleKorok",
            )
            .into()),
        }
    }
}

impl Korok<'_> for ModuleKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_module(self);
    }
}

#[derive(Debug)]
pub struct StructKorok<'a> {
    pub ast: &'a syn::ItemStruct,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: Vec<FieldKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> StructKorok<'a> {
    pub fn parse(ast: &'a syn::ItemStruct) -> ParsingResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            fields: FieldKorok::parse_all(&ast.fields)?,
            node: None,
        })
    }
}

impl Korok<'_> for StructKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_struct(self);
    }
}

#[derive(Debug)]
pub struct FieldKorok<'a> {
    pub ast: &'a syn::Field,
    pub attributes: Vec<Attribute<'a>>,
    pub node: Option<Node>,
}

impl<'a> FieldKorok<'a> {
    pub fn parse_all(fields: &'a syn::Fields) -> ParsingResult<Vec<Self>> {
        match fields {
            syn::Fields::Named(f) => f.named.iter().map(Self::parse).collect(),
            syn::Fields::Unnamed(f) => f.unnamed.iter().map(Self::parse).collect(),
            syn::Fields::Unit => Ok(vec![]),
        }
    }

    pub fn parse(ast: &'a syn::Field) -> ParsingResult<Self> {
        let attributes = Attribute::parse_all(&ast.attrs)?;
        Ok(Self {
            ast,
            attributes,
            node: None,
        })
    }
}

impl Korok<'_> for FieldKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_field(self);
    }
}

#[derive(Debug)]
pub struct EnumKorok<'a> {
    pub ast: &'a syn::ItemEnum,
    pub attributes: Vec<Attribute<'a>>,
    pub node: Option<Node>,
    pub variants: Vec<EnumVariantKorok<'a>>,
}

impl<'a> EnumKorok<'a> {
    pub fn parse(ast: &'a syn::ItemEnum) -> ParsingResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            node: None,
            variants: EnumVariantKorok::parse_all(&ast.variants)?,
        })
    }
}

impl Korok<'_> for EnumKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_enum(self);
    }
}

#[derive(Debug)]
pub struct EnumVariantKorok<'a> {
    pub ast: &'a syn::Variant,
    pub attributes: Vec<Attribute<'a>>,
    pub fields: Vec<FieldKorok<'a>>,
    pub node: Option<Node>,
}

impl<'a> EnumVariantKorok<'a> {
    pub fn parse(ast: &'a syn::Variant) -> ParsingResult<Self> {
        Ok(Self {
            ast,
            attributes: Attribute::parse_all(&ast.attrs)?,
            fields: FieldKorok::parse_all(&ast.fields)?,
            node: None,
        })
    }

    pub fn parse_all(
        variants: &'a syn::punctuated::Punctuated<syn::Variant, syn::Token![,]>,
    ) -> ParsingResult<Vec<Self>> {
        variants.iter().map(Self::parse).collect()
    }
}

impl Korok<'_> for EnumVariantKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_enum_variant(self);
    }
}

#[derive(Debug)]
pub struct UnsupportedItemKorok<'a> {
    pub ast: &'a syn::Item,
    pub node: Option<Node>,
}

impl Korok<'_> for UnsupportedItemKorok<'_> {
    fn accept(&self, visitor: &mut dyn KorokVisitor) {
        visitor.visit_unsupported_item(self);
    }
}
