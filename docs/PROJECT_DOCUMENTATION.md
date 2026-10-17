# DocWeave API

## Contents

- [`showcase/c_api.c`](#file-showcase-c-api-c)
  - [Function `multiply_values`](#symbol-showcase-c-api-c-function-multiply-values)
- [`showcase/catalog.rs`](#file-showcase-catalog-rs)
  - [Struct `ItemId`](#symbol-showcase-catalog-rs-struct-itemid)
  - [Method `new`](#symbol-showcase-catalog-rs-method-new)
  - [Method `as_str`](#symbol-showcase-catalog-rs-method-as-str)
  - [Method `fmt`](#symbol-showcase-catalog-rs-method-fmt)
  - [Enum `ItemStatus`](#symbol-showcase-catalog-rs-enum-itemstatus)
  - [Struct `CatalogItem`](#symbol-showcase-catalog-rs-struct-catalogitem)
  - [Method `new`](#symbol-showcase-catalog-rs-method-new-2)
  - [Method `id`](#symbol-showcase-catalog-rs-method-id)
  - [Method `title`](#symbol-showcase-catalog-rs-method-title)
  - [Method `payload`](#symbol-showcase-catalog-rs-method-payload)
  - [Method `status`](#symbol-showcase-catalog-rs-method-status)
  - [Method `publish`](#symbol-showcase-catalog-rs-method-publish)
  - [Method `archive`](#symbol-showcase-catalog-rs-method-archive)
  - [Trait `Summary`](#symbol-showcase-catalog-rs-trait-summary)
  - [Method `summary`](#symbol-showcase-catalog-rs-method-summary)
  - [Method `summary`](#symbol-showcase-catalog-rs-method-summary-2)
  - [Constant `MAX_TITLE_LENGTH`](#symbol-showcase-catalog-rs-constant-max-title-length)
- [`showcase/cpp_api.cpp`](#file-showcase-cpp-api-cpp)
  - [Module `examples`](#symbol-showcase-cpp-api-cpp-module-examples)
  - [Struct `Greeter`](#symbol-showcase-cpp-api-cpp-struct-greeter)
  - [Method `greet`](#symbol-showcase-cpp-api-cpp-method-greet)
  - [Function `add_values`](#symbol-showcase-cpp-api-cpp-function-add-values)
- [`showcase/data_model.py`](#file-showcase-data-model-py)
  - [Constant `MAX_ITEMS`](#symbol-showcase-data-model-py-constant-max-items)
  - [Struct `Catalog`](#symbol-showcase-data-model-py-struct-catalog)
  - [Method `find`](#symbol-showcase-data-model-py-method-find)
  - [Function `normalize_title`](#symbol-showcase-data-model-py-function-normalize-title)
- [`showcase/lib.rs`](#file-showcase-lib-rs)
  - [Module `catalog`](#symbol-showcase-lib-rs-module-catalog)
  - [Module `service`](#symbol-showcase-lib-rs-module-service)
  - [Module `markdown_examples`](#symbol-showcase-lib-rs-module-markdown-examples)
- [`showcase/markdown_examples.rs`](#file-showcase-markdown-examples-rs)
  - [Function `markdown_features_example`](#symbol-showcase-markdown-examples-rs-function-markdown-features-example)
- [`showcase/plugin.lua`](#file-showcase-plugin-lua)
  - [Function `plugin.add`](#symbol-showcase-plugin-lua-function-plugin-add)
  - [Method `plugin:label`](#symbol-showcase-plugin-lua-method-plugin-label)
- [`showcase/service.rs`](#file-showcase-service-rs)
  - [Struct `Catalog`](#symbol-showcase-service-rs-struct-catalog)
  - [Method `new`](#symbol-showcase-service-rs-method-new)
  - [Method `find`](#symbol-showcase-service-rs-method-find)
  - [Method `len`](#symbol-showcase-service-rs-method-len)
  - [Method `is_empty`](#symbol-showcase-service-rs-method-is-empty)
  - [Function `render_public_item`](#symbol-showcase-service-rs-function-render-public-item)
- [`src/adapters/c_like.rs`](#file-src-adapters-c-like-rs)
  - [Struct `CAdapter`](#symbol-src-adapters-c-like-rs-struct-cadapter)
  - [Struct `CppAdapter`](#symbol-src-adapters-c-like-rs-struct-cppadapter)
  - [Method `id`](#symbol-src-adapters-c-like-rs-method-id)
  - [Method `extensions`](#symbol-src-adapters-c-like-rs-method-extensions)
  - [Method `language`](#symbol-src-adapters-c-like-rs-method-language)
  - [Method `extract`](#symbol-src-adapters-c-like-rs-method-extract)
  - [Method `id`](#symbol-src-adapters-c-like-rs-method-id-2)
  - [Method `extensions`](#symbol-src-adapters-c-like-rs-method-extensions-2)
  - [Method `language`](#symbol-src-adapters-c-like-rs-method-language-2)
  - [Method `extract`](#symbol-src-adapters-c-like-rs-method-extract-2)
  - [Function `extract_c_family`](#symbol-src-adapters-c-like-rs-function-extract-c-family)
  - [Function `collect_symbols`](#symbol-src-adapters-c-like-rs-function-collect-symbols)
  - [Function `declaration_kind`](#symbol-src-adapters-c-like-rs-function-declaration-kind)
  - [Function `named_children`](#symbol-src-adapters-c-like-rs-function-named-children)
  - [Function `module_docs`](#symbol-src-adapters-c-like-rs-function-module-docs)
  - [Function `contains_type_specifier`](#symbol-src-adapters-c-like-rs-function-contains-type-specifier)
  - [Function `find_function_declarator`](#symbol-src-adapters-c-like-rs-function-find-function-declarator)
  - [Function `function_name`](#symbol-src-adapters-c-like-rs-function-function-name)
  - [Function `deepest_name`](#symbol-src-adapters-c-like-rs-function-deepest-name)
  - [Function `signature`](#symbol-src-adapters-c-like-rs-function-signature)
  - [Function `leading_docs`](#symbol-src-adapters-c-like-rs-function-leading-docs)
  - [Function `clean_doc_comment`](#symbol-src-adapters-c-like-rs-function-clean-doc-comment)
- [`src/adapters/lua.rs`](#file-src-adapters-lua-rs)
  - [Struct `LuaAdapter`](#symbol-src-adapters-lua-rs-struct-luaadapter)
  - [Method `id`](#symbol-src-adapters-lua-rs-method-id)
  - [Method `extensions`](#symbol-src-adapters-lua-rs-method-extensions)
  - [Method `language`](#symbol-src-adapters-lua-rs-method-language)
  - [Method `extract`](#symbol-src-adapters-lua-rs-method-extract)
  - [Function `extract_lua`](#symbol-src-adapters-lua-rs-function-extract-lua)
  - [Function `collect_functions`](#symbol-src-adapters-lua-rs-function-collect-functions)
  - [Function `named_children`](#symbol-src-adapters-lua-rs-function-named-children)
  - [Function `module_docs`](#symbol-src-adapters-lua-rs-function-module-docs)
  - [Function `leading_docs`](#symbol-src-adapters-lua-rs-function-leading-docs)
  - [Function `clean_doc_comment`](#symbol-src-adapters-lua-rs-function-clean-doc-comment)
- [`src/adapters/mod.rs`](#file-src-adapters-mod-rs)
  - [Module `c_like`](#symbol-src-adapters-mod-rs-module-c-like)
  - [Module `lua`](#symbol-src-adapters-mod-rs-module-lua)
  - [Module `python`](#symbol-src-adapters-mod-rs-module-python)
  - [Module `rust`](#symbol-src-adapters-mod-rs-module-rust)
  - [Trait `LanguageAdapter`](#symbol-src-adapters-mod-rs-trait-languageadapter)
  - [Method `id`](#symbol-src-adapters-mod-rs-method-id)
  - [Method `extensions`](#symbol-src-adapters-mod-rs-method-extensions)
  - [Method `language`](#symbol-src-adapters-mod-rs-method-language)
  - [Method `extract`](#symbol-src-adapters-mod-rs-method-extract)
  - [Constant `RUST`](#symbol-src-adapters-mod-rs-constant-rust)
  - [Constant `C`](#symbol-src-adapters-mod-rs-constant-c)
  - [Constant `CPP`](#symbol-src-adapters-mod-rs-constant-cpp)
  - [Constant `PYTHON`](#symbol-src-adapters-mod-rs-constant-python)
  - [Constant `LUA`](#symbol-src-adapters-mod-rs-constant-lua)
  - [Constant `ADAPTERS`](#symbol-src-adapters-mod-rs-constant-adapters)
  - [Function `adapter_for_path`](#symbol-src-adapters-mod-rs-function-adapter-for-path)
  - [Function `adapter_by_id`](#symbol-src-adapters-mod-rs-function-adapter-by-id)
  - [Function `parser_for`](#symbol-src-adapters-mod-rs-function-parser-for)
  - [Function `diagnostics`](#symbol-src-adapters-mod-rs-function-diagnostics)
  - [Function `source_span`](#symbol-src-adapters-mod-rs-function-source-span)
  - [Function `node_text`](#symbol-src-adapters-mod-rs-function-node-text)
- [`src/adapters/python.rs`](#file-src-adapters-python-rs)
  - [Struct `PythonAdapter`](#symbol-src-adapters-python-rs-struct-pythonadapter)
  - [Method `id`](#symbol-src-adapters-python-rs-method-id)
  - [Method `extensions`](#symbol-src-adapters-python-rs-method-extensions)
  - [Method `language`](#symbol-src-adapters-python-rs-method-language)
  - [Method `extract`](#symbol-src-adapters-python-rs-method-extract)
  - [Function `extract_python`](#symbol-src-adapters-python-rs-function-extract-python)
  - [Function `collect_symbols`](#symbol-src-adapters-python-rs-function-collect-symbols)
  - [Function `constant_assignment`](#symbol-src-adapters-python-rs-function-constant-assignment)
  - [Function `suite_docstring`](#symbol-src-adapters-python-rs-function-suite-docstring)
  - [Function `unquote_docstring`](#symbol-src-adapters-python-rs-function-unquote-docstring)
  - [Function `dedent_docstring`](#symbol-src-adapters-python-rs-function-dedent-docstring)
- [`src/adapters/rust.rs`](#file-src-adapters-rust-rs)
  - [Struct `RustAdapter`](#symbol-src-adapters-rust-rs-struct-rustadapter)
  - [Method `id`](#symbol-src-adapters-rust-rs-method-id)
  - [Method `extensions`](#symbol-src-adapters-rust-rs-method-extensions)
  - [Method `language`](#symbol-src-adapters-rust-rs-method-language)
  - [Method `extract`](#symbol-src-adapters-rust-rs-method-extract)
- [`src/config.rs`](#file-src-config-rs)
  - [Struct `Config`](#symbol-src-config-rs-struct-config)
  - [Struct `FileConfig`](#symbol-src-config-rs-struct-fileconfig)
  - [Method `default`](#symbol-src-config-rs-method-default)
  - [Method `load`](#symbol-src-config-rs-method-load)
  - [Method `with_output`](#symbol-src-config-rs-method-with-output)
  - [Method `validate`](#symbol-src-config-rs-method-validate)
  - [Function `resolve`](#symbol-src-config-rs-function-resolve)
  - [Module `tests`](#symbol-src-config-rs-module-tests)
  - [Function `loads_partial_config_and_resolves_paths_from_config_directory`](#symbol-src-config-rs-function-loads-partial-config-and-resolves-paths-from-config-directory)
  - [Function `rejects_unknown_keys_and_invalid_values`](#symbol-src-config-rs-function-rejects-unknown-keys-and-invalid-values)
- [`src/html.rs`](#file-src-html-rs)
  - [Function `render`](#symbol-src-html-rs-function-render)
  - [Function `render_markdown`](#symbol-src-html-rs-function-render-markdown)
  - [Function `safe_url`](#symbol-src-html-rs-function-safe-url)
  - [Function `escape_html`](#symbol-src-html-rs-function-escape-html)
  - [Function `kind_label`](#symbol-src-html-rs-function-kind-label)
  - [Function `syntax_language`](#symbol-src-html-rs-function-syntax-language)
  - [Function `source_link`](#symbol-src-html-rs-function-source-link)
  - [Function `relative_path`](#symbol-src-html-rs-function-relative-path)
  - [Function `encode_path`](#symbol-src-html-rs-function-encode-path)
- [`src/json.rs`](#file-src-json-rs)
  - [Function `render`](#symbol-src-json-rs-function-render)
- [`src/lib.rs`](#file-src-lib-rs)
  - [Module `adapters`](#symbol-src-lib-rs-module-adapters)
  - [Module `config`](#symbol-src-lib-rs-module-config)
  - [Module `html`](#symbol-src-lib-rs-module-html)
  - [Module `json`](#symbol-src-lib-rs-module-json)
  - [Module `markdown`](#symbol-src-lib-rs-module-markdown)
  - [Module `model`](#symbol-src-lib-rs-module-model)
  - [Module `rust_extract`](#symbol-src-lib-rs-module-rust-extract)
  - [Struct `ParsedFile`](#symbol-src-lib-rs-struct-parsedfile)
  - [Function `scan_rust`](#symbol-src-lib-rs-function-scan-rust)
  - [Function `scan_project`](#symbol-src-lib-rs-function-scan-project)
  - [Function `scan_sources`](#symbol-src-lib-rs-function-scan-sources)
  - [Function `document_rust`](#symbol-src-lib-rs-function-document-rust)
  - [Function `document_project`](#symbol-src-lib-rs-function-document-project)
  - [Function `generate_documentation`](#symbol-src-lib-rs-function-generate-documentation)
  - [Function `generate_markdown`](#symbol-src-lib-rs-function-generate-markdown)
  - [Function `syntax_issue_count`](#symbol-src-lib-rs-function-syntax-issue-count)
  - [Module `tests`](#symbol-src-lib-rs-module-tests)
  - [Function `scans_and_parses_rust_files`](#symbol-src-lib-rs-function-scans-and-parses-rust-files)
  - [Function `detects_invalid_rust_syntax`](#symbol-src-lib-rs-function-detects-invalid-rust-syntax)
  - [Function `builds_a_project_documentation_model`](#symbol-src-lib-rs-function-builds-a-project-documentation-model)
- [`src/main.rs`](#file-src-main-rs)
  - [Struct `Cli`](#symbol-src-main-rs-struct-cli)
  - [Enum `Command`](#symbol-src-main-rs-enum-command)
  - [Function `main`](#symbol-src-main-rs-function-main)
- [`src/markdown.rs`](#file-src-markdown-rs)
  - [Function `render`](#symbol-src-markdown-rs-function-render)
  - [Struct `Anchors`](#symbol-src-markdown-rs-struct-anchors)
  - [Method `new`](#symbol-src-markdown-rs-method-new)
  - [Function `unique_anchor`](#symbol-src-markdown-rs-function-unique-anchor)
  - [Function `slug`](#symbol-src-markdown-rs-function-slug)
  - [Function `kind_label`](#symbol-src-markdown-rs-function-kind-label)
  - [Function `source_link`](#symbol-src-markdown-rs-function-source-link)
  - [Function `project_relative_path`](#symbol-src-markdown-rs-function-project-relative-path)
  - [Function `relative_path`](#symbol-src-markdown-rs-function-relative-path)
  - [Function `encode_path`](#symbol-src-markdown-rs-function-encode-path)
  - [Function `encode_url_base`](#symbol-src-markdown-rs-function-encode-url-base)
  - [Function `code_span`](#symbol-src-markdown-rs-function-code-span)
  - [Function `fenced_code`](#symbol-src-markdown-rs-function-fenced-code)
  - [Function `syntax_language`](#symbol-src-markdown-rs-function-syntax-language)
  - [Function `max_backtick_run`](#symbol-src-markdown-rs-function-max-backtick-run)
  - [Function `escape_heading`](#symbol-src-markdown-rs-function-escape-heading)
  - [Module `tests`](#symbol-src-markdown-rs-module-tests)
  - [Function `fixture`](#symbol-src-markdown-rs-function-fixture)
  - [Function `renders_deterministic_anchors_and_local_source_links`](#symbol-src-markdown-rs-function-renders-deterministic-anchors-and-local-source-links)
  - [Function `remote_links_encode_source_paths_and_duplicate_anchors_are_unique`](#symbol-src-markdown-rs-function-remote-links-encode-source-paths-and-duplicate-anchors-are-unique)
  - [Function `inline_code_delimiters_do_not_break_on_backticks_in_names`](#symbol-src-markdown-rs-function-inline-code-delimiters-do-not-break-on-backticks-in-names)
- [`src/model.rs`](#file-src-model-rs)
  - [Struct `DocProject`](#symbol-src-model-rs-struct-docproject)
  - [Struct `DocFile`](#symbol-src-model-rs-struct-docfile)
  - [Struct `DocSymbol`](#symbol-src-model-rs-struct-docsymbol)
  - [Enum `SymbolKind`](#symbol-src-model-rs-enum-symbolkind)
  - [Struct `SourceSpan`](#symbol-src-model-rs-struct-sourcespan)
  - [Struct `Diagnostic`](#symbol-src-model-rs-struct-diagnostic)
  - [Enum `DiagnosticSeverity`](#symbol-src-model-rs-enum-diagnosticseverity)
- [`src/rust_extract.rs`](#file-src-rust-extract-rs)
  - [Function `extract_rust`](#symbol-src-rust-extract-rs-function-extract-rust)
  - [Function `collect_items`](#symbol-src-rust-extract-rs-function-collect-items)
  - [Function `symbol_kind`](#symbol-src-rust-extract-rs-function-symbol-kind)
  - [Function `leading_docs`](#symbol-src-rust-extract-rs-function-leading-docs)
  - [Function `inner_docs`](#symbol-src-rust-extract-rs-function-inner-docs)
  - [Function `signature`](#symbol-src-rust-extract-rs-function-signature)
  - [Function `span`](#symbol-src-rust-extract-rs-function-span)
  - [Function `collect_diagnostics`](#symbol-src-rust-extract-rs-function-collect-diagnostics)
  - [Module `tests`](#symbol-src-rust-extract-rs-module-tests)
  - [Function `parse`](#symbol-src-rust-extract-rs-function-parse)
  - [Function `extracts_nested_items_generics_attributes_and_docs`](#symbol-src-rust-extract-rs-function-extracts-nested-items-generics-attributes-and-docs)
  - [Function `malformed_syntax_becomes_a_diagnostic`](#symbol-src-rust-extract-rs-function-malformed-syntax-becomes-a-diagnostic)
  - [Function `fixture_covers_nested_generics_attributes_and_doc_comments`](#symbol-src-rust-extract-rs-function-fixture-covers-nested-generics-attributes-and-doc-comments)
- [`tests/fixtures/malformed.rs`](#file-tests-fixtures-malformed-rs)
- [`tests/fixtures/nested_generics.rs`](#file-tests-fixtures-nested-generics-rs)
  - [Module `outer`](#symbol-tests-fixtures-nested-generics-rs-module-outer)
  - [Struct `Record`](#symbol-tests-fixtures-nested-generics-rs-struct-record)
  - [Trait `Convert`](#symbol-tests-fixtures-nested-generics-rs-trait-convert)
  - [Method `convert`](#symbol-tests-fixtures-nested-generics-rs-method-convert)
  - [Method `value`](#symbol-tests-fixtures-nested-generics-rs-method-value)
  - [Enum `Choice`](#symbol-tests-fixtures-nested-generics-rs-enum-choice)
  - [Constant `ANSWER`](#symbol-tests-fixtures-nested-generics-rs-constant-answer)
  - [Function `make`](#symbol-tests-fixtures-nested-generics-rs-function-make)
- [`tests/generate_markdown.rs`](#file-tests-generate-markdown-rs)
  - [Function `cli_generates_markdown_with_anchors_docs_and_source_links`](#symbol-tests-generate-markdown-rs-function-cli-generates-markdown-with-anchors-docs-and-source-links)
  - [Function `cli_rejects_invalid_configuration_without_creating_output`](#symbol-tests-generate-markdown-rs-function-cli-rejects-invalid-configuration-without-creating-output)

<a id="file-showcase-c-api-c"></a>
## `showcase/c_api.c`

# C adapter example

This file demonstrates a block documentation comment and a C function.

<a id="symbol-showcase-c-api-c-function-multiply-values"></a>
### Function: `multiply_values`

```c
int multiply_values(int left, int right)
```

Multiply two integers and return the result.

**Parameters:** `left` and `right` are the input values.

[View source](../showcase/c_api.c#L11)

<a id="file-showcase-catalog-rs"></a>
## `showcase/catalog.rs`

Tipos de domínio usados pelo catálogo.

Este módulo serve como exemplo de documentação de módulo e de declarações
com campos, variantes e métodos.

<a id="symbol-showcase-catalog-rs-struct-itemid"></a>
### Struct: `ItemId`

```rust
pub struct ItemId(String);
```

Identificador estável de um item.

O valor é mantido como texto para que exemplos possam usar IDs legíveis:
`"book-rust-101"`.

[View source](../showcase/catalog.rs#L13)

<a id="symbol-showcase-catalog-rs-method-new"></a>
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

<a id="symbol-showcase-catalog-rs-method-as-str"></a>
### Method: `as_str`

```rust
pub fn as_str(&self) -> &str
```

Retorna o identificador como uma fatia de texto.

[View source](../showcase/catalog.rs#L29)

<a id="symbol-showcase-catalog-rs-method-fmt"></a>
### Method: `fmt`

```rust
fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

[View source](../showcase/catalog.rs#L35)

<a id="symbol-showcase-catalog-rs-enum-itemstatus"></a>
### Enum: `ItemStatus`

```rust
pub enum ItemStatus
```

Situação editorial de um item do catálogo.

[View source](../showcase/catalog.rs#L42)

<a id="symbol-showcase-catalog-rs-struct-catalogitem"></a>
### Struct: `CatalogItem`

```rust
pub struct CatalogItem<T>
```

Um item do catálogo com conteúdo definido pelo consumidor da biblioteca.

`T` pode representar uma página, um registro ou qualquer outro payload.

[View source](../showcase/catalog.rs#L55)

<a id="symbol-showcase-catalog-rs-method-new-2"></a>
### Method: `new`

```rust
pub fn new(id: ItemId, title: impl Into<String>, payload: T) -> Self
```

Cria um item com status inicial `Draft`.

[View source](../showcase/catalog.rs#L64)

<a id="symbol-showcase-catalog-rs-method-id"></a>
### Method: `id`

```rust
pub fn id(&self) -> &ItemId
```

Devolve o identificador do item.

[View source](../showcase/catalog.rs#L74)

<a id="symbol-showcase-catalog-rs-method-title"></a>
### Method: `title`

```rust
pub fn title(&self) -> &str
```

Devolve o título do item.

[View source](../showcase/catalog.rs#L79)

<a id="symbol-showcase-catalog-rs-method-payload"></a>
### Method: `payload`

```rust
pub fn payload(&self) -> &T
```

Devolve o payload sem transferir sua propriedade.

[View source](../showcase/catalog.rs#L84)

<a id="symbol-showcase-catalog-rs-method-status"></a>
### Method: `status`

```rust
pub fn status(&self) -> &ItemStatus
```

Devolve o status editorial atual.

[View source](../showcase/catalog.rs#L89)

<a id="symbol-showcase-catalog-rs-method-publish"></a>
### Method: `publish`

```rust
pub fn publish(&mut self)
```

Publica o item para que ele possa ser exibido em resultados.

[View source](../showcase/catalog.rs#L94)

<a id="symbol-showcase-catalog-rs-method-archive"></a>
### Method: `archive`

```rust
pub fn archive(&mut self)
```

Arquiva o item, mantendo seus dados disponíveis para consulta interna.

[View source](../showcase/catalog.rs#L99)

<a id="symbol-showcase-catalog-rs-trait-summary"></a>
### Trait: `Summary`

```rust
pub trait Summary
```

Fornece um resumo legível de um item.

[View source](../showcase/catalog.rs#L105)

<a id="symbol-showcase-catalog-rs-method-summary"></a>
### Method: `summary`

```rust
fn summary(&self) -> String;
```

Monta uma linha de resumo para listas e resultados de busca.

[View source](../showcase/catalog.rs#L107)

<a id="symbol-showcase-catalog-rs-method-summary-2"></a>
### Method: `summary`

```rust
fn summary(&self) -> String
```

[View source](../showcase/catalog.rs#L111)

<a id="symbol-showcase-catalog-rs-constant-max-title-length"></a>
### Constant: `MAX_TITLE_LENGTH`

```rust
pub const MAX_TITLE_LENGTH: usize = 120;
```

Tamanho máximo recomendado para títulos exibidos em uma linha.

[View source](../showcase/catalog.rs#L117)

<a id="file-showcase-cpp-api-cpp"></a>
## `showcase/cpp_api.cpp`

# C++ adapter example

C++ namespaces and classes map into the shared module and struct kinds.

<a id="symbol-showcase-cpp-api-cpp-module-examples"></a>
### Module: `examples`

```cpp
namespace examples
```

[View source](../showcase/cpp_api.cpp#L4)

<a id="symbol-showcase-cpp-api-cpp-struct-greeter"></a>
### Struct: `Greeter`

```cpp
class Greeter
```

A small type with one documented method.

[View source](../showcase/cpp_api.cpp#L7)

<a id="symbol-showcase-cpp-api-cpp-method-greet"></a>
### Method: `greet`

```cpp
const char *greet(const char *name) const
```

Return a greeting for `name`.

[View source](../showcase/cpp_api.cpp#L10)

<a id="symbol-showcase-cpp-api-cpp-function-add-values"></a>
### Function: `add_values`

```cpp
int add_values(int left, int right);
```

Add two integers.

[View source](../showcase/cpp_api.cpp#L16)

<a id="file-showcase-data-model-py"></a>
## `showcase/data_model.py`

# Python adapter example

Python module, class, and function docstrings become Markdown documentation.

<a id="symbol-showcase-data-model-py-constant-max-items"></a>
### Constant: `MAX_ITEMS`

```python
MAX_ITEMS = 25
```

[View source](../showcase/data_model.py#L6)

<a id="symbol-showcase-data-model-py-struct-catalog"></a>
### Struct: `Catalog`

```python
class Catalog:
```

A small collection with a documented lookup method.

[View source](../showcase/data_model.py#L9)

<a id="symbol-showcase-data-model-py-method-find"></a>
### Method: `find`

```python
def find(self, item_id: str) -> str | None:
```

Find an item by its stable identifier.

Returns `None` when the item is not present.

[View source](../showcase/data_model.py#L12)

<a id="symbol-showcase-data-model-py-function-normalize-title"></a>
### Function: `normalize_title`

```python
def normalize_title(value: str) -> str:
```

Normalize a title by trimming surrounding whitespace.

[View source](../showcase/data_model.py#L20)

<a id="file-showcase-lib-rs"></a>
## `showcase/lib.rs`

# DocWeave showcase

Um pequeno crate de exemplo para demonstrar a extração de comentários Rust.
Os módulos incluem comentários de arquivo (`//!`), documentação de itens
(`///`), Markdown, atributos, genéricos, traits e métodos.

Comece pelos [tipos do catálogo](crate::catalog) e depois veja o
[serviço de consulta](crate::service).

<a id="symbol-showcase-lib-rs-module-catalog"></a>
### Module: `catalog`

```rust
pub mod catalog;
```

Tipos que descrevem itens disponíveis no catálogo.

[View source](../showcase/lib.rs#L11)

<a id="symbol-showcase-lib-rs-module-service"></a>
### Module: `service`

```rust
pub mod service;
```

Operações de consulta e apresentação do catálogo.

[View source](../showcase/lib.rs#L14)

<a id="symbol-showcase-lib-rs-module-markdown-examples"></a>
### Module: `markdown_examples`

```rust
pub mod markdown_examples;
```

Exemplos de sintaxe Markdown em comentários de documentação.

[View source](../showcase/lib.rs#L17)

<a id="file-showcase-markdown-examples-rs"></a>
## `showcase/markdown_examples.rs`

Exemplos de construções Markdown preservadas nos comentários de documentação.

<a id="symbol-showcase-markdown-examples-rs-function-markdown-features-example"></a>
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

<a id="file-showcase-plugin-lua"></a>
## `showcase/plugin.lua`

# Lua adapter example
Lua `---` comments immediately above a function are treated as docs.

<a id="symbol-showcase-plugin-lua-function-plugin-add"></a>
### Function: `plugin.add`

```lua
function plugin.add(left, right)
```

Calculate the sum of two numbers.

[View source](../showcase/plugin.lua#L5)

<a id="symbol-showcase-plugin-lua-method-plugin-label"></a>
### Method: `plugin:label`

```lua
function plugin:label(value)
```

Format a label using method-style syntax.

[View source](../showcase/plugin.lua#L11)

<a id="file-showcase-service-rs"></a>
## `showcase/service.rs`

Operações de consulta para o catálogo.

O serviço mantém uma coleção em memória para que os exemplos permaneçam
pequenos e não dependam de banco de dados ou rede.

<a id="symbol-showcase-service-rs-struct-catalog"></a>
### Struct: `Catalog`

```rust
pub struct Catalog<T>
```

Coleção consultável de itens com payload do tipo `T`.

O catálogo preserva a ordem recebida e permite localizar itens pelo ID.

[View source](../showcase/service.rs#L14)

<a id="symbol-showcase-service-rs-method-new"></a>
### Method: `new`

```rust
pub fn new(items: impl IntoIterator<Item = CatalogItem<T>>) -> Self
```

Cria um catálogo a partir de uma sequência de itens.

[View source](../showcase/service.rs#L20)

<a id="symbol-showcase-service-rs-method-find"></a>
### Method: `find`

```rust
pub fn find(&self, id: &ItemId) -> Option<&CatalogItem<T>>
```

Procura um item pelo seu identificador.

Retorna `None` quando o ID não está presente.

[View source](../showcase/service.rs#L29)

<a id="symbol-showcase-service-rs-method-len"></a>
### Method: `len`

```rust
pub fn len(&self) -> usize
```

Conta os itens armazenados, incluindo rascunhos e itens arquivados.

[View source](../showcase/service.rs#L34)

<a id="symbol-showcase-service-rs-method-is-empty"></a>
### Method: `is_empty`

```rust
pub fn is_empty(&self) -> bool
```

Informa se o catálogo não contém itens.

[View source](../showcase/service.rs#L39)

<a id="symbol-showcase-service-rs-function-render-public-item"></a>
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

<a id="file-src-adapters-c-like-rs"></a>
## `src/adapters/c_like.rs`

C and C++ declaration extraction from their related Tree-sitter grammars.

<a id="symbol-src-adapters-c-like-rs-struct-cadapter"></a>
### Struct: `CAdapter`

```rust
pub struct CAdapter;
```

[View source](../src/adapters/c_like.rs#L11)

<a id="symbol-src-adapters-c-like-rs-struct-cppadapter"></a>
### Struct: `CppAdapter`

```rust
pub struct CppAdapter;
```

[View source](../src/adapters/c_like.rs#L12)

<a id="symbol-src-adapters-c-like-rs-method-id"></a>
### Method: `id`

```rust
fn id(&self) -> &'static str
```

[View source](../src/adapters/c_like.rs#L15)

<a id="symbol-src-adapters-c-like-rs-method-extensions"></a>
### Method: `extensions`

```rust
fn extensions(&self) -> &'static [&'static str]
```

[View source](../src/adapters/c_like.rs#L18)

<a id="symbol-src-adapters-c-like-rs-method-language"></a>
### Method: `language`

```rust
fn language(&self) -> Language
```

[View source](../src/adapters/c_like.rs#L21)

<a id="symbol-src-adapters-c-like-rs-method-extract"></a>
### Method: `extract`

```rust
fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile
```

[View source](../src/adapters/c_like.rs#L24)

<a id="symbol-src-adapters-c-like-rs-method-id-2"></a>
### Method: `id`

```rust
fn id(&self) -> &'static str
```

[View source](../src/adapters/c_like.rs#L30)

<a id="symbol-src-adapters-c-like-rs-method-extensions-2"></a>
### Method: `extensions`

```rust
fn extensions(&self) -> &'static [&'static str]
```

[View source](../src/adapters/c_like.rs#L33)

<a id="symbol-src-adapters-c-like-rs-method-language-2"></a>
### Method: `language`

```rust
fn language(&self) -> Language
```

[View source](../src/adapters/c_like.rs#L36)

<a id="symbol-src-adapters-c-like-rs-method-extract-2"></a>
### Method: `extract`

```rust
fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile
```

[View source](../src/adapters/c_like.rs#L39)

<a id="symbol-src-adapters-c-like-rs-function-extract-c-family"></a>
### Function: `extract_c_family`

```rust
fn extract_c_family(path: &Path, source: &str, tree: &Tree, language: &str) -> DocFile
```

[View source](../src/adapters/c_like.rs#L44)

<a id="symbol-src-adapters-c-like-rs-function-collect-symbols"></a>
### Function: `collect_symbols`

```rust
fn collect_symbols(
    parent: Node<'_>,
    parent_scope: Option<SymbolKind>,
    file_documentation: Option<&str>,
    source: &str,
    path: &str,
    symbols: &mut Vec<DocSymbol>,
)
```

[View source](../src/adapters/c_like.rs#L66)

<a id="symbol-src-adapters-c-like-rs-function-declaration-kind"></a>
### Function: `declaration_kind`

```rust
fn declaration_kind(node_kind: &str) -> Option<SymbolKind>
```

[View source](../src/adapters/c_like.rs#L132)

<a id="symbol-src-adapters-c-like-rs-function-named-children"></a>
### Function: `named_children`

```rust
fn named_children(node: Node<'_>) -> Vec<Node<'_>>
```

[View source](../src/adapters/c_like.rs#L141)

<a id="symbol-src-adapters-c-like-rs-function-module-docs"></a>
### Function: `module_docs`

```rust
fn module_docs(children: &[Node<'_>], source: &str) -> Option<String>
```

[View source](../src/adapters/c_like.rs#L146)

<a id="symbol-src-adapters-c-like-rs-function-contains-type-specifier"></a>
### Function: `contains_type_specifier`

```rust
fn contains_type_specifier(node: Node<'_>) -> bool
```

[View source](../src/adapters/c_like.rs#L172)

<a id="symbol-src-adapters-c-like-rs-function-find-function-declarator"></a>
### Function: `find_function_declarator`

```rust
fn find_function_declarator(node: Node<'_>) -> Option<Node<'_>>
```

[View source](../src/adapters/c_like.rs#L184)

<a id="symbol-src-adapters-c-like-rs-function-function-name"></a>
### Function: `function_name`

```rust
fn function_name(function: Node<'_>) -> Option<Node<'_>>
```

[View source](../src/adapters/c_like.rs#L193)

<a id="symbol-src-adapters-c-like-rs-function-deepest-name"></a>
### Function: `deepest_name`

```rust
fn deepest_name(node: Node<'_>) -> Option<Node<'_>>
```

[View source](../src/adapters/c_like.rs#L198)

<a id="symbol-src-adapters-c-like-rs-function-signature"></a>
### Function: `signature`

```rust
fn signature(node: Node<'_>, source: &str) -> Option<String>
```

[View source](../src/adapters/c_like.rs#L209)

<a id="symbol-src-adapters-c-like-rs-function-leading-docs"></a>
### Function: `leading_docs`

```rust
fn leading_docs(children: &[Node<'_>], item_index: usize, source: &str) -> Option<String>
```

[View source](../src/adapters/c_like.rs#L220)

<a id="symbol-src-adapters-c-like-rs-function-clean-doc-comment"></a>
### Function: `clean_doc_comment`

```rust
fn clean_doc_comment(raw: &str) -> Option<String>
```

[View source](../src/adapters/c_like.rs#L246)

<a id="file-src-adapters-lua-rs"></a>
## `src/adapters/lua.rs`

Lua adapter for documented named functions and method-style declarations.

<a id="symbol-src-adapters-lua-rs-struct-luaadapter"></a>
### Struct: `LuaAdapter`

```rust
pub struct LuaAdapter;
```

[View source](../src/adapters/lua.rs#L11)

<a id="symbol-src-adapters-lua-rs-method-id"></a>
### Method: `id`

```rust
fn id(&self) -> &'static str
```

[View source](../src/adapters/lua.rs#L14)

<a id="symbol-src-adapters-lua-rs-method-extensions"></a>
### Method: `extensions`

```rust
fn extensions(&self) -> &'static [&'static str]
```

[View source](../src/adapters/lua.rs#L17)

<a id="symbol-src-adapters-lua-rs-method-language"></a>
### Method: `language`

```rust
fn language(&self) -> Language
```

[View source](../src/adapters/lua.rs#L20)

<a id="symbol-src-adapters-lua-rs-method-extract"></a>
### Method: `extract`

```rust
fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile
```

[View source](../src/adapters/lua.rs#L23)

<a id="symbol-src-adapters-lua-rs-function-extract-lua"></a>
### Function: `extract_lua`

```rust
fn extract_lua(path: &Path, source: &str, tree: &Tree) -> DocFile
```

[View source](../src/adapters/lua.rs#L28)

<a id="symbol-src-adapters-lua-rs-function-collect-functions"></a>
### Function: `collect_functions`

```rust
fn collect_functions(parent: Node<'_>, source: &str, path: &str, symbols: &mut Vec<DocSymbol>)
```

[View source](../src/adapters/lua.rs#L43)

<a id="symbol-src-adapters-lua-rs-function-named-children"></a>
### Function: `named_children`

```rust
fn named_children(node: Node<'_>) -> Vec<Node<'_>>
```

[View source](../src/adapters/lua.rs#L77)

<a id="symbol-src-adapters-lua-rs-function-module-docs"></a>
### Function: `module_docs`

```rust
fn module_docs(children: &[Node<'_>], source: &str) -> Option<String>
```

[View source](../src/adapters/lua.rs#L82)

<a id="symbol-src-adapters-lua-rs-function-leading-docs"></a>
### Function: `leading_docs`

```rust
fn leading_docs(children: &[Node<'_>], item_index: usize, source: &str) -> Option<String>
```

[View source](../src/adapters/lua.rs#L108)

<a id="symbol-src-adapters-lua-rs-function-clean-doc-comment"></a>
### Function: `clean_doc_comment`

```rust
fn clean_doc_comment(raw: &str) -> Option<String>
```

[View source](../src/adapters/lua.rs#L134)

<a id="file-src-adapters-mod-rs"></a>
## `src/adapters/mod.rs`

Source-language adapters that normalize syntax trees into the shared model.

<a id="symbol-src-adapters-mod-rs-module-c-like"></a>
### Module: `c_like`

```rust
mod c_like;
```

[View source](../src/adapters/mod.rs#L3)

<a id="symbol-src-adapters-mod-rs-module-lua"></a>
### Module: `lua`

```rust
mod lua;
```

[View source](../src/adapters/mod.rs#L4)

<a id="symbol-src-adapters-mod-rs-module-python"></a>
### Module: `python`

```rust
mod python;
```

[View source](../src/adapters/mod.rs#L5)

<a id="symbol-src-adapters-mod-rs-module-rust"></a>
### Module: `rust`

```rust
mod rust;
```

[View source](../src/adapters/mod.rs#L6)

<a id="symbol-src-adapters-mod-rs-trait-languageadapter"></a>
### Trait: `LanguageAdapter`

```rust
pub trait LanguageAdapter: Sync
```

Contract for a language implementation registered with the source scanner.

An adapter owns file extensions, Tree-sitter grammar selection, and the
mapping from that grammar's nodes and comments into the shared model.

[View source](../src/adapters/mod.rs#L18)

<a id="symbol-src-adapters-mod-rs-method-id"></a>
### Method: `id`

```rust
fn id(&self) -> &'static str;
```

Stable identifier used in diagnostics and the adapter registry.

[View source](../src/adapters/mod.rs#L20)

<a id="symbol-src-adapters-mod-rs-method-extensions"></a>
### Method: `extensions`

```rust
fn extensions(&self) -> &'static [&'static str];
```

File extensions handled by this adapter, without a leading dot.

[View source](../src/adapters/mod.rs#L23)

<a id="symbol-src-adapters-mod-rs-method-language"></a>
### Method: `language`

```rust
fn language(&self) -> Language;
```

Return the Tree-sitter grammar for this language.

[View source](../src/adapters/mod.rs#L26)

<a id="symbol-src-adapters-mod-rs-method-extract"></a>
### Method: `extract`

```rust
fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile;
```

Extract one parsed file into the language-neutral model.

[View source](../src/adapters/mod.rs#L29)

<a id="symbol-src-adapters-mod-rs-constant-rust"></a>
### Constant: `RUST`

```rust
static RUST: rust::RustAdapter = rust::RustAdapter;
```

[View source](../src/adapters/mod.rs#L32)

<a id="symbol-src-adapters-mod-rs-constant-c"></a>
### Constant: `C`

```rust
static C: c_like::CAdapter = c_like::CAdapter;
```

[View source](../src/adapters/mod.rs#L33)

<a id="symbol-src-adapters-mod-rs-constant-cpp"></a>
### Constant: `CPP`

```rust
static CPP: c_like::CppAdapter = c_like::CppAdapter;
```

[View source](../src/adapters/mod.rs#L34)

<a id="symbol-src-adapters-mod-rs-constant-python"></a>
### Constant: `PYTHON`

```rust
static PYTHON: python::PythonAdapter = python::PythonAdapter;
```

[View source](../src/adapters/mod.rs#L35)

<a id="symbol-src-adapters-mod-rs-constant-lua"></a>
### Constant: `LUA`

```rust
static LUA: lua::LuaAdapter = lua::LuaAdapter;
```

[View source](../src/adapters/mod.rs#L36)

<a id="symbol-src-adapters-mod-rs-constant-adapters"></a>
### Constant: `ADAPTERS`

```rust
pub static ADAPTERS: [&dyn LanguageAdapter; 5] = [&RUST, &C, &CPP, &PYTHON, &LUA];
```

Built-in adapters in deterministic extension-dispatch order.

[View source](../src/adapters/mod.rs#L39)

<a id="symbol-src-adapters-mod-rs-function-adapter-for-path"></a>
### Function: `adapter_for_path`

```rust
pub fn adapter_for_path(path: &Path) -> Option<&'static dyn LanguageAdapter>
```

Find the registered adapter responsible for a source path.

[View source](../src/adapters/mod.rs#L42)

<a id="symbol-src-adapters-mod-rs-function-adapter-by-id"></a>
### Function: `adapter_by_id`

```rust
pub fn adapter_by_id(id: &str) -> Option<&'static dyn LanguageAdapter>
```

Find a registered adapter by its stable identifier.

[View source](../src/adapters/mod.rs#L53)

<a id="symbol-src-adapters-mod-rs-function-parser-for"></a>
### Function: `parser_for`

```rust
pub fn parser_for(path: &Path) -> anyhow::Result<Option<(Parser, &'static dyn LanguageAdapter)>>
```

Configure a parser for the adapter selected by a source file extension.

[View source](../src/adapters/mod.rs#L58)

<a id="symbol-src-adapters-mod-rs-function-diagnostics"></a>
### Function: `diagnostics`

```rust
pub fn diagnostics(root: Node<'_>, path: &str, language: &str) -> Vec<Diagnostic>
```

Produce stable parse diagnostics shared by the built-in adapters.

[View source](../src/adapters/mod.rs#L71)

<a id="symbol-src-adapters-mod-rs-function-source-span"></a>
### Function: `source_span`

```rust
pub fn source_span(node: Node<'_>, path: &str) -> SourceSpan
```

Extract a one-based source range from a syntax node.

[View source](../src/adapters/mod.rs#L109)

<a id="symbol-src-adapters-mod-rs-function-node-text"></a>
### Function: `node_text`

```rust
pub fn node_text<'a>(node: Node<'_>, source: &'a str) -> Option<&'a str>
```

Read a node's source text without panicking on invalid UTF-8 boundaries.

[View source](../src/adapters/mod.rs#L122)

<a id="file-src-adapters-python-rs"></a>
## `src/adapters/python.rs`

Python adapter using docstrings as explicit documentation.

<a id="symbol-src-adapters-python-rs-struct-pythonadapter"></a>
### Struct: `PythonAdapter`

```rust
pub struct PythonAdapter;
```

[View source](../src/adapters/python.rs#L11)

<a id="symbol-src-adapters-python-rs-method-id"></a>
### Method: `id`

```rust
fn id(&self) -> &'static str
```

[View source](../src/adapters/python.rs#L14)

<a id="symbol-src-adapters-python-rs-method-extensions"></a>
### Method: `extensions`

```rust
fn extensions(&self) -> &'static [&'static str]
```

[View source](../src/adapters/python.rs#L17)

<a id="symbol-src-adapters-python-rs-method-language"></a>
### Method: `language`

```rust
fn language(&self) -> Language
```

[View source](../src/adapters/python.rs#L20)

<a id="symbol-src-adapters-python-rs-method-extract"></a>
### Method: `extract`

```rust
fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile
```

[View source](../src/adapters/python.rs#L23)

<a id="symbol-src-adapters-python-rs-function-extract-python"></a>
### Function: `extract_python`

```rust
fn extract_python(path: &Path, source: &str, tree: &Tree) -> DocFile
```

[View source](../src/adapters/python.rs#L28)

<a id="symbol-src-adapters-python-rs-function-collect-symbols"></a>
### Function: `collect_symbols`

```rust
fn collect_symbols(
    parent: Node<'_>,
    in_class: bool,
    at_module_scope: bool,
    source: &str,
    path: &str,
    symbols: &mut Vec<DocSymbol>,
)
```

[View source](../src/adapters/python.rs#L42)

<a id="symbol-src-adapters-python-rs-function-constant-assignment"></a>
### Function: `constant_assignment`

```rust
fn constant_assignment(node: Node<'_>, source: &str) -> Option<SymbolKind>
```

[View source](../src/adapters/python.rs#L106)

<a id="symbol-src-adapters-python-rs-function-suite-docstring"></a>
### Function: `suite_docstring`

```rust
fn suite_docstring(suite: Node<'_>, source: &str) -> Option<String>
```

[View source](../src/adapters/python.rs#L115)

<a id="symbol-src-adapters-python-rs-function-unquote-docstring"></a>
### Function: `unquote_docstring`

```rust
fn unquote_docstring(raw: &str) -> String
```

[View source](../src/adapters/python.rs#L130)

<a id="symbol-src-adapters-python-rs-function-dedent-docstring"></a>
### Function: `dedent_docstring`

```rust
fn dedent_docstring(docstring: &str) -> String
```

[View source](../src/adapters/python.rs#L148)

<a id="file-src-adapters-rust-rs"></a>
## `src/adapters/rust.rs`

<a id="symbol-src-adapters-rust-rs-struct-rustadapter"></a>
### Struct: `RustAdapter`

```rust
pub struct RustAdapter;
```

[View source](../src/adapters/rust.rs#L9)

<a id="symbol-src-adapters-rust-rs-method-id"></a>
### Method: `id`

```rust
fn id(&self) -> &'static str
```

[View source](../src/adapters/rust.rs#L12)

<a id="symbol-src-adapters-rust-rs-method-extensions"></a>
### Method: `extensions`

```rust
fn extensions(&self) -> &'static [&'static str]
```

[View source](../src/adapters/rust.rs#L16)

<a id="symbol-src-adapters-rust-rs-method-language"></a>
### Method: `language`

```rust
fn language(&self) -> Language
```

[View source](../src/adapters/rust.rs#L20)

<a id="symbol-src-adapters-rust-rs-method-extract"></a>
### Method: `extract`

```rust
fn extract(&self, path: &Path, source: &str, tree: &Tree) -> DocFile
```

[View source](../src/adapters/rust.rs#L24)

<a id="file-src-config-rs"></a>
## `src/config.rs`

TOML configuration loading and validation.

<a id="symbol-src-config-rs-struct-config"></a>
### Struct: `Config`

```rust
pub struct Config
```

Validated settings for a documentation generation run.

[View source](../src/config.rs#L10)

<a id="symbol-src-config-rs-struct-fileconfig"></a>
### Struct: `FileConfig`

```rust
struct FileConfig
```

[View source](../src/config.rs#L19)

<a id="symbol-src-config-rs-method-default"></a>
### Method: `default`

```rust
fn default() -> Self
```

[View source](../src/config.rs#L27)

<a id="symbol-src-config-rs-method-load"></a>
### Method: `load`

```rust
pub fn load(path: Option<&Path>) -> Result<Self>
```

Load and validate a TOML file. Relative paths are resolved from its directory.
If no path is supplied, use defaults rooted at the current working directory.

[View source](../src/config.rs#L40)

<a id="symbol-src-config-rs-method-with-output"></a>
### Method: `with_output`

```rust
pub fn with_output(mut self, output: PathBuf) -> Result<Self>
```

Override the output path, resolving relative values from the current directory.

[View source](../src/config.rs#L73)

<a id="symbol-src-config-rs-method-validate"></a>
### Method: `validate`

```rust
pub fn validate(&self) -> Result<()>
```

Validate all public settings before a generation run.

[View source](../src/config.rs#L84)

<a id="symbol-src-config-rs-function-resolve"></a>
### Function: `resolve`

```rust
fn resolve(base: &Path, path: PathBuf) -> PathBuf
```

[View source](../src/config.rs#L120)

<a id="symbol-src-config-rs-module-tests"></a>
### Module: `tests`

```rust
mod tests
```

[View source](../src/config.rs#L129)

<a id="symbol-src-config-rs-function-loads-partial-config-and-resolves-paths-from-config-directory"></a>
### Function: `loads_partial_config_and_resolves_paths_from_config_directory`

```rust
fn loads_partial_config_and_resolves_paths_from_config_directory()
```

[View source](../src/config.rs#L133)

<a id="symbol-src-config-rs-function-rejects-unknown-keys-and-invalid-values"></a>
### Function: `rejects_unknown_keys_and_invalid_values`

```rust
fn rejects_unknown_keys_and_invalid_values()
```

[View source](../src/config.rs#L155)

<a id="file-src-html-rs"></a>
## `src/html.rs`

HTML renderer with escaped source text and safe Markdown conversion.

<a id="symbol-src-html-rs-function-render"></a>
### Function: `render`

```rust
pub fn render(
    project: &DocProject,
    title: &str,
    output_parent: &Path,
    source_url: Option<&str>,
) -> String
```

Render a project as a self-contained HTML document.

[View source](../src/html.rs#L10)

<a id="symbol-src-html-rs-function-render-markdown"></a>
### Function: `render_markdown`

```rust
fn render_markdown(markdown: &str) -> String
```

[View source](../src/html.rs#L100)

<a id="symbol-src-html-rs-function-safe-url"></a>
### Function: `safe_url`

```rust
fn safe_url(url: &str) -> bool
```

[View source](../src/html.rs#L144)

<a id="symbol-src-html-rs-function-escape-html"></a>
### Function: `escape_html`

```rust
fn escape_html(value: &str) -> String
```

[View source](../src/html.rs#L162)

<a id="symbol-src-html-rs-function-kind-label"></a>
### Function: `kind_label`

```rust
fn kind_label(kind: crate::model::SymbolKind) -> &'static str
```

[View source](../src/html.rs#L171)

<a id="symbol-src-html-rs-function-syntax-language"></a>
### Function: `syntax_language`

```rust
fn syntax_language(path: &str) -> &'static str
```

[View source](../src/html.rs#L183)

<a id="symbol-src-html-rs-function-source-link"></a>
### Function: `source_link`

```rust
fn source_link(
    span: &SourceSpan,
    project: &DocProject,
    output_parent: &Path,
    source_url: Option<&str>,
) -> String
```

[View source](../src/html.rs#L200)

<a id="symbol-src-html-rs-function-relative-path"></a>
### Function: `relative_path`

```rust
fn relative_path(from: &Path, to: &Path) -> std::path::PathBuf
```

[View source](../src/html.rs#L221)

<a id="symbol-src-html-rs-function-encode-path"></a>
### Function: `encode_path`

```rust
fn encode_path(path: &Path) -> String
```

[View source](../src/html.rs#L241)

<a id="file-src-json-rs"></a>
## `src/json.rs`

JSON renderer for the language-neutral documentation model.

<a id="symbol-src-json-rs-function-render"></a>
### Function: `render`

```rust
pub fn render(project: &DocProject) -> Result<String>
```

Serialize a project model as pretty-printed JSON.

[View source](../src/json.rs#L8)

<a id="file-src-lib-rs"></a>
## `src/lib.rs`

Core library for DocWeave Engine.

The first milestone validates the Tree-sitter parsing pipeline for Rust.
Documentation extraction and language-neutral rendering will be layered on
top of this API in subsequent milestones.

<a id="symbol-src-lib-rs-module-adapters"></a>
### Module: `adapters`

```rust
pub mod adapters;
```

[View source](../src/lib.rs#L13)

<a id="symbol-src-lib-rs-module-config"></a>
### Module: `config`

```rust
pub mod config;
```

[View source](../src/lib.rs#L14)

<a id="symbol-src-lib-rs-module-html"></a>
### Module: `html`

```rust
pub mod html;
```

[View source](../src/lib.rs#L15)

<a id="symbol-src-lib-rs-module-json"></a>
### Module: `json`

```rust
pub mod json;
```

[View source](../src/lib.rs#L16)

<a id="symbol-src-lib-rs-module-markdown"></a>
### Module: `markdown`

```rust
pub mod markdown;
```

[View source](../src/lib.rs#L17)

<a id="symbol-src-lib-rs-module-model"></a>
### Module: `model`

```rust
pub mod model;
```

[View source](../src/lib.rs#L18)

<a id="symbol-src-lib-rs-module-rust-extract"></a>
### Module: `rust_extract`

```rust
mod rust_extract;
```

[View source](../src/lib.rs#L19)

<a id="symbol-src-lib-rs-struct-parsedfile"></a>
### Struct: `ParsedFile`

```rust
pub struct ParsedFile
```

A source file discovered and parsed by DocWeave.

[View source](../src/lib.rs#L27)

<a id="symbol-src-lib-rs-function-scan-rust"></a>
### Function: `scan_rust`

```rust
pub fn scan_rust(root: &Path) -> Result<Vec<ParsedFile>>
```

Recursively discover Rust files and parse them with Tree-sitter.

Directories commonly containing generated files or dependencies are skipped.
Individual unreadable or malformed files return an error with path context.

[View source](../src/lib.rs#L39)

<a id="symbol-src-lib-rs-function-scan-project"></a>
### Function: `scan_project`

```rust
pub fn scan_project(root: &Path) -> Result<Vec<ParsedFile>>
```

Recursively discover supported C, C++, Python, Lua, and Rust files and parse
each file with its registered Tree-sitter adapter.

[View source](../src/lib.rs#L45)

<a id="symbol-src-lib-rs-function-scan-sources"></a>
### Function: `scan_sources`

```rust
fn scan_sources(root: &Path, only_adapter: Option<&str>) -> Result<Vec<ParsedFile>>
```

[View source](../src/lib.rs#L49)

<a id="symbol-src-lib-rs-function-document-rust"></a>
### Function: `document_rust`

```rust
pub fn document_rust(root: &Path) -> Result<DocProject>
```

Scan a Rust project and extract its language-neutral documentation model.

[View source](../src/lib.rs#L106)

<a id="symbol-src-lib-rs-function-document-project"></a>
### Function: `document_project`

```rust
pub fn document_project(root: &Path) -> Result<DocProject>
```

Scan all supported source languages and extract the shared documentation model.

[View source](../src/lib.rs#L125)

<a id="symbol-src-lib-rs-function-generate-documentation"></a>
### Function: `generate_documentation`

```rust
pub fn generate_documentation(config: &config::Config) -> Result<PathBuf>
```

Generate documentation in the format selected by the output extension.

[View source](../src/lib.rs#L145)

<a id="symbol-src-lib-rs-function-generate-markdown"></a>
### Function: `generate_markdown`

```rust
pub fn generate_markdown(config: &config::Config) -> Result<PathBuf>
```

Generate Markdown documentation using validated configuration.

[View source](../src/lib.rs#L186)

<a id="symbol-src-lib-rs-function-syntax-issue-count"></a>
### Function: `syntax_issue_count`

```rust
pub fn syntax_issue_count(tree: &Tree) -> usize
```

Count syntax error and missing nodes in a parsed syntax tree.

[View source](../src/lib.rs#L198)

<a id="symbol-src-lib-rs-module-tests"></a>
### Module: `tests`

```rust
mod tests
```

[View source](../src/lib.rs#L218)

<a id="symbol-src-lib-rs-function-scans-and-parses-rust-files"></a>
### Function: `scans_and_parses_rust_files`

```rust
fn scans_and_parses_rust_files()
```

[View source](../src/lib.rs#L224)

<a id="symbol-src-lib-rs-function-detects-invalid-rust-syntax"></a>
### Function: `detects_invalid_rust_syntax`

```rust
fn detects_invalid_rust_syntax()
```

[View source](../src/lib.rs#L243)

<a id="symbol-src-lib-rs-function-builds-a-project-documentation-model"></a>
### Function: `builds_a_project_documentation_model`

```rust
fn builds_a_project_documentation_model()
```

[View source](../src/lib.rs#L256)

<a id="file-src-main-rs"></a>
## `src/main.rs`

<a id="symbol-src-main-rs-struct-cli"></a>
### Struct: `Cli`

```rust
struct Cli
```

[View source](../src/main.rs#L13)

<a id="symbol-src-main-rs-enum-command"></a>
### Enum: `Command`

```rust
enum Command
```

[View source](../src/main.rs#L19)

<a id="symbol-src-main-rs-function-main"></a>
### Function: `main`

```rust
fn main() -> Result<()>
```

[View source](../src/main.rs#L37)

<a id="file-src-markdown-rs"></a>
## `src/markdown.rs`

Deterministic Markdown renderer for the language-neutral documentation model.

<a id="symbol-src-markdown-rs-function-render"></a>
### Function: `render`

```rust
pub fn render(
    project: &DocProject,
    title: &str,
    output_parent: &Path,
    source_url: Option<&str>,
) -> String
```

Render a project as Markdown, linking declarations to local source or a source URL.

[View source](../src/markdown.rs#L8)

<a id="symbol-src-markdown-rs-struct-anchors"></a>
### Struct: `Anchors`

```rust
struct Anchors
```

[View source](../src/markdown.rs#L97)

<a id="symbol-src-markdown-rs-method-new"></a>
### Method: `new`

```rust
fn new(project: &DocProject) -> Self
```

[View source](../src/markdown.rs#L103)

<a id="symbol-src-markdown-rs-function-unique-anchor"></a>
### Function: `unique_anchor`

```rust
fn unique_anchor(used: &mut std::collections::HashMap<String, usize>, base: String) -> String
```

[View source](../src/markdown.rs#L136)

<a id="symbol-src-markdown-rs-function-slug"></a>
### Function: `slug`

```rust
fn slug(value: &str) -> String
```

[View source](../src/markdown.rs#L146)

<a id="symbol-src-markdown-rs-function-kind-label"></a>
### Function: `kind_label`

```rust
fn kind_label(kind: crate::model::SymbolKind) -> &'static str
```

[View source](../src/markdown.rs#L168)

<a id="symbol-src-markdown-rs-function-source-link"></a>
### Function: `source_link`

```rust
fn source_link(
    span: &SourceSpan,
    project: &DocProject,
    output_parent: &Path,
    source_url: Option<&str>,
) -> String
```

[View source](../src/markdown.rs#L180)

<a id="symbol-src-markdown-rs-function-project-relative-path"></a>
### Function: `project_relative_path`

```rust
fn project_relative_path(project: &DocProject, path: &str) -> PathBuf
```

[View source](../src/markdown.rs#L206)

<a id="symbol-src-markdown-rs-function-relative-path"></a>
### Function: `relative_path`

```rust
fn relative_path(from: &Path, to: &Path) -> PathBuf
```

[View source](../src/markdown.rs#L217)

<a id="symbol-src-markdown-rs-function-encode-path"></a>
### Function: `encode_path`

```rust
fn encode_path(path: &Path) -> String
```

[View source](../src/markdown.rs#L243)

<a id="symbol-src-markdown-rs-function-encode-url-base"></a>
### Function: `encode_url_base`

```rust
fn encode_url_base(base: &str) -> String
```

[View source](../src/markdown.rs#L257)

<a id="symbol-src-markdown-rs-function-code-span"></a>
### Function: `code_span`

```rust
fn code_span(value: &str) -> String
```

[View source](../src/markdown.rs#L269)

<a id="symbol-src-markdown-rs-function-fenced-code"></a>
### Function: `fenced_code`

```rust
fn fenced_code(value: &str, language: &str) -> String
```

[View source](../src/markdown.rs#L279)

<a id="symbol-src-markdown-rs-function-syntax-language"></a>
### Function: `syntax_language`

```rust
fn syntax_language(path: &str) -> &'static str
```

[View source](../src/markdown.rs#L285)

<a id="symbol-src-markdown-rs-function-max-backtick-run"></a>
### Function: `max_backtick_run`

```rust
fn max_backtick_run(value: &str) -> usize
```

[View source](../src/markdown.rs#L302)

<a id="symbol-src-markdown-rs-function-escape-heading"></a>
### Function: `escape_heading`

```rust
fn escape_heading(value: &str) -> String
```

[View source](../src/markdown.rs#L316)

<a id="symbol-src-markdown-rs-module-tests"></a>
### Module: `tests`

```rust
mod tests
```

[View source](../src/markdown.rs#L328)

<a id="symbol-src-markdown-rs-function-fixture"></a>
### Function: `fixture`

```rust
fn fixture() -> DocProject
```

[View source](../src/markdown.rs#L332)

<a id="symbol-src-markdown-rs-function-renders-deterministic-anchors-and-local-source-links"></a>
### Function: `renders_deterministic_anchors_and_local_source_links`

```rust
fn renders_deterministic_anchors_and_local_source_links()
```

[View source](../src/markdown.rs#L362)

<a id="symbol-src-markdown-rs-function-remote-links-encode-source-paths-and-duplicate-anchors-are-unique"></a>
### Function: `remote_links_encode_source_paths_and_duplicate_anchors_are_unique`

```rust
fn remote_links_encode_source_paths_and_duplicate_anchors_are_unique()
```

[View source](../src/markdown.rs#L373)

<a id="symbol-src-markdown-rs-function-inline-code-delimiters-do-not-break-on-backticks-in-names"></a>
### Function: `inline_code_delimiters_do_not_break_on_backticks_in_names`

```rust
fn inline_code_delimiters_do_not_break_on_backticks_in_names()
```

[View source](../src/markdown.rs#L390)

<a id="file-src-model-rs"></a>
## `src/model.rs`

Language-neutral data structures produced by source adapters.

<a id="symbol-src-model-rs-struct-docproject"></a>
### Struct: `DocProject`

```rust
pub struct DocProject
```

A complete documentation model for one scanned project.

[View source](../src/model.rs#L7)

<a id="symbol-src-model-rs-struct-docfile"></a>
### Struct: `DocFile`

```rust
pub struct DocFile
```

Documentation and diagnostics extracted from one source file.

[View source](../src/model.rs#L14)

<a id="symbol-src-model-rs-struct-docsymbol"></a>
### Struct: `DocSymbol`

```rust
pub struct DocSymbol
```

A documented declaration found in source code.

[View source](../src/model.rs#L23)

<a id="symbol-src-model-rs-enum-symbolkind"></a>
### Enum: `SymbolKind`

```rust
pub enum SymbolKind
```

Shared declaration kinds used by the built-in language adapters.

[View source](../src/model.rs#L34)

<a id="symbol-src-model-rs-struct-sourcespan"></a>
### Struct: `SourceSpan`

```rust
pub struct SourceSpan
```

A one-based source range. End positions are exclusive, matching Tree-sitter.

[View source](../src/model.rs#L46)

<a id="symbol-src-model-rs-struct-diagnostic"></a>
### Struct: `Diagnostic`

```rust
pub struct Diagnostic
```

A stable, source-located issue found while parsing or extracting.

[View source](../src/model.rs#L56)

<a id="symbol-src-model-rs-enum-diagnosticseverity"></a>
### Enum: `DiagnosticSeverity`

```rust
pub enum DiagnosticSeverity
```

Diagnostic importance.

[View source](../src/model.rs#L66)

<a id="file-src-rust-extract-rs"></a>
## `src/rust_extract.rs`

Rust Tree-sitter adapter that maps syntax trees to the shared documentation model.

<a id="symbol-src-rust-extract-rs-function-extract-rust"></a>
### Function: `extract_rust`

```rust
pub fn extract_rust(path: &Path, source: &str, tree: &Tree) -> DocFile
```

Extract Rust declarations, explicit doc comments, and syntax diagnostics.

[View source](../src/rust_extract.rs#L10)

<a id="symbol-src-rust-extract-rs-function-collect-items"></a>
### Function: `collect_items`

```rust
fn collect_items(
    parent: Node<'_>,
    parent_kind: Option<&str>,
    source: &str,
    path: &str,
    symbols: &mut Vec<DocSymbol>,
)
```

[View source](../src/rust_extract.rs#L27)

<a id="symbol-src-rust-extract-rs-function-symbol-kind"></a>
### Function: `symbol_kind`

```rust
fn symbol_kind(kind: &str, parent_kind: Option<&str>) -> Option<SymbolKind>
```

[View source](../src/rust_extract.rs#L76)

<a id="symbol-src-rust-extract-rs-function-leading-docs"></a>
### Function: `leading_docs`

```rust
fn leading_docs(children: &[Node<'_>], item_index: usize, source: &str) -> Option<String>
```

[View source](../src/rust_extract.rs#L92)

<a id="symbol-src-rust-extract-rs-function-inner-docs"></a>
### Function: `inner_docs`

```rust
fn inner_docs(node: Node<'_>, source: &str) -> Option<String>
```

[View source](../src/rust_extract.rs#L112)

<a id="symbol-src-rust-extract-rs-function-signature"></a>
### Function: `signature`

```rust
fn signature(node: Node<'_>, source: &str) -> Option<String>
```

[View source](../src/rust_extract.rs#L129)

<a id="symbol-src-rust-extract-rs-function-span"></a>
### Function: `span`

```rust
fn span(node: Node<'_>, path: &str) -> SourceSpan
```

[View source](../src/rust_extract.rs#L146)

<a id="symbol-src-rust-extract-rs-function-collect-diagnostics"></a>
### Function: `collect_diagnostics`

```rust
fn collect_diagnostics(node: Node<'_>, path: &str, diagnostics: &mut Vec<Diagnostic>)
```

[View source](../src/rust_extract.rs#L158)

<a id="symbol-src-rust-extract-rs-module-tests"></a>
### Module: `tests`

```rust
mod tests
```

[View source](../src/rust_extract.rs#L182)

<a id="symbol-src-rust-extract-rs-function-parse"></a>
### Function: `parse`

```rust
fn parse(source: &str) -> Tree
```

[View source](../src/rust_extract.rs#L186)

<a id="symbol-src-rust-extract-rs-function-extracts-nested-items-generics-attributes-and-docs"></a>
### Function: `extracts_nested_items_generics_attributes_and_docs`

```rust
fn extracts_nested_items_generics_attributes_and_docs()
```

[View source](../src/rust_extract.rs#L195)

<a id="symbol-src-rust-extract-rs-function-malformed-syntax-becomes-a-diagnostic"></a>
### Function: `malformed_syntax_becomes_a_diagnostic`

```rust
fn malformed_syntax_becomes_a_diagnostic()
```

[View source](../src/rust_extract.rs#L270)

<a id="symbol-src-rust-extract-rs-function-fixture-covers-nested-generics-attributes-and-doc-comments"></a>
### Function: `fixture_covers_nested_generics_attributes_and_doc_comments`

```rust
fn fixture_covers_nested_generics_attributes_and_doc_comments()
```

[View source](../src/rust_extract.rs#L280)

<a id="file-tests-fixtures-malformed-rs"></a>
## `tests/fixtures/malformed.rs`

### Diagnostics

- `rust.syntax_error` — Tree-sitter found invalid Rust syntax. ([line 2](../tests/fixtures/malformed.rs#L2))

<a id="file-tests-fixtures-nested-generics-rs"></a>
## `tests/fixtures/nested_generics.rs`

Fixture for file-level documentation.

<a id="symbol-tests-fixtures-nested-generics-rs-module-outer"></a>
### Module: `outer`

```rust
pub mod outer
```

Outer module docs.
Inline module documentation.

[View source](../tests/fixtures/nested_generics.rs#L5)

<a id="symbol-tests-fixtures-nested-generics-rs-struct-record"></a>
### Struct: `Record`

```rust
pub struct Record<T>
```

Generic struct docs.

[View source](../tests/fixtures/nested_generics.rs#L10)

<a id="symbol-tests-fixtures-nested-generics-rs-trait-convert"></a>
### Trait: `Convert`

```rust
pub trait Convert<T>
```

Generic trait docs.

[View source](../tests/fixtures/nested_generics.rs#L15)

<a id="symbol-tests-fixtures-nested-generics-rs-method-convert"></a>
### Method: `convert`

```rust
fn convert(&self) -> T;
```

[View source](../tests/fixtures/nested_generics.rs#L16)

<a id="symbol-tests-fixtures-nested-generics-rs-method-value"></a>
### Method: `value`

```rust
pub fn value(&self) -> &T
```

Method docs.

[View source](../tests/fixtures/nested_generics.rs#L21)

<a id="symbol-tests-fixtures-nested-generics-rs-enum-choice"></a>
### Enum: `Choice`

```rust
pub enum Choice<T>
```

Generic enum docs.

[View source](../tests/fixtures/nested_generics.rs#L27)

<a id="symbol-tests-fixtures-nested-generics-rs-constant-answer"></a>
### Constant: `ANSWER`

```rust
pub const ANSWER: usize = 42;
```

Constant docs.

[View source](../tests/fixtures/nested_generics.rs#L33)

<a id="symbol-tests-fixtures-nested-generics-rs-function-make"></a>
### Function: `make`

```rust
pub fn make<T>(value: T) -> Record<T>
```

Generic function docs.

[View source](../tests/fixtures/nested_generics.rs#L36)

<a id="file-tests-generate-markdown-rs"></a>
## `tests/generate_markdown.rs`

<a id="symbol-tests-generate-markdown-rs-function-cli-generates-markdown-with-anchors-docs-and-source-links"></a>
### Function: `cli_generates_markdown_with_anchors_docs_and_source_links`

```rust
fn cli_generates_markdown_with_anchors_docs_and_source_links()
```

[View source](../tests/generate_markdown.rs#L4)

<a id="symbol-tests-generate-markdown-rs-function-cli-rejects-invalid-configuration-without-creating-output"></a>
### Function: `cli_rejects_invalid_configuration_without_creating_output`

```rust
fn cli_rejects_invalid_configuration_without_creating_output()
```

[View source](../tests/generate_markdown.rs#L41)
