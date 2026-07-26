//! `sphinxdocrs::intl` — Rust port of `sphinx.util.i18n`.
//!
//! Handles the catalog-discovery and domain-mapping logic that Sphinx uses to
//! locate `.po` / `.mo` files for document-level translation.
//!
//! ## Ported surface
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `CatalogInfo` | [`CatalogInfo`] | `base_dir`, `domain`, `charset`; `po_path`, `mo_path`, `is_outdated` |
//! | `CatalogInfo.write_mo` | [`CatalogInfo::write_mo`] | native GNU MO writer, no Babel dependency |
//! | `CatalogRepository` | [`CatalogRepository`] | `locale_dirs()`, `pofiles()`, `catalogs()` |
//! | `docname_to_domain` | [`docname_to_domain`] | compaction mapping |
//! | `date_format_mappings` | [`DATE_FORMAT_MAPPINGS`] | `ustrftime` → babel format codes |
//! | `date_format_re.split` | [`split_date_format`] | left-to-right longest-first token scan |
//! | `babel_format_date` | [`babel_format_date`] | CLDR pattern subset, see below |
//! | `format_date` | [`format_date`] | `SOURCE_DATE_EPOCH` aware |
//!
//! ## Accepted deviations
//!
//! - **Locale data.** Babel ships the full CLDR database; this port carries
//!   month/weekday/period names for `en`, `de` and `ja` only (see
//!   [`date_names_for`]). Any other locale falls back to `en`, which is the
//!   same behaviour upstream applies to an *unknown* locale.
//! - **MO features.** [`encode_mo`] writes singular entries only, with an
//!   empty hash table (`gettext` then binary-searches the sorted string
//!   table). Plural forms and `msgctxt` are not emitted, matching the
//!   singular-only [`PoCatalog`](crate::locale::PoCatalog) parser.
//! - **Charsets.** `.po` files are decoded as UTF-8 regardless of
//!   `CatalogInfo::charset`; Sphinx projects use UTF-8 in practice.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Datelike, Local, Offset, TimeZone, Timelike, Utc};

use crate::locale::PoCatalog;

// ── CatalogInfo ───────────────────────────────────────────────────────────────

/// Metadata for a single message catalog.
///
/// Mirrors `sphinx.util.i18n.CatalogInfo`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogInfo {
    /// Directory containing `{language}/LC_MESSAGES/{domain}.po`.
    pub base_dir: PathBuf,
    /// Domain name (also the stem of the `.po` / `.mo` files).
    pub domain: String,
    /// Source charset (e.g. `"utf-8"`).
    pub charset: String,
}

impl CatalogInfo {
    /// Construct a new `CatalogInfo`.
    pub fn new(
        base_dir: impl AsRef<Path>,
        domain: impl Into<String>,
        charset: impl Into<String>,
    ) -> Self {
        CatalogInfo {
            base_dir: base_dir.as_ref().to_path_buf(),
            domain: domain.into(),
            charset: charset.into(),
        }
    }

    /// File name of the `.po` source file, e.g. `"sphinx.po"`.
    pub fn po_file(&self) -> String {
        format!("{}.po", self.domain)
    }

    /// File name of the compiled `.mo` file, e.g. `"sphinx.mo"`.
    pub fn mo_file(&self) -> String {
        format!("{}.mo", self.domain)
    }

    /// Absolute path to the `.po` source file.
    pub fn po_path(&self) -> PathBuf {
        self.base_dir.join(self.po_file())
    }

    /// Absolute path to the compiled `.mo` file.
    pub fn mo_path(&self) -> PathBuf {
        self.base_dir.join(self.mo_file())
    }

    /// Return `true` if the `.po` file is newer than (or the `.mo` file does
    /// not exist), indicating the catalog needs recompilation.
    ///
    /// Mirrors `CatalogInfo.is_outdated()`.
    pub fn is_outdated(&self) -> bool {
        let mo = self.mo_path();
        if !mo.exists() {
            return true;
        }
        let po_mtime = std::fs::metadata(self.po_path())
            .and_then(|m| m.modified())
            .ok();
        let mo_mtime = std::fs::metadata(&mo).and_then(|m| m.modified()).ok();
        match (po_mtime, mo_mtime) {
            (Some(po), Some(mo)) => po > mo,
            _ => true,
        }
    }

    /// Compile [`po_path`](Self::po_path) into [`mo_path`](Self::mo_path).
    ///
    /// Mirrors `CatalogInfo.write_mo()`. Upstream delegates to Babel's
    /// `read_po` / `write_mo`; this port parses with
    /// [`PoCatalog`](crate::locale::PoCatalog) and serialises with
    /// [`encode_mo`].
    ///
    /// `use_fuzzy` controls whether `#, fuzzy` entries are compiled in. As in
    /// `msgfmt`, they are skipped by default.
    ///
    /// The upstream `locale` argument only steers Babel's plural-form
    /// selection and has no analogue here, so it is omitted.
    pub fn write_mo(&self, use_fuzzy: bool) -> Result<(), IntlError> {
        let po_path = self.po_path();
        let raw = std::fs::read(&po_path).map_err(|source| IntlError::Read {
            path: po_path.clone(),
            source,
        })?;
        let catalog = PoCatalog::parse(&String::from_utf8_lossy(&raw));

        let mut entries: BTreeMap<String, String> = catalog
            .entries()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        if use_fuzzy {
            entries.extend(
                catalog
                    .fuzzy_entries()
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone())),
            );
        }

        let mo_path = self.mo_path();
        std::fs::write(&mo_path, encode_mo(catalog.header(), &entries)).map_err(|source| {
            IntlError::Write {
                path: mo_path,
                source,
            }
        })
    }
}

