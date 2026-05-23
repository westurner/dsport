use clap::Args;

#[derive(Args, Debug, Clone, Default)]
pub struct CommonOptions {
    #[arg(long = "character-level-inline-markup", num_args = 0..=1)]
    pub character_level_inline_markup: Option<String>,

    #[arg(long = "config", num_args = 0..=1)]
    pub config: Option<String>,

    #[arg(long = "date", num_args = 0..=1)]
    pub date: Option<String>,

    #[arg(long = "debug", num_args = 0..=1)]
    pub debug: Option<String>,

    #[arg(long = "error-encoding", num_args = 0..=1)]
    pub error_encoding: Option<String>,

    #[arg(long = "exit-status", num_args = 0..=1)]
    pub exit_status: Option<String>,

    #[arg(long = "file-insertion-enabled", num_args = 0..=1)]
    pub file_insertion_enabled: Option<String>,

    #[arg(long = "footnote-backlinks", num_args = 0..=1)]
    pub footnote_backlinks: Option<String>,

    #[arg(long = "generator", num_args = 0..=1)]
    pub generator: Option<String>,

    #[arg(long = "halt", num_args = 0..=1)]
    pub halt: Option<String>,

    #[arg(long = "input-encoding", num_args = 0..=1)]
    pub input_encoding: Option<String>,

    #[arg(long = "language", num_args = 0..=1)]
    pub language: Option<String>,

    #[arg(long = "leave-comments", num_args = 0..=1)]
    pub leave_comments: Option<String>,

    #[arg(long = "leave-footnote-reference-space", num_args = 0..=1)]
    pub leave_footnote_reference_space: Option<String>,

    #[arg(long = "legacy-ids", num_args = 0..=1)]
    pub legacy_ids: Option<String>,

    #[arg(long = "line-length-limit", num_args = 0..=1)]
    pub line_length_limit: Option<String>,

    #[arg(long = "matching-ids", num_args = 0..=1)]
    pub matching_ids: Option<String>,

    #[arg(long = "no-datestamp", num_args = 0..=1)]
    pub no_datestamp: Option<String>,

    #[arg(long = "no-debug", num_args = 0..=1)]
    pub no_debug: Option<String>,

    #[arg(long = "no-doc-info", num_args = 0..=1)]
    pub no_doc_info: Option<String>,

    #[arg(long = "no-doc-title", num_args = 0..=1)]
    pub no_doc_title: Option<String>,

    #[arg(long = "no-file-insertion", num_args = 0..=1)]
    pub no_file_insertion: Option<String>,

    #[arg(long = "no-footnote-backlinks", num_args = 0..=1)]
    pub no_footnote_backlinks: Option<String>,

    #[arg(long = "no-generator", num_args = 0..=1)]
    pub no_generator: Option<String>,

    #[arg(long = "no-raw", num_args = 0..=1)]
    pub no_raw: Option<String>,

    #[arg(long = "no-section-numbering", num_args = 0..=1)]
    pub no_section_numbering: Option<String>,

    #[arg(long = "no-section-subtitles", num_args = 0..=1)]
    pub no_section_subtitles: Option<String>,

    #[arg(long = "no-source-link", num_args = 0..=1)]
    pub no_source_link: Option<String>,

    #[arg(long = "no-toc-backlinks", num_args = 0..=1)]
    pub no_toc_backlinks: Option<String>,

    #[arg(long = "no-traceback", num_args = 0..=1)]
    pub no_traceback: Option<String>,

    #[arg(long = "no-validation", num_args = 0..=1)]
    pub no_validation: Option<String>,

    #[arg(long = "output", num_args = 0..=1)]
    pub output: Option<String>,

    #[arg(long = "output-encoding", num_args = 0..=1)]
    pub output_encoding: Option<String>,

    #[arg(long = "output-path", num_args = 0..=1)]
    pub output_path: Option<String>,

    #[arg(long = "pep-base-url", num_args = 0..=1)]
    pub pep_base_url: Option<String>,

    #[arg(long = "pep-file-url-template", num_args = 0..=1)]
    pub pep_file_url_template: Option<String>,

    #[arg(long = "pep-references", num_args = 0..=1)]
    pub pep_references: Option<String>,

    #[arg(long = "raw-enabled", num_args = 0..=1)]
    pub raw_enabled: Option<String>,

    #[arg(long = "record-dependencies", num_args = 0..=1)]
    pub record_dependencies: Option<String>,

