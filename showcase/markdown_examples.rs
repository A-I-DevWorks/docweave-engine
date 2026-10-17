//! Exemplos de construções Markdown preservadas nos comentários de documentação.

/// Demonstra títulos, ênfase, listas, links, citações, tabelas e blocos de código.
///
/// ## Formatação inline
///
/// Use **negrito**, *itálico*, ~~tachado~~ e `código` em uma frase. Links também
/// funcionam, como [a documentação do Rust](https://doc.rust-lang.org/).
///
/// > Uma citação pode destacar uma observação importante.
/// >
/// > Ela pode ocupar mais de uma linha.
///
/// ## Listas
///
/// 1. Primeiro, descreva a preparação.
/// 2. Depois, execute os passos:
///    - valide os dados;
///    - gere a saída.
///
/// ## Tabela
///
/// | Entrada | Resultado |
/// |:--|--:|
/// | válida | sucesso |
/// | inválida | erro |
///
/// ## Bloco de código
///
/// ```rust
/// let title = "Guia";
/// assert_eq!(title.len(), 4);
/// ```
///
/// Abaixo há um separador horizontal:
///
/// ---
pub fn markdown_features_example() -> &'static str {
    "Consulte a documentação gerada para ver os exemplos de Markdown."
}
