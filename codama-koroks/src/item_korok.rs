use crate::{EnumKorok, FileModuleKorok, ModuleKorok, StructKorok, UnsupportedItemKorok};
use codama_errors::CodamaResult;
use codama_nodes::Node;
use codama_stores::FileModuleStore;
use std::ops::AddAssign;

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
        file_modules: &'a Vec<FileModuleStore>,
        file_module_index: &mut usize,
    ) -> CodamaResult<Self> {
        match item {
            syn::Item::Mod(ast) if ast.content.is_none() => {
                match file_modules.iter().nth(*file_module_index) {
                    Some(module) => {
                        file_module_index.add_assign(1);
                        Ok(ItemKorok::FileModule(FileModuleKorok::parse(ast, module)?))
                    }
                    None => {
                        Err(syn::Error::new_spanned(ast, "Associated ModuleStore not found").into())
                    }
                }
            }
            syn::Item::Mod(ast) if ast.content.is_some() => Ok(ItemKorok::Module(
                ModuleKorok::parse(ast, file_modules, file_module_index)?,
            )),
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
        file_modules: &'a Vec<FileModuleStore>,
        file_module_index: &mut usize,
    ) -> CodamaResult<Vec<Self>> {
        items
            .iter()
            .map(|item| Self::parse(item, file_modules, file_module_index))
            .collect()
    }

    pub fn node(&self) -> Option<Node> {
        match self {
            ItemKorok::Struct(k) => k.node.clone(),
            ItemKorok::Enum(k) => k.node.clone(),
            ItemKorok::FileModule(k) => k.node.clone(),
            ItemKorok::Module(k) => k.node.clone(),
            ItemKorok::Unsupported(k) => k.node.clone(),
        }
    }

    pub fn set_node(&mut self, node: Option<Node>) {
        match self {
            ItemKorok::Struct(k) => k.node = node,
            ItemKorok::Enum(k) => k.node = node,
            ItemKorok::FileModule(k) => k.node = node,
            ItemKorok::Module(k) => k.node = node,
            ItemKorok::Unsupported(k) => k.node = node,
        }
    }
}