    #[arg(long = "report", num_args = 0..=1)]
    pub report: Option<String>,

    #[arg(long = "rfc-base-url", num_args = 0..=1)]
    pub rfc_base_url: Option<String>,

    #[arg(long = "rfc-references", num_args = 0..=1)]
    pub rfc_references: Option<String>,

    #[arg(long = "root-prefix", num_args = 0..=1)]
    pub root_prefix: Option<String>,

    #[arg(long = "section-numbering", num_args = 0..=1)]
    pub section_numbering: Option<String>,

    #[arg(long = "section-subtitles", num_args = 0..=1)]
    pub section_subtitles: Option<String>,

    #[arg(long = "smart-quotes", num_args = 0..=1)]
    pub smart_quotes: Option<String>,

    #[arg(long = "smartquotes-locales", num_args = 0..=1)]
    pub smartquotes_locales: Option<String>,

    #[arg(long = "source-link", num_args = 0..=1)]
    pub source_link: Option<String>,

    #[arg(long = "source-url", num_args = 0..=1)]
    pub source_url: Option<String>,

    #[arg(long = "strict", num_args = 0..=1)]
    pub strict: Option<String>,

    #[arg(long = "strip-class", num_args = 0..=1)]
    pub strip_class: Option<String>,

    #[arg(long = "strip-comments", num_args = 0..=1)]
    pub strip_comments: Option<String>,

    #[arg(long = "strip-elements-with-class", num_args = 0..=1)]
    pub strip_elements_with_class: Option<String>,

    #[arg(long = "syntax-highlight", num_args = 0..=1)]
    pub syntax_highlight: Option<String>,

    #[arg(long = "tab-width", num_args = 0..=1)]
    pub tab_width: Option<String>,

    #[arg(long = "time", num_args = 0..=1)]
    pub time: Option<String>,

    #[arg(long = "title", num_args = 0..=1)]
    pub title: Option<String>,

    #[arg(long = "toc-entry-backlinks", num_args = 0..=1)]
    pub toc_entry_backlinks: Option<String>,

    #[arg(long = "toc-top-backlinks", num_args = 0..=1)]
    pub toc_top_backlinks: Option<String>,

    #[arg(long = "traceback", num_args = 0..=1)]
    pub traceback: Option<String>,

    #[arg(long = "trim-footnote-reference-space", num_args = 0..=1)]
    pub trim_footnote_reference_space: Option<String>,

    #[arg(long = "validate", num_args = 0..=1)]
    pub validate: Option<String>,

    #[arg(long = "version", num_args = 0..=1)]
    pub version: Option<String>,

    #[arg(long = "warnings", num_args = 0..=1)]
    pub warnings: Option<String>,

    #[arg(long = "word-level-inline-markup", num_args = 0..=1)]
    pub word_level_inline_markup: Option<String>,

    #[arg(short = 'V', num_args = 0..=1)]
    pub version_flag: Option<String>,

    #[arg(short = 'd', num_args = 0..=1)]
    pub d: Option<String>,

    #[arg(short = 'e', num_args = 0..=1)]
    pub e: Option<String>,

    #[arg(short = 'g', num_args = 0..=1)]
    pub g: Option<String>,

    #[arg(short = 'l', num_args = 0..=1)]
    pub l: Option<String>,

    #[arg(short = 'r', num_args = 0..=1)]
    pub r: Option<String>,

    #[arg(short = 's', num_args = 0..=1)]
    pub s: Option<String>,

    #[arg(short = 't', num_args = 0..=1)]
    pub t: Option<String>,
}

#[derive(Args, Debug, Clone, Default)]
pub struct LatexOptions {
    #[arg(long = "attribution", num_args = 0..=1)]
    pub attribution: Option<String>,

    #[arg(long = "compound-enumerators", num_args = 0..=1)]
    pub compound_enumerators: Option<String>,

    #[arg(long = "documentclass", num_args = 0..=1)]
    pub documentclass: Option<String>,

    #[arg(long = "documentoptions", num_args = 0..=1)]
    pub documentoptions: Option<String>,

    #[arg(long = "docutils-footnotes", num_args = 0..=1)]
    pub docutils_footnotes: Option<String>,

    #[arg(long = "embed-stylesheet", num_args = 0..=1)]
    pub embed_stylesheet: Option<String>,

    #[arg(long = "figure-citations", num_args = 0..=1)]
    pub figure_citations: Option<String>,

