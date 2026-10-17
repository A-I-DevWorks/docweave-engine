# PDF output evaluation

## Decision

PDF generation was evaluated after the shared model and Markdown, JSON, and
HTML renderers were in place. **DocWeave will not add a PDF renderer in
Milestone 4.** Keep the current HTML renderer as the canonical presentation
layer and prototype PDF as an optional HTML-to-PDF export in a later milestone.

This reuses the existing documentation layout, Markdown conversion, escaping,
and source links instead of creating a second document layout engine. Before a
PDF feature is scheduled, settle page size, print styles, code wrapping, table
pagination, link behavior, accessibility, and where the required browser
runtime comes from.

## Approaches considered

### Print the generated HTML with Chromium

The current HTML is a single local document with embedded CSS, so a headless
browser can print it without a server or remote requests. The Rust
`headless_chrome` crate exposes Chromium's `print_to_pdf` method and can use a
system browser; its documentation also describes downloading a compatible
browser binary. See the [`headless_chrome` overview](https://docs.rs/headless_chrome/latest/headless_chrome/),
[`Tab::print_to_pdf`](https://docs.rs/headless_chrome/latest/headless_chrome/browser/tab/struct.Tab.html#method.print_to_pdf),
and [`Browser`](https://docs.rs/headless_chrome/latest/headless_chrome/browser/struct.Browser.html).

This is the preferred prototype because browser CSS print behavior is the
closest match to the existing HTML output. It also keeps PDF support optional
and avoids a separate PDF layout implementation. The main cost is the browser
runtime: its installation, supported versions, launch flags, and reproducible
output need a clear cross-platform policy. Ordinary Markdown, JSON, and HTML
generation must remain offline and must not download a browser implicitly.

### Generate PDF directly from Rust

[`printpdf`](https://docs.rs/printpdf/latest/printpdf/) provides PDF primitives,
text shaping, and HTML handling through a layout engine. A direct backend could
avoid launching a browser, but it would need a compatibility prototype against
DocWeave's Markdown-derived headings, tables, code blocks, and links before the
project could rely on its HTML/CSS behavior. It would also introduce a second
layout path to maintain.

## Follow-up criteria

Revisit PDF when users need a generated artifact rather than printing the HTML
themselves. The prototype should:

- render the current showcase with no network access;
- preserve headings, links, tables, and code with readable wrapping;
- handle page breaks in long code blocks and tables;
- use a predictable paper size and print stylesheet;
- explain the browser/runtime requirement and fail with an actionable error;
- remain an optional output so the existing renderers do not depend on a PDF
  toolchain.