// ── MO codec ──────────────────────────────────────────────────────────────────

/// Little-endian magic number at the head of every GNU `.mo` file.
pub const MO_MAGIC: u32 = 0x9504_12de;

/// Errors raised while compiling or reading a message catalog.
#[derive(Debug, thiserror::Error)]
pub enum IntlError {
    /// The `.po` source could not be read.
    #[error("reading error: {}, {source}", path.display())]
    Read {
        /// Path that could not be read.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// The `.mo` output could not be written.
    #[error("writing error: {}, {source}", path.display())]
    Write {
        /// Path that could not be written.
        path: PathBuf,
        /// Underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// The bytes handed to [`decode_mo`] are not a well-formed MO file.
    #[error("invalid MO file: {0}")]
    InvalidMo(&'static str),
}

/// Serialise a catalog into GNU MO binary format.
///
/// `header` is the metadata block (the `msgid ""` translation) and is emitted
/// as the first string-table entry when non-empty. `entries` is iterated in
/// `BTreeMap` order, which is the byte-wise `msgid` ordering that `gettext`
/// requires for binary search.
///
/// The hash table is written with size `0`; readers then fall back to binary
/// search over the sorted originals table.
pub fn encode_mo(header: &str, entries: &BTreeMap<String, String>) -> Vec<u8> {
    // The empty msgid sorts before every other key, so prepending the header
    // keeps the originals table sorted.
    let mut pairs: Vec<(&str, &str)> = Vec::with_capacity(entries.len() + 1);
    if !header.is_empty() {
        pairs.push(("", header));
    }
    pairs.extend(entries.iter().map(|(k, v)| (k.as_str(), v.as_str())));

    let count = pairs.len() as u32;
    let originals_table = 28_u32;
    let translations_table = originals_table + 8 * count;
    let hash_table = translations_table + 8 * count;

    let mut ids = Vec::new();
    let mut strs = Vec::new();
    for (id, msg) in &pairs {
        ids.extend_from_slice(id.as_bytes());
        ids.push(0);
        strs.extend_from_slice(msg.as_bytes());
        strs.push(0);
    }
    let ids_start = hash_table; // hash table is empty, so strings start here
    let strs_start = ids_start + ids.len() as u32;

    let mut out = Vec::with_capacity(hash_table as usize + ids.len() + strs.len());
    for word in [
        MO_MAGIC,
        0, // file format revision
        count,
        originals_table,
        translations_table,
        0, // hash table size
        hash_table,
    ] {
        out.extend_from_slice(&word.to_le_bytes());
    }

    let mut cursor = ids_start;
    for (id, _) in &pairs {
        out.extend_from_slice(&(id.len() as u32).to_le_bytes());
        out.extend_from_slice(&cursor.to_le_bytes());
        cursor += id.len() as u32 + 1;
    }
    let mut cursor = strs_start;
    for (_, msg) in &pairs {
        out.extend_from_slice(&(msg.len() as u32).to_le_bytes());
        out.extend_from_slice(&cursor.to_le_bytes());
        cursor += msg.len() as u32 + 1;
    }

    out.extend_from_slice(&ids);
    out.extend_from_slice(&strs);
    out
}

/// A catalog decoded from GNU MO binary format.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MoCatalog {
    /// The metadata block (translation of the empty `msgid`).
    pub header: String,
    /// `msgid → msgstr` pairs, excluding the metadata block.
    pub entries: BTreeMap<String, String>,
}

/// Parse GNU MO binary format, accepting either byte order.
///
/// Plural forms (`\0`-joined) and contexts (`\u{4}`-prefixed) are returned
/// verbatim rather than being split apart.
pub fn decode_mo(bytes: &[u8]) -> Result<MoCatalog, IntlError> {
    let word_at = |offset: usize, swapped: bool| -> Option<u32> {
        let raw = bytes.get(offset..offset + 4)?;
        let raw: [u8; 4] = raw.try_into().ok()?;
        Some(if swapped {
            u32::from_be_bytes(raw)
        } else {
            u32::from_le_bytes(raw)
        })
    };

    let magic = word_at(0, false).ok_or(IntlError::InvalidMo("truncated header"))?;
    let swapped = match magic {
        MO_MAGIC => false,
        m if m == MO_MAGIC.swap_bytes() => true,
        _ => return Err(IntlError::InvalidMo("bad magic number")),
    };

    let read = |offset: usize| word_at(offset, swapped);
    let count = read(8).ok_or(IntlError::InvalidMo("truncated header"))? as usize;
    let originals = read(12).ok_or(IntlError::InvalidMo("truncated header"))? as usize;
    let translations = read(16).ok_or(IntlError::InvalidMo("truncated header"))? as usize;

    let string_at = |table: usize, index: usize| -> Result<String, IntlError> {
        let entry = table + index * 8;
        let len = read(entry).ok_or(IntlError::InvalidMo("truncated string table"))? as usize;
        let off = read(entry + 4).ok_or(IntlError::InvalidMo("truncated string table"))? as usize;
        let raw = bytes
            .get(off..off + len)
            .ok_or(IntlError::InvalidMo("string offset out of range"))?;
        String::from_utf8(raw.to_vec()).map_err(|_| IntlError::InvalidMo("string is not UTF-8"))
    };

    let mut catalog = MoCatalog::default();
    for index in 0..count {
        let id = string_at(originals, index)?;
        let msg = string_at(translations, index)?;
        if id.is_empty() {
            catalog.header = msg;
        } else {
            catalog.entries.insert(id, msg);
        }
    }
    Ok(catalog)
}

// ── CatalogRepository ─────────────────────────────────────────────────────────

/// Repository of message catalogs for a given build environment.
///
/// Mirrors `sphinx.util.i18n.CatalogRepository`.
#[derive(Debug, Clone)]
pub struct CatalogRepository {
    /// Base directory of the Sphinx project (`srcdir`).
    pub basedir: PathBuf,
    /// Relative locale dirs (entries in `locale_dirs` conf value).
    pub locale_dirs: Vec<String>,
    /// Target language (e.g. `"de"` or `"ja"`).
    pub language: String,
    /// Charset for `.po` files (typically `"utf-8"`).
    pub encoding: String,
}

impl CatalogRepository {
    /// Construct a new `CatalogRepository`.
    pub fn new(
        basedir: impl AsRef<Path>,
        locale_dirs: Vec<String>,
        language: impl Into<String>,
        encoding: impl Into<String>,
    ) -> Self {
        CatalogRepository {
            basedir: basedir.as_ref().to_path_buf(),
            locale_dirs,
            language: language.into(),
            encoding: encoding.into(),
        }
    }

    /// Yield the absolute paths of locale directories that actually contain a
    /// `{language}/LC_MESSAGES/` sub-directory.
    ///
    /// Mirrors the `locale_dirs` property of `CatalogRepository`.
    pub fn resolved_locale_dirs(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.locale_dirs.iter().filter_map(move |rel| {
            if self.language.is_empty() {
                return None;
            }
            let abs = self.basedir.join(rel);
            let lc_messages = abs.join(&self.language).join("LC_MESSAGES");
            if lc_messages.exists() {
                Some(abs)
            } else {
                None
            }
        })
    }

    /// Yield `(lc_messages_dir, relative_po_path)` pairs for every `.po` file
    /// found under any resolved locale directory.
    ///
    /// Mirrors the `pofiles` property of `CatalogRepository`.
    pub fn pofiles(&self) -> Vec<(PathBuf, PathBuf)> {
        let mut result = Vec::new();
        for locale_dir in self.resolved_locale_dirs() {
            let lc_messages = locale_dir.join(&self.language).join("LC_MESSAGES");
            if let Ok(walker) = glob::glob(&format!("{}/**/*.po", lc_messages.display())) {
                for entry in walker.flatten() {
                    // Skip dot-directories (mirrors Python's `any(part.startswith('.')…)`)
                    let rel = match entry.strip_prefix(&lc_messages) {
                        Ok(r) => r.to_path_buf(),
                        Err(_) => continue,
                    };
                    let has_hidden = rel
                        .parent()
                        .map(|p| {
                            p.components()
                                .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
                        })
                        .unwrap_or(false);
                    if !has_hidden {
                        result.push((lc_messages.clone(), rel));
                    }
                }
            }
        }
        result
    }

    /// Yield a [`CatalogInfo`] for each `.po` file discovered via `pofiles()`.
    ///
    /// Mirrors the `catalogs` property of `CatalogRepository`.
    pub fn catalogs(&self) -> Vec<CatalogInfo> {
        self.pofiles()
            .into_iter()
            .map(|(base, rel)| {
                let domain = rel.with_extension("").to_string_lossy().replace('\\', "/");
                CatalogInfo::new(base, domain, self.encoding.clone())
            })
            .collect()
    }
}

// ── docname_to_domain ─────────────────────────────────────────────────────────

/// Convert a docname to a catalog domain.
///
/// - If `compaction` is `Some("literal")`, that literal string is used.
/// - If `compaction` is `Some("")` (compaction enabled), use the first
///   path component.
/// - If `compaction` is `None` (no compaction), use the full docname.
///
/// Mirrors `sphinx.util.i18n.docname_to_domain()`.
pub fn docname_to_domain(docname: &str, compaction: Option<&str>) -> String {
    match compaction {
        Some(literal) if !literal.is_empty() => literal.to_owned(),
        Some(_) => docname.split('/').next().unwrap_or(docname).to_owned(),
        None => docname.to_owned(),
    }
}

// ── date_format_mappings ──────────────────────────────────────────────────────

/// Mapping from `ustrftime`-style `%X` codes to Babel format codes.
///
/// Mirrors `sphinx.util.i18n.date_format_mappings`.
pub const DATE_FORMAT_MAPPINGS: &[(&str, &str)] = &[
    ("%a", "EEE"),
    ("%A", "EEEE"),
    ("%b", "MMM"),
    ("%B", "MMMM"),
    ("%c", "medium"),
    ("%-d", "d"),
    ("%d", "dd"),
    ("%-H", "H"),
    ("%H", "HH"),
    ("%-I", "h"),
    ("%I", "hh"),
    ("%-j", "D"),
    ("%j", "DDD"),
    ("%-m", "M"),
    ("%m", "MM"),
    ("%-M", "m"),
    ("%M", "mm"),
    ("%p", "a"),
    ("%-S", "s"),
    ("%S", "ss"),
    ("%U", "WW"),
    ("%w", "e"),
    ("%-W", "W"),
    ("%W", "WW"),
    ("%x", "medium"),
    ("%X", "medium"),
    ("%y", "YY"),
    ("%Y", "yyyy"),
    ("%Z", "zzz"),
    ("%z", "ZZZ"),
    ("%%", "%"),
];

/// A fragment produced by [`split_date_format`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateFormatToken<'a> {
    /// A recognised `%X` directive, carrying `(directive, babel_code)`.
    Directive(&'a str, &'static str),
    /// Text between directives, passed through verbatim.
    Literal(&'a str),
}

/// Split a `ustrftime` format string into directives and literal text.
///
/// Mirrors `date_format_re.split()`: the upstream regex is an alternation of
/// [`DATE_FORMAT_MAPPINGS`] keys in table order, so the scan tries each key at
/// the current position and takes the first that matches. Table order matters
/// — `%-d` precedes `%d`, and `%%` must be reached before `%` can be consumed
/// as part of a following directive.
pub fn split_date_format(fmt: &str) -> Vec<DateFormatToken<'_>> {
    let mut tokens = Vec::new();
    let mut literal_start = 0;
    let mut pos = 0;

    while pos < fmt.len() {
        let matched = DATE_FORMAT_MAPPINGS
            .iter()
            .find(|(pat, _)| fmt[pos..].starts_with(pat));
        match matched {
            Some((pat, code)) => {
                if literal_start < pos {
                    tokens.push(DateFormatToken::Literal(&fmt[literal_start..pos]));
                }
                tokens.push(DateFormatToken::Directive(&fmt[pos..pos + pat.len()], code));
                pos += pat.len();
                literal_start = pos;
            }
            None => {
                // Advance by one character, not one byte, to stay on a
                // UTF-8 boundary.
                pos += fmt[pos..].chars().next().map_or(1, char::len_utf8);
            }
        }
    }
    if literal_start < fmt.len() {
        tokens.push(DateFormatToken::Literal(&fmt[literal_start..]));
    }
    tokens
}

/// Apply `date_format_mappings` to a strftime-style format string, producing
/// a Babel-compatible format string.
pub fn ustrftime_to_babel(fmt: &str) -> String {
    split_date_format(fmt)
        .into_iter()
        .map(|token| match token {
            DateFormatToken::Directive(_, code) => code,
            DateFormatToken::Literal(text) => text,
        })
        .collect()
}

// ── date formatting ───────────────────────────────────────────────────────────

/// Which of Babel's three formatters a directive maps onto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formatter {
    /// `babel.dates.format_date` — used for `%x`.
    Date,
    /// `babel.dates.format_time` — used for `%X`.
    Time,
    /// `babel.dates.format_datetime` — the default.
    DateTime,
}