    #[arg(long = "font-encoding", num_args = 0..=1)]
    pub font_encoding: Option<String>,

    #[arg(long = "footnote-references", num_args = 0..=1)]
    pub footnote_references: Option<String>,

    #[arg(long = "graphicx-option", num_args = 0..=1)]
    pub graphicx_option: Option<String>,

    #[arg(long = "hyperlink-color", num_args = 0..=1)]
    pub hyperlink_color: Option<String>,

    #[arg(long = "hyperref-options", num_args = 0..=1)]
    pub hyperref_options: Option<String>,

    #[arg(long = "latex-footnotes", num_args = 0..=1)]
    pub latex_footnotes: Option<String>,

    #[arg(long = "latex-preamble", num_args = 0..=1)]
    pub latex_preamble: Option<String>,

    #[arg(long = "legacy-class-functions", num_args = 0..=1)]
    pub legacy_class_functions: Option<String>,

    #[arg(long = "legacy-column-widths", num_args = 0..=1)]
    pub legacy_column_widths: Option<String>,

    #[arg(long = "link-stylesheet", num_args = 0..=1)]
    pub link_stylesheet: Option<String>,

    #[arg(long = "literal-block-env", num_args = 0..=1)]
    pub literal_block_env: Option<String>,

    #[arg(long = "new-class-functions", num_args = 0..=1)]
    pub new_class_functions: Option<String>,

    #[arg(long = "new-column-widths", num_args = 0..=1)]
    pub new_column_widths: Option<String>,

    #[arg(long = "no-compound-enumerators", num_args = 0..=1)]
    pub no_compound_enumerators: Option<String>,

    #[arg(long = "no-section-prefix-for-enumerators", num_args = 0..=1)]
    pub no_section_prefix_for_enumerators: Option<String>,

    #[arg(long = "reference-label", num_args = 0..=1)]
    pub reference_label: Option<String>,

    #[arg(long = "section-enumerator-separator", num_args = 0..=1)]
    pub section_enumerator_separator: Option<String>,

    #[arg(long = "section-prefix-for-enumerators", num_args = 0..=1)]
    pub section_prefix_for_enumerators: Option<String>,

    #[arg(long = "stylesheet", num_args = 0..=1)]
    pub stylesheet: Option<String>,

    #[arg(long = "stylesheet-dirs", num_args = 0..=1)]
    pub stylesheet_dirs: Option<String>,

    #[arg(long = "stylesheet-path", num_args = 0..=1)]
    pub stylesheet_path: Option<String>,

    #[arg(long = "table-style", num_args = 0..=1)]
    pub table_style: Option<String>,

    #[arg(long = "template", num_args = 0..=1)]
    pub template: Option<String>,

    #[arg(long = "topic-abstract", num_args = 0..=1)]
    pub topic_abstract: Option<String>,

    #[arg(long = "use-bibtex", num_args = 0..=1)]
    pub use_bibtex: Option<String>,

    #[arg(long = "use-docutils-docinfo", num_args = 0..=1)]
    pub use_docutils_docinfo: Option<String>,

    #[arg(long = "use-docutils-toc", num_args = 0..=1)]
    pub use_docutils_toc: Option<String>,

    #[arg(long = "use-latex-abstract", num_args = 0..=1)]
    pub use_latex_abstract: Option<String>,

    #[arg(long = "use-latex-citations", num_args = 0..=1)]
    pub use_latex_citations: Option<String>,

    #[arg(long = "use-latex-docinfo", num_args = 0..=1)]
    pub use_latex_docinfo: Option<String>,

    #[arg(long = "use-latex-toc", num_args = 0..=1)]
    pub use_latex_toc: Option<String>,

    #[arg(long = "use-part-section", num_args = 0..=1)]
    pub use_part_section: Option<String>,
}

#[derive(Args, Debug, Clone, Default)]
pub struct ManOptions {
    #[arg(long = "macro-references", num_args = 0..=1)]
    pub macro_references: Option<String>,

    #[arg(long = "text-references", num_args = 0..=1)]
    pub text_references: Option<String>,
}

#[derive(Args, Debug, Clone, Default)]
pub struct Html5Options {
    #[arg(long = "attribution", num_args = 0..=1)]
    pub attribution: Option<String>,

    #[arg(long = "cloak-email-addresses", num_args = 0..=1)]
    pub cloak_email_addresses: Option<String>,

