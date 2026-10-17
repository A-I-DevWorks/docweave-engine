//! Tipos de domínio usados pelo catálogo.
//!
//! Este módulo serve como exemplo de documentação de módulo e de declarações
//! com campos, variantes e métodos.

use std::fmt::Display;

/// Identificador estável de um item.
///
/// O valor é mantido como texto para que exemplos possam usar IDs legíveis:
/// `"book-rust-101"`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ItemId(String);

impl ItemId {
    /// Cria um identificador a partir de qualquer valor conversível em `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// let id = ItemId::new("guide-rust");
    /// assert_eq!(id.as_str(), "guide-rust");
    /// ```
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Retorna o identificador como uma fatia de texto.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ItemId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Situação editorial de um item do catálogo.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ItemStatus {
    // Ainda está sendo preparado e não deve aparecer em listas públicas.
    Draft,
    // Está disponível para consulta.
    Published,
    // Foi arquivado, mas permanece no histórico.
    Archived,
}

/// Um item do catálogo com conteúdo definido pelo consumidor da biblioteca.
///
/// `T` pode representar uma página, um registro ou qualquer outro payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CatalogItem<T> {
    id: ItemId,
    title: String,
    payload: T,
    status: ItemStatus,
}

impl<T> CatalogItem<T> {
    /// Cria um item com status inicial `Draft`.
    pub fn new(id: ItemId, title: impl Into<String>, payload: T) -> Self {
        Self {
            id,
            title: title.into(),
            payload,
            status: ItemStatus::Draft,
        }
    }

    /// Devolve o identificador do item.
    pub fn id(&self) -> &ItemId {
        &self.id
    }

    /// Devolve o título do item.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Devolve o payload sem transferir sua propriedade.
    pub fn payload(&self) -> &T {
        &self.payload
    }

    /// Devolve o status editorial atual.
    pub fn status(&self) -> &ItemStatus {
        &self.status
    }

    /// Publica o item para que ele possa ser exibido em resultados.
    pub fn publish(&mut self) {
        self.status = ItemStatus::Published;
    }

    /// Arquiva o item, mantendo seus dados disponíveis para consulta interna.
    pub fn archive(&mut self) {
        self.status = ItemStatus::Archived;
    }
}

/// Fornece um resumo legível de um item.
pub trait Summary {
    /// Monta uma linha de resumo para listas e resultados de busca.
    fn summary(&self) -> String;
}

impl<T: Display> Summary for CatalogItem<T> {
    fn summary(&self) -> String {
        format!("{} — {} ({:?})", self.title, self.payload, self.status)
    }
}

/// Tamanho máximo recomendado para títulos exibidos em uma linha.
pub const MAX_TITLE_LENGTH: usize = 120;