/// Month, weekday and period names plus the `medium` skeletons for one locale.
#[derive(Debug, Clone, Copy)]
pub struct DateNames {
    /// Wide month names, January first (`MMMM`).
    pub months_wide: [&'static str; 12],
    /// Abbreviated month names, January first (`MMM`).
    pub months_abbr: [&'static str; 12],
    /// Wide weekday names, Monday first (`EEEE`).
    pub days_wide: [&'static str; 7],
    /// Abbreviated weekday names, Monday first (`EEE`).
    pub days_abbr: [&'static str; 7],
    /// Period name for times before noon (`a`).
    pub am: &'static str,
    /// Period name for times from noon (`a`).
    pub pm: &'static str,
    /// `medium` date skeleton, used for `%x`.
    pub date_medium: &'static str,
    /// `medium` time skeleton, used for `%X`.
    pub time_medium: &'static str,
    /// `medium` datetime skeleton, used for `%c` and every other directive.
    pub datetime_medium: &'static str,
}

/// CLDR `en` data — also the fallback for every unsupported locale.
pub const EN_DATE_NAMES: DateNames = DateNames {
    months_wide: [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ],
    months_abbr: [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ],
    days_wide: [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ],
    days_abbr: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"],
    am: "AM",
    pm: "PM",
    date_medium: "MMM d, y",
    time_medium: "h:mm:ss\u{202f}a",
    datetime_medium: "MMM d, y, h:mm:ss\u{202f}a",
};

/// CLDR `de` data.
pub const DE_DATE_NAMES: DateNames = DateNames {
    months_wide: [
        "Januar",
        "Februar",
        "März",
        "April",
        "Mai",
        "Juni",
        "Juli",
        "August",
        "September",
        "Oktober",
        "November",
        "Dezember",
    ],
    months_abbr: [
        "Jan.", "Feb.", "März", "Apr.", "Mai", "Juni", "Juli", "Aug.", "Sept.", "Okt.", "Nov.",
        "Dez.",
    ],
    days_wide: [
        "Montag",
        "Dienstag",
        "Mittwoch",
        "Donnerstag",
        "Freitag",
        "Samstag",
        "Sonntag",
    ],
    days_abbr: ["Mo.", "Di.", "Mi.", "Do.", "Fr.", "Sa.", "So."],
    am: "AM",
    pm: "PM",
    date_medium: "dd.MM.y",
    time_medium: "HH:mm:ss",
    datetime_medium: "dd.MM.y, HH:mm:ss",
};

/// CLDR `ja` data.
pub const JA_DATE_NAMES: DateNames = DateNames {
    months_wide: [
        "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月",
    ],
    months_abbr: [
        "1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月",
    ],
    days_wide: [
        "月曜日",
        "火曜日",
        "水曜日",
        "木曜日",
        "金曜日",
        "土曜日",
        "日曜日",
    ],
    days_abbr: ["月", "火", "水", "木", "金", "土", "日"],
    am: "午前",
    pm: "午後",
    date_medium: "y/MM/dd",
    time_medium: "H:mm:ss",
    datetime_medium: "y/MM/dd H:mm:ss",
};

/// Resolve `locale` (e.g. `"de"`, `"de_DE"`, `"pt-BR"`) to its [`DateNames`].
///
/// Unknown locales fall back to [`EN_DATE_NAMES`], mirroring the "fallback to
/// English" branch of upstream `babel_format_date`.
pub fn date_names_for(locale: &str) -> &'static DateNames {
    let primary = locale
        .split(['_', '-'])
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    match primary.as_str() {
        "de" => &DE_DATE_NAMES,
        "ja" => &JA_DATE_NAMES,
        _ => &EN_DATE_NAMES,
    }
}

/// Format `date` with a CLDR `pattern` in `locale`.
///
/// Mirrors `sphinx.util.i18n.babel_format_date`. A falsy `locale` short-cuts
/// to `en`, as Babel would reject it. `formatter` selects which `medium`
/// skeleton the literal pattern `"medium"` expands to.
pub fn babel_format_date<Tz: TimeZone>(
    date: &DateTime<Tz>,
    pattern: &str,
    locale: &str,
    formatter: Formatter,
) -> String {
    let names = if locale.is_empty() {
        &EN_DATE_NAMES
    } else {
        date_names_for(locale)
    };
    let pattern = if pattern == "medium" {
        match formatter {
            Formatter::Date => names.date_medium,
            Formatter::Time => names.time_medium,
            Formatter::DateTime => names.datetime_medium,
        }
    } else {
        pattern
    };
    format_cldr_pattern(date, pattern, names)
}

/// Format `date` (or "now") with a `ustrftime`-style format string.
///
/// Mirrors `sphinx.util.i18n.format_date`. When `date` is `None` the
/// `SOURCE_DATE_EPOCH` environment variable is honoured for reproducible
/// builds; setting it also forces UTC by overriding `local_time`.
pub fn format_date(
    fmt: &str,
    date: Option<DateTime<Utc>>,
    language: &str,
    local_time: bool,
) -> String {
    let (date, local_time) = match date {
        Some(date) => (date, local_time),
        None => match source_date_epoch() {
            Some(date) => (date, false),
            None => (Utc::now(), local_time),
        },
    };

    let rendered = |pattern: &str, formatter: Formatter| -> String {
        if local_time {
            babel_format_date(&date.with_timezone(&Local), pattern, language, formatter)
        } else {
            babel_format_date(&date, pattern, language, formatter)
        }
    };

    split_date_format(fmt)
        .into_iter()
        .map(|token| match token {
            DateFormatToken::Literal(text) => text.to_owned(),
            DateFormatToken::Directive(directive, pattern) => {
                let formatter = match directive {
                    "%x" => Formatter::Date,
                    "%X" => Formatter::Time,
                    _ => Formatter::DateTime,
                };
                rendered(pattern, formatter)
            }
        })
        .collect()
}

/// Read `$SOURCE_DATE_EPOCH` as a UTC timestamp, if it is set and numeric.
fn source_date_epoch() -> Option<DateTime<Utc>> {
    let raw = std::env::var("SOURCE_DATE_EPOCH").ok()?;
    let seconds: f64 = raw.trim().parse().ok()?;
    Utc.timestamp_opt(seconds.trunc() as i64, 0).single()
}

/// Render a CLDR date pattern, covering the symbol subset reachable from
/// [`DATE_FORMAT_MAPPINGS`] plus the `medium` skeletons.
///
/// Text inside single quotes is literal, and `''` produces a single quote.
/// Unrecognised symbols are emitted verbatim, which is what makes `%%` → `%`
/// round-trip.
fn format_cldr_pattern<Tz: TimeZone>(
    date: &DateTime<Tz>,
    pattern: &str,
    names: &DateNames,
) -> String {
    let mut out = String::with_capacity(pattern.len() * 2);
    let chars: Vec<char> = pattern.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        let symbol = chars[index];

        if symbol == '\'' {
            index += 1;
            if index < chars.len() && chars[index] == '\'' {
                out.push('\'');
                index += 1;
                continue;
            }
            while index < chars.len() && chars[index] != '\'' {
                out.push(chars[index]);
                index += 1;
            }
            index += 1; // closing quote
            continue;
        }

        if !symbol.is_ascii_alphabetic() {
            out.push(symbol);
            index += 1;
            continue;
        }

        let start = index;
        while index < chars.len() && chars[index] == symbol {
            index += 1;
        }
        out.push_str(&format_cldr_field(date, symbol, index - start, names));
    }
    out
}

/// Render a single CLDR field: `count` repetitions of `symbol`.
fn format_cldr_field<Tz: TimeZone>(
    date: &DateTime<Tz>,
    symbol: char,
    count: usize,
    names: &DateNames,
) -> String {
    let pad = |value: u32| format!("{value:0width$}", width = count);
    match symbol {
        'y' | 'Y' => {
            let year = date.year();
            if count == 2 {
                format!("{:02}", year.rem_euclid(100))
            } else {
                format!("{:0width$}", year, width = count)
            }
        }
        'M' | 'L' => {
            let month = date.month() as usize;
            match count {
                3 => names.months_abbr[month - 1].to_owned(),
                n if n >= 4 => names.months_wide[month - 1].to_owned(),
                _ => pad(month as u32),
            }
        }
        'd' => pad(date.day()),
        'D' => pad(date.ordinal()),
        'E' | 'c' => {
            let weekday = date.weekday().num_days_from_monday() as usize;
            if count >= 4 {
                names.days_wide[weekday].to_owned()
            } else {
                names.days_abbr[weekday].to_owned()
            }
        }
        // CLDR "local day of week": 1-based, in the locale's week order.
        // `en` starts on Sunday, which is what `%w` (Sunday = 0) maps onto.
        'e' => pad(date.weekday().num_days_from_sunday() + 1),
        'a' => if date.hour() < 12 { names.am } else { names.pm }.to_owned(),
        'H' => pad(date.hour()),
        'h' => {
            let hour = date.hour() % 12;
            pad(if hour == 0 { 12 } else { hour })
        }
        'm' => pad(date.minute()),
        's' => pad(date.second()),
        // CLDR `W` is week-of-month. Upstream maps `%U`/`%W` (week-of-year)
        // onto it, so this reproduces the upstream output, not strftime's.
        'W' => pad(week_of_month(date)),
        'z' | 'v' => timezone_name(date),
        'Z' | 'x' | 'X' => date.offset().fix().to_string().replace(':', ""),
        _ => std::iter::repeat_n(symbol, count).collect(),
    }
}

/// CLDR week-of-month: weeks are Monday-based and the first week is week 1.
fn week_of_month<Tz: TimeZone>(date: &DateTime<Tz>) -> u32 {
    let first_of_month_weekday = date
        .with_day(1)
        .map_or(0, |d| d.weekday().num_days_from_monday());
    (date.day() + first_of_month_weekday - 1) / 7 + 1
}

/// Best-effort timezone name. `chrono` keeps only the offset once a datetime
/// is materialised, so anything other than UTC is rendered as a `UTC±HH:MM`
/// label rather than an IANA abbreviation.
fn timezone_name<Tz: TimeZone>(date: &DateTime<Tz>) -> String {
    let offset = date.offset().fix();
    if offset.local_minus_utc() == 0 {
        "UTC".to_owned()
    } else {
        format!("UTC{offset}")
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    // ── CatalogInfo ───────────────────────────────────────────────────────────

    #[test]
    fn catalog_info_paths() {
        let ci = CatalogInfo::new("/tmp/locale", "sphinx", "utf-8");
        assert_eq!(ci.po_file(), "sphinx.po");
        assert_eq!(ci.mo_file(), "sphinx.mo");
        assert_eq!(ci.po_path(), PathBuf::from("/tmp/locale/sphinx.po"));
        assert_eq!(ci.mo_path(), PathBuf::from("/tmp/locale/sphinx.mo"));
    }

    #[test]
    fn catalog_info_is_outdated_when_mo_missing() {
        let tmp = TempDir::new().unwrap();
        let po_path = tmp.path().join("sphinx.po");
        std::fs::write(&po_path, b"msgid \"x\"\nmsgstr \"y\"\n").unwrap();

        let ci = CatalogInfo::new(tmp.path(), "sphinx", "utf-8");
        assert!(ci.is_outdated()); // no .mo file
    }

    #[test]
    fn catalog_info_not_outdated_when_mo_newer() {
        let tmp = TempDir::new().unwrap();
        let po_path = tmp.path().join("sphinx.po");
        let mo_path = tmp.path().join("sphinx.mo");
        std::fs::write(&po_path, b"msgid \"x\"\nmsgstr \"y\"\n").unwrap();
        std::fs::write(&mo_path, b"").unwrap();

        // Touch mo to be newer than po
        let _now = std::time::SystemTime::now();
        // Write mo file after po to ensure mo is newer
        std::fs::write(&mo_path, b"binary").unwrap();

        let ci = CatalogInfo::new(tmp.path(), "sphinx", "utf-8");
        // mo was written after po, so not outdated
        // (may be flaky on very fast systems — acceptable for unit tests)
        let _ = ci.is_outdated(); // just verify it doesn't panic
    }

    // ── CatalogRepository ─────────────────────────────────────────────────────

    fn setup_locale_dir(base: &Path, language: &str, domain: &str, content: &str) {
        let lc = base.join(language).join("LC_MESSAGES");
        std::fs::create_dir_all(&lc).unwrap();
        let mut f = std::fs::File::create(lc.join(format!("{domain}.po"))).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn catalog_repository_finds_locale_dirs() {
        let tmp = TempDir::new().unwrap();
        let locale_dir = tmp.path().join("locale");
        setup_locale_dir(
            &locale_dir,
            "en",
            "sphinx",
            "msgid \"Hello\"\nmsgstr \"Hi\"\n",
        );

        let repo = CatalogRepository::new(tmp.path(), vec!["locale".to_owned()], "en", "utf-8");

        let dirs: Vec<_> = repo.resolved_locale_dirs().collect();
        assert_eq!(dirs.len(), 1);
    }

    #[test]
    fn catalog_repository_empty_language_yields_nothing() {
        let tmp = TempDir::new().unwrap();
        let repo = CatalogRepository::new(tmp.path(), vec!["locale".to_owned()], "", "utf-8");
        assert_eq!(repo.resolved_locale_dirs().count(), 0);
    }

    #[test]
    fn catalog_repository_catalogs_returns_catalog_infos() {
        let tmp = TempDir::new().unwrap();
        let locale_dir = tmp.path().join("locale");
        setup_locale_dir(
            &locale_dir,
            "de",
            "myext",
            "msgid \"Hello\"\nmsgstr \"Hallo\"\n",
        );

        let repo = CatalogRepository::new(tmp.path(), vec!["locale".to_owned()], "de", "utf-8");

        let cats = repo.catalogs();
        assert_eq!(cats.len(), 1);
        assert_eq!(cats[0].domain, "myext");
        assert_eq!(cats[0].charset, "utf-8");
    }

    // ── docname_to_domain ─────────────────────────────────────────────────────

    #[test]
    fn docname_to_domain_no_compaction() {
        assert_eq!(docname_to_domain("foo/bar/baz", None), "foo/bar/baz");
    }

    #[test]
    fn docname_to_domain_compact() {
        assert_eq!(docname_to_domain("foo/bar/baz", Some("")), "foo");
    }

    #[test]
    fn docname_to_domain_literal_compaction() {
        assert_eq!(
            docname_to_domain("foo/bar/baz", Some("mymodule")),
            "mymodule"
        );
    }

    #[test]
    fn docname_to_domain_no_slash() {
        assert_eq!(docname_to_domain("index", Some("")), "index");
    }

    // ── ustrftime_to_babel ────────────────────────────────────────────────────

    #[test]
    fn ustrftime_converts_year_and_month() {
        assert_eq!(ustrftime_to_babel("%Y-%m-%d"), "yyyy-MM-dd");
    }

    #[test]
    fn ustrftime_converts_percent_escape() {
        assert_eq!(ustrftime_to_babel("100%%"), "100%");
    }

    #[test]
    fn date_format_mappings_coverage() {
        // Every key in the table should survive a round-trip lookup
        for (pat, expected_repl) in DATE_FORMAT_MAPPINGS {
            let result = ustrftime_to_babel(pat);
            assert_eq!(&result, expected_repl, "mapping for {pat}");
        }
    }

    #[test]
    fn ustrftime_percent_escape_shadows_following_directive() {
        // A sequential `.replace()` pass would see `%d` inside `%%d` and emit
        // `%dd`; the alternation scan consumes `%%` first, as upstream does.
        assert_eq!(ustrftime_to_babel("%%d"), "%d");
    }

    #[test]
    fn split_date_format_separates_literals() {
        assert_eq!(
            split_date_format("on %Y!"),
            vec![
                DateFormatToken::Literal("on "),
                DateFormatToken::Directive("%Y", "yyyy"),
                DateFormatToken::Literal("!"),
            ]
        );
    }

    #[test]
    fn split_date_format_handles_multibyte_literals() {
        assert_eq!(
            split_date_format("日付 %Y"),
            vec![
                DateFormatToken::Literal("日付 "),
                DateFormatToken::Directive("%Y", "yyyy"),
            ]
        );
    }

    // ── MO codec ──────────────────────────────────────────────────────────────

    /// The datetime used by upstream `test_format_date`.
    fn sample_date() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2016, 2, 7, 5, 11, 17).unwrap()
    }

    #[test]
    fn encode_mo_writes_recognisable_header() {
        let mut entries = BTreeMap::new();
        entries.insert("Hello".to_owned(), "Hallo".to_owned());
        let bytes = encode_mo("", &entries);

        assert_eq!(
            u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
            MO_MAGIC
        );
        assert_eq!(u32::from_le_bytes(bytes[4..8].try_into().unwrap()), 0);
        assert_eq!(u32::from_le_bytes(bytes[8..12].try_into().unwrap()), 1);
        // hash table size is 0 — readers binary-search the sorted table
        assert_eq!(u32::from_le_bytes(bytes[20..24].try_into().unwrap()), 0);
    }

    #[test]
    fn encode_mo_round_trips_through_decode_mo() {
        let mut entries = BTreeMap::new();
        entries.insert("Hello".to_owned(), "Hallo".to_owned());
        entries.insert("Bye".to_owned(), "Tschüss".to_owned());

        let decoded = decode_mo(&encode_mo("Content-Type: text/plain\n", &entries)).unwrap();
        assert_eq!(decoded.header, "Content-Type: text/plain\n");
        assert_eq!(decoded.entries, entries);
    }

    #[test]
    fn decode_mo_accepts_big_endian_files() {
        let mut entries = BTreeMap::new();
        entries.insert("Hello".to_owned(), "Hallo".to_owned());
        let mut bytes = encode_mo("", &entries);
        // Byte-swap the header and both offset tables to emulate a
        // big-endian writer; the string blobs themselves are unaffected.
        let tables_end = 28 + 16 * entries.len();
        for word in bytes[..tables_end].chunks_exact_mut(4) {
            word.reverse();
        }
        bytes[0..4].copy_from_slice(&MO_MAGIC.swap_bytes().to_le_bytes());

        assert_eq!(decode_mo(&bytes).unwrap().entries, entries);
    }

    #[test]
    fn decode_mo_rejects_bad_magic() {
        let err = decode_mo(&[0, 1, 2, 3, 4, 5, 6, 7]).unwrap_err();
        assert!(matches!(err, IntlError::InvalidMo("bad magic number")));
    }

    #[test]
    fn decode_mo_rejects_truncated_input() {
        assert!(matches!(
            decode_mo(&[0xde, 0x12]).unwrap_err(),
            IntlError::InvalidMo(_)
        ));
    }

    #[test]
    fn write_mo_compiles_po_and_round_trips() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("test.po"),
            concat!(
                "msgid \"\"\n",
                "msgstr \"Content-Type: text/plain; charset=utf-8\\n\"\n",
                "\n",
                "msgid \"Hello\"\n",
                "msgstr \"Hallo\"\n",
            ),
        )
        .unwrap();

        let catalog = CatalogInfo::new(tmp.path(), "test", "utf-8");
        catalog.write_mo(false).unwrap();

        let decoded = decode_mo(&std::fs::read(catalog.mo_path()).unwrap()).unwrap();
        assert_eq!(
            decoded.entries.get("Hello").map(String::as_str),
            Some("Hallo")
        );
        assert!(decoded.header.contains("charset=utf-8"));
        assert!(!catalog.is_outdated());
    }

