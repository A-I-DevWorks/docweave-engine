//! # DocWeave showcase
//!
//! Um pequeno crate de exemplo para demonstrar a extração de comentários Rust.
//! Os módulos incluem comentários de arquivo (`//!`), documentação de itens
//! (`///`), Markdown, atributos, genéricos, traits e métodos.
//!
//! Comece pelos [tipos do catálogo](crate::catalog) e depois veja o
//! [serviço de consulta](crate::service).

/// Tipos que descrevem itens disponíveis no catálogo.
pub mod catalog;

/// Operações de consulta e apresentação do catálogo.
pub mod service;

/// Exemplos de sintaxe Markdown em comentários de documentação.
pub mod markdown_examples;
