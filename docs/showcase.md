# Showcase

## Contents

- [`catalog.rs`](#file-catalog-rs)
  - [Struct `ItemId`](#symbol-catalog-rs-struct-itemid)
  - [Method `new`](#symbol-catalog-rs-method-new)
  - [Method `as_str`](#symbol-catalog-rs-method-as-str)
  - [Method `fmt`](#symbol-catalog-rs-method-fmt)
  - [Enum `ItemStatus`](#symbol-catalog-rs-enum-itemstatus)
  - [Struct `CatalogItem`](#symbol-catalog-rs-struct-catalogitem)
  - [Method `new`](#symbol-catalog-rs-method-new-2)
  - [Method `id`](#symbol-catalog-rs-method-id)
  - [Method `title`](#symbol-catalog-rs-method-title)
  - [Method `payload`](#symbol-catalog-rs-method-payload)
  - [Method `status`](#symbol-catalog-rs-method-status)
  - [Method `publish`](#symbol-catalog-rs-method-publish)
  - [Method `archive`](#symbol-catalog-rs-method-archive)
  - [Trait `Summary`](#symbol-catalog-rs-trait-summary)
  - [Method `summary`](#symbol-catalog-rs-method-summary)
  - [Method `summary`](#symbol-catalog-rs-method-summary-2)
  - [Constant `MAX_TITLE_LENGTH`](#symbol-catalog-rs-constant-max-title-length)
- [`lib.rs`](#file-lib-rs)
  - [Module `catalog`](#symbol-lib-rs-module-catalog)
  - [Module `service`](#symbol-lib-rs-module-service)
  - [Module `markdown_examples`](#symbol-lib-rs-module-markdown-examples)
- [`markdown_examples.rs`](#file-markdown-examples-rs)
  - [Function `markdown_features_example`](#symbol-markdown-examples-rs-function-markdown-features-example)
- [`service.rs`](#file-service-rs)
  - [Struct `Catalog`](#symbol-service-rs-struct-catalog)
  - [Method `new`](#symbol-service-rs-method-new)
  - [Method `find`](#symbol-service-rs-method-find)
  - [Method `len`](#symbol-service-rs-method-len)
  - [Method `is_empty`](#symbol-service-rs-method-is-empty)
  - [Function `render_public_item`](#symbol-service-rs-function-render-public-item)

<a id="file-catalog-rs"></a>
## `catalog.rs`

Tipos de domínio usados pelo catálogo.

Este módulo serve como exemplo de documentação de módulo e de declarações
com campos, variantes e métodos.

<a id="symbol-catalog-rs-struct-itemid"></a>
### Struct: `ItemId`

```rust
pub struct ItemId(String);
```

Identificador estável de um item.

O valor é mantido como texto para que exemplos possam usar IDs legíveis:
`"book-rust-101"`.

[View source](../showcase/catalog.rs#L13)

<a id="symbol-catalog-rs-method-new"></a>
### Method: `new`

```rust
pub fn new(value: impl Into<String>) -> Self
```

Cria um identificador a partir de qualquer valor conversível em `String`.

# Examples

```
let id = ItemId::new("guide-rust");
assert_eq!(id.as_str(), "guide-rust");
```

[View source](../showcase/catalog.rs#L24)

<a id="symbol-catalog-rs-method-as-str"></a>
### Method: `as_str`

```rust
pub fn as_str(&self) -> &str
```

Retorna o identificador como uma fatia de texto.

[View source](../showcase/catalog.rs#L29)

<a id="symbol-catalog-rs-method-fmt"></a>
### Method: `fmt`

```rust
fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[View source](../showcase/catalog.rs#L35)

<a id="symbol-catalog-rs-enum-itemstatus"></a>
### Enum: `ItemStatus`

```rust
pub enum ItemStatus
```

Situação editorial de um item do catálogo.

[View source](../showcase/catalog.rs#L42)

<a id="symbol-catalog-rs-struct-catalogitem"></a>
### Struct: `CatalogItem`

```rust
pub struct CatalogItem<T>
```

Um item do catálogo com conteúdo definido pelo consumidor da biblioteca.

`T` pode representar uma página, um registro ou qualquer outro payload.

[View source](../showcase/catalog.rs#L55)

<a id="symbol-catalog-rs-method-new-2"></a>
### Method: `new`

```rust
pub fn new(id: ItemId, title: impl Into<String>, payload: T) -> Self
```

Cria um item com status inicial `Draft`.

[View source](../showcase/catalog.rs#L64)

<a id="symbol-catalog-rs-method-id"></a>
### Method: `id`

```rust
pub fn id(&self) -> &ItemId
```

Devolve o identificador do item.

[View source](../showcase/catalog.rs#L74)

<a id="symbol-catalog-rs-method-title"></a>
### Method: `title`

```rust
pub fn title(&self) -> &str
```

Devolve o título do item.

[View source](../showcase/catalog.rs#L79)

<a id="symbol-catalog-rs-method-payload"></a>
### Method: `payload`

```rust
pub fn payload(&self) -> &T
```

Devolve o payload sem transferir sua propriedade.

[View source](../showcase/catalog.rs#L84)

<a id="symbol-catalog-rs-method-status"></a>
### Method: `status`

```rust
pub fn status(&self) -> &ItemStatus
```

Devolve o status editorial atual.

[View source](../showcase/catalog.rs#L89)

<a id="symbol-catalog-rs-method-publish"></a>
### Method: `publish`

```rust
pub fn publish(&mut self)
```

Publica o item para que ele possa ser exibido em resultados.

[View source](../showcase/catalog.rs#L94)

<a id="symbol-catalog-rs-method-archive"></a>
### Method: `archive`

```rust
pub fn archive(&mut self)
```

Arquiva o item, mantendo seus dados disponíveis para consulta interna.

[View source](../showcase/catalog.rs#L99)

<a id="symbol-catalog-rs-trait-summary"></a>
### Trait: `Summary`

```rust
pub trait Summary
```

Fornece um resumo legível de um item.

[View source](../showcase/catalog.rs#L105)

<a id="symbol-catalog-rs-method-summary"></a>
### Method: `summary`

```rust
fn summary(&self) -> String;
```

Monta uma linha de resumo para listas e resultados de busca.

[View source](../showcase/catalog.rs#L107)

<a id="symbol-catalog-rs-method-summary-2"></a>
### Method: `summary`

```rust
fn summary(&self) -> String
```

[View source](../showcase/catalog.rs#L111)

<a id="symbol-catalog-rs-constant-max-title-length"></a>
### Constant: `MAX_TITLE_LENGTH`

```rust
pub const MAX_TITLE_LENGTH: usize = 120;
```

Tamanho máximo recomendado para títulos exibidos em uma linha.

[View source](../showcase/catalog.rs#L117)

<a id="file-lib-rs"></a>
## `lib.rs`

# DocWeave showcase

Um pequeno crate de exemplo para demonstrar a extração de comentários Rust.
Os módulos incluem comentários de arquivo (`//!`), documentação de itens
(`///`), Markdown, atributos, genéricos, traits e métodos.

Comece pelos [tipos do catálogo](crate::catalog) e depois veja o
[serviço de consulta](crate::service).

<a id="symbol-lib-rs-module-catalog"></a>
### Module: `catalog`

```rust
pub mod catalog;
```

Tipos que descrevem itens disponíveis no catálogo.

[View source](../showcase/lib.rs#L11)

<a id="symbol-lib-rs-module-service"></a>
### Module: `service`

```rust
pub mod service;
```

Operações de consulta e apresentação do catálogo.

[View source](../showcase/lib.rs#L14)

<a id="symbol-lib-rs-module-markdown-examples"></a>
### Module: `markdown_examples`

```rust
pub mod markdown_examples;
```

Exemplos de sintaxe Markdown em comentários de documentação.

[View source](../showcase/lib.rs#L17)

<a id="file-markdown-examples-rs"></a>
## `markdown_examples.rs`

Exemplos de construções Markdown preservadas nos comentários de documentação.

<a id="symbol-markdown-examples-rs-function-markdown-features-example"></a>
### Function: `markdown_features_example`

```rust
pub fn markdown_features_example() -> &'static str
```

Demonstra títulos, ênfase, listas, links, citações, tabelas e blocos de código.

## Formatação inline

Use **negrito**, *itálico*, ~~tachado~~ e `código` em uma frase. Links também
funcionam, como [a documentação do Rust](https://doc.rust-lang.org/).

> Uma citação pode destacar uma observação importante.
>
> Ela pode ocupar mais de uma linha.

## Listas

1. Primeiro, descreva a preparação.
2. Depois, execute os passos:
   - valide os dados;
   - gere a saída.

## Tabela

| Entrada | Resultado |
|:--|--:|
| válida | sucesso |
| inválida | erro |

## Bloco de código

```rust
let title = "Guia";
assert_eq!(title.len(), 4);
```

Abaixo há um separador horizontal:

---

[View source](../showcase/markdown_examples.rs#L38)

<a id="file-service-rs"></a>
## `service.rs`

Operações de consulta para o catálogo.

O serviço mantém uma coleção em memória para que os exemplos permaneçam
pequenos e não dependam de banco de dados ou rede.

<a id="symbol-service-rs-struct-catalog"></a>
### Struct: `Catalog`

```rust
pub struct Catalog<T>
```

Coleção consultável de itens com payload do tipo `T`.

O catálogo preserva a ordem recebida e permite localizar itens pelo ID.

[View source](../showcase/service.rs#L14)

<a id="symbol-service-rs-method-new"></a>
### Method: `new`

```rust
pub fn new(items: impl IntoIterator<Item = CatalogItem<T>>) -> Self
```

Cria um catálogo a partir de uma sequência de itens.

[View source](../showcase/service.rs#L20)

<a id="symbol-service-rs-method-find"></a>
### Method: `find`

```rust
pub fn find(&self, id: &ItemId) -> Option<&CatalogItem<T>>
```

Procura um item pelo seu identificador.

Retorna `None` quando o ID não está presente.

[View source](../showcase/service.rs#L29)

<a id="symbol-service-rs-method-len"></a>
### Method: `len`

```rust
pub fn len(&self) -> usize
```

Conta os itens armazenados, incluindo rascunhos e itens arquivados.

[View source](../showcase/service.rs#L34)

<a id="symbol-service-rs-method-is-empty"></a>
### Method: `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

Informa se o catálogo não contém itens.

[View source](../showcase/service.rs#L39)

<a id="symbol-service-rs-function-render-public-item"></a>
### Function: `render_public_item`

```rust
pub fn render_public_item<T: Display>(item: &CatalogItem<T>) -> Option<String>
```

Formata um item publicado para uma listagem simples.

Itens em rascunho ou arquivados não são exibidos:

```
let line = render_public_item(&item);
assert!(line.is_some());
```

[View source](../showcase/service.rs#L52)