    #[arg(long = "compact-field-lists", num_args = 0..=1)]
    pub compact_field_lists: Option<String>,

    #[arg(long = "compact-lists", num_args = 0..=1)]
    pub compact_lists: Option<String>,

    #[arg(long = "embed-stylesheet", num_args = 0..=1)]
    pub embed_stylesheet: Option<String>,

    #[arg(long = "footnote-references", num_args = 0..=1)]
    pub footnote_references: Option<String>,

    #[arg(long = "image-loading", num_args = 0..=1)]
    pub image_loading: Option<String>,

    #[arg(long = "initial-header-level", num_args = 0..=1)]
    pub initial_header_level: Option<String>,

    #[arg(long = "link-stylesheet", num_args = 0..=1)]
    pub link_stylesheet: Option<String>,

    #[arg(long = "math-output", num_args = 0..=1)]
    pub math_output: Option<String>,

    #[arg(long = "no-compact-field-lists", num_args = 0..=1)]
    pub no_compact_field_lists: Option<String>,

    #[arg(long = "no-compact-lists", num_args = 0..=1)]
    pub no_compact_lists: Option<String>,

    #[arg(long = "no-section-self-link", num_args = 0..=1)]
    pub no_section_self_link: Option<String>,

    #[arg(long = "no-xml-declaration", num_args = 0..=1)]
    pub no_xml_declaration: Option<String>,

    #[arg(long = "section-self-link", num_args = 0..=1)]
    pub section_self_link: Option<String>,

    #[arg(long = "stylesheet", num_args = 0..=1)]
    pub stylesheet: Option<String>,

    #[arg(long = "stylesheet-dirs", num_args = 0..=1)]
    pub stylesheet_dirs: Option<String>,

    #[arg(long = "stylesheet-path", num_args = 0..=1)]
    pub stylesheet_path: Option<String>,

    #[arg(long = "table-style", num_args = 0..=1)]
    pub table_style: Option<String>,

    #[arg(long = "template", num_args = 0..=1)]
    pub template: Option<String>,

    #[arg(long = "xml-declaration", num_args = 0..=1)]
    pub xml_declaration: Option<String>,
}

#[derive(Args, Debug, Clone, Default)]
pub struct PseudoXmlOptions {
    #[arg(long = "detailed", num_args = 0..=1)]
    pub detailed: Option<String>,
}

#[derive(Args, Debug, Clone, Default)]
pub struct OdtOptions {
    #[arg(long = "add-syntax-highlighting", num_args = 0..=1)]
    pub add_syntax_highlighting: Option<String>,

    #[arg(long = "cloak-email-addresses", num_args = 0..=1)]
    pub cloak_email_addresses: Option<String>,

    #[arg(long = "create-links", num_args = 0..=1)]
    pub create_links: Option<String>,

    #[arg(long = "create-sections", num_args = 0..=1)]
    pub create_sections: Option<String>,

    #[arg(long = "custom-odt-footer", num_args = 0..=1)]
    pub custom_odt_footer: Option<String>,

    #[arg(long = "custom-odt-header", num_args = 0..=1)]
    pub custom_odt_header: Option<String>,

    #[arg(long = "endnotes-end-doc", num_args = 0..=1)]
    pub endnotes_end_doc: Option<String>,

    #[arg(long = "generate-list-toc", num_args = 0..=1)]
    pub generate_list_toc: Option<String>,

    #[arg(long = "generate-oowriter-toc", num_args = 0..=1)]
    pub generate_oowriter_toc: Option<String>,

    #[arg(long = "no-cloak-email-addresses", num_args = 0..=1)]
    pub no_cloak_email_addresses: Option<String>,

    #[arg(long = "no-endnotes-end-doc", num_args = 0..=1)]
    pub no_endnotes_end_doc: Option<String>,

    #[arg(long = "no-links", num_args = 0..=1)]
    pub no_links: Option<String>,

    #[arg(long = "no-sections", num_args = 0..=1)]
    pub no_sections: Option<String>,

    #[arg(long = "no-syntax-highlighting", num_args = 0..=1)]
    pub no_syntax_highlighting: Option<String>,

    #[arg(long = "odf-config-file", num_args = 0..=1)]
    pub odf_config_file: Option<String>,

    #[arg(long = "stylesheet", num_args = 0..=1)]
    pub stylesheet: Option<String>,

    #[arg(long = "table-border-thickness", num_args = 0..=1)]
    pub table_border_thickness: Option<String>,
}