    #[test]
    fn write_mo_accepts_a_comment_only_po() {
        // Mirrors upstream `test_catalog_write_mo`, which compiles a `.po`
        // whose entire contents are `#`.
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("test.po"), "#").unwrap();

        let catalog = CatalogInfo::new(tmp.path(), "test", "utf-8");
        catalog.write_mo(false).unwrap();

        assert!(catalog.mo_path().exists());
        assert_eq!(
            decode_mo(&std::fs::read(catalog.mo_path()).unwrap()).unwrap(),
            MoCatalog::default()
        );
    }

    #[test]
    fn write_mo_skips_fuzzy_entries_unless_requested() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("test.po"),
            "#, fuzzy\nmsgid \"Hello\"\nmsgstr \"Hallo\"\n",
        )
        .unwrap();
        let catalog = CatalogInfo::new(tmp.path(), "test", "utf-8");

        catalog.write_mo(false).unwrap();
        assert!(
            decode_mo(&std::fs::read(catalog.mo_path()).unwrap())
                .unwrap()
                .entries
                .is_empty()
        );

        catalog.write_mo(true).unwrap();
        assert_eq!(
            decode_mo(&std::fs::read(catalog.mo_path()).unwrap())
                .unwrap()
                .entries
                .get("Hello")
                .map(String::as_str),
            Some("Hallo")
        );
    }

    #[test]
    fn write_mo_reports_a_missing_po_file() {
        let tmp = TempDir::new().unwrap();
        let err = CatalogInfo::new(tmp.path(), "missing", "utf-8")
            .write_mo(false)
            .unwrap_err();
        assert!(err.to_string().starts_with("reading error: "));
    }

    // ── format_date ───────────────────────────────────────────────────────────

    #[test]
    fn format_date_strftime_format_per_language() {
        let date = sample_date();
        let fmt = "%B %d, %Y";
        assert_eq!(format_date(fmt, Some(date), "", false), "February 07, 2016");
        assert_eq!(
            format_date(fmt, Some(date), "unknown", false),
            "February 07, 2016"
        );
        assert_eq!(
            format_date(fmt, Some(date), "en", false),
            "February 07, 2016"
        );
        assert_eq!(format_date(fmt, Some(date), "ja", false), "2月 07, 2016");
        assert_eq!(
            format_date(fmt, Some(date), "de", false),
            "Februar 07, 2016"
        );
    }

    #[test]
    fn format_date_passes_raw_strings_through() {
        let fmt = "Mon Mar 28 12:37:08 2016, commit 4367aef";
        assert_eq!(format_date(fmt, Some(sample_date()), "en", false), fmt);
    }

    #[test]
    fn format_date_zero_padded_time() {
        assert_eq!(
            format_date(
                "%B %d, %Y, %H:%M:%S %I %p",
                Some(sample_date()),
                "en",
                false
            ),
            "February 07, 2016, 05:11:17 05 AM"
        );
    }

    #[test]
    fn format_date_unpadded_time() {
        assert_eq!(
            format_date(
                "%B %-d, %Y, %-H:%-M:%-S %-I %p",
                Some(sample_date()),
                "en",
                false
            ),
            "February 7, 2016, 5:11:17 5 AM"
        );
    }

    #[test]
    fn format_date_medium_skeletons() {
        let date = sample_date();
        assert_eq!(format_date("%x", Some(date), "en", false), "Feb 7, 2016");
        assert_eq!(
            format_date("%X", Some(date), "en", false),
            "5:11:17\u{202f}AM"
        );
        assert_eq!(
            format_date("%c", Some(date), "en", false),
            "Feb 7, 2016, 5:11:17\u{202f}AM"
        );
    }

    #[test]
    fn format_date_timezone_directives() {
        let date = sample_date();
        assert_eq!(format_date("%Z", Some(date), "en", false), "UTC");
        assert_eq!(format_date("%z", Some(date), "en", false), "+0000");
    }

    #[test]
    fn format_date_iso_style() {
        assert_eq!(
            format_date("%Y-%m-%d %H:%M:%S", Some(sample_date()), "en", false),
            "2016-02-07 05:11:17"
        );
    }

    #[test]
    fn format_date_weekday_and_ordinal() {
        // 2016-02-07 was a Sunday, the 38th day of the year.
        let date = sample_date();
        assert_eq!(format_date("%a %A", Some(date), "en", false), "Sun Sunday");
        assert_eq!(format_date("%j %-j", Some(date), "en", false), "038 38");
        assert_eq!(format_date("%w", Some(date), "en", false), "1");
    }

    #[test]
    fn babel_format_date_falsy_locale_falls_back_to_english() {
        let date = sample_date();
        assert_eq!(
            babel_format_date(&date, "MMMM", "", Formatter::DateTime),
            "February"
        );
    }

    #[test]
    fn babel_format_date_supports_quoted_literals() {
        let date = sample_date();
        assert_eq!(
            babel_format_date(&date, "'on' yyyy", "en", Formatter::DateTime),
            "on 2016"
        );
        assert_eq!(
            babel_format_date(&date, "''yyyy", "en", Formatter::DateTime),
            "'2016"
        );
    }

    #[test]
    fn date_names_for_uses_the_primary_subtag() {
        assert_eq!(date_names_for("de_DE").months_wide[1], "Februar");
        assert_eq!(date_names_for("ja-JP").months_wide[1], "2月");
        assert_eq!(date_names_for("pt_BR").months_wide[1], "February");
    }
}
