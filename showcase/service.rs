//! Operações de consulta para o catálogo.
//!
//! O serviço mantém uma coleção em memória para que os exemplos permaneçam
//! pequenos e não dependam de banco de dados ou rede.

use std::fmt::Display;

use crate::catalog::{CatalogItem, ItemId, ItemStatus, Summary};

/// Coleção consultável de itens com payload do tipo `T`.
///
/// O catálogo preserva a ordem recebida e permite localizar itens pelo ID.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Catalog<T> {
    items: Vec<CatalogItem<T>>,
}

impl<T> Catalog<T> {
    /// Cria um catálogo a partir de uma sequência de itens.
    pub fn new(items: impl IntoIterator<Item = CatalogItem<T>>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }

    /// Procura um item pelo seu identificador.
    ///
    /// Retorna `None` quando o ID não está presente.
    pub fn find(&self, id: &ItemId) -> Option<&CatalogItem<T>> {
        self.items.iter().find(|item| item.id() == id)
    }

    /// Conta os itens armazenados, incluindo rascunhos e itens arquivados.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Informa se o catálogo não contém itens.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Formata um item publicado para uma listagem simples.
///
/// Itens em rascunho ou arquivados não são exibidos:
///
/// ```
/// let line = render_public_item(&item);
/// assert!(line.is_some());
/// ```
pub fn render_public_item<T: Display>(item: &CatalogItem<T>) -> Option<String> {
    if item.status() == &ItemStatus::Published {
        Some(item.summary())
    } else {
        None
    }
}
