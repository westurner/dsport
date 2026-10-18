#!/usr/bin/env python3
"""Compare Python and Rust Sphinx HTML output semantically."""

from __future__ import annotations

import argparse
from collections import Counter
from html.parser import HTMLParser
from pathlib import Path
import re
import sys


KNOWN_CONTENT_GAPS = (
    "extdev/",
    "usage/advanced/websupport/",
    "usage/extensions/example_google.html",
    "usage/extensions/example_numpy.html",
)
PARSER_ERRORS = (
    "Unknown target name:",
    "Unknown directive type",
    "Unknown interpreted text role",
)
WORD_RE = re.compile(r"\w+", re.UNICODE)
VERSION_RE = re.compile(r"\bSphinx v?\d+\.\d+(?:\.\d+)?(?:\+/[0-9a-f]+)?", re.I)
ASSET_VERSION_RE = re.compile(r"\?v=[0-9a-f]{6,}$", re.I)


class HtmlTokens(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.hidden_depth = 0
        self.text: Counter[str] = Counter()
        self.links: Counter[str] = Counter()
        self.anchors: Counter[str] = Counter()

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attributes = dict(attrs)
        if tag in {"script", "style"}:
            self.hidden_depth += 1
            return
        if self.hidden_depth:
            return

        if tag == "a" and attributes.get("href"):
            href = ASSET_VERSION_RE.sub("?v=<hash>", attributes["href"] or "")
            self.links[href] += 1
        if tag == "html" and attributes.get("data-content_root"):
            self.anchors[f"content_root:{attributes['data-content_root']}"] += 1
        if tag in {"section", "dt", "h1", "h2", "h3", "h4", "h5", "h6"}:
            if attributes.get("id"):
                self.anchors[f"{tag}#{attributes['id']}"] += 1

    def handle_endtag(self, tag: str) -> None:
        if tag in {"script", "style"} and self.hidden_depth:
            self.hidden_depth -= 1

    def handle_data(self, data: str) -> None:
        if self.hidden_depth:
            return
        normalized = VERSION_RE.sub("sphinx version", " ".join(data.split()))
        normalized = (
            normalized.replace("’", "'")
            .replace("‘", "'")
            .replace("“", '"')
            .replace("”", '"')
            .replace("—", "-")
            .replace("–", "-")
        )
        self.text.update(token.lower() for token in WORD_RE.findall(normalized))


def parse_html(path: Path) -> HtmlTokens:
    parser = HtmlTokens()
    parser.feed(path.read_text(encoding="utf-8", errors="replace"))
    return parser


def overlap(left: Counter[str], right: Counter[str]) -> float:
    total = sum(left.values()) + sum(right.values())
    if not total:
        return 1.0
    shared = sum(min(count, right.get(token, 0)) for token, count in left.items())
    return 2 * shared / total


def print_counter_gaps(label: str, left: Counter[str], right: Counter[str]) -> None:
    left_only = (left - right).most_common(12)
    right_only = (right - left).most_common(12)
    if not left_only and not right_only:
        return
    print(f"{label} only in Python:")
    for token, count in left_only:
        print(f"  - {count} {token}")
    print(f"{label} only in Rust:")
    for token, count in right_only:
        print(f"  + {count} {token}")


def source_pages(root: Path, source_root: Path) -> set[str]:
    pages = set()
    for path in root.rglob("*.html"):
        relative = path.relative_to(root).as_posix()
        if relative.startswith(KNOWN_CONTENT_GAPS):
            continue
        source = source_root / Path(relative).with_suffix(".rst")
        if source.is_file():
            pages.add(relative)
    return pages


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python-html", type=Path, required=True)
    parser.add_argument("--rust-html", type=Path, required=True)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument("--min-overlap", type=float, default=1.00)
    parser.add_argument("--min-link-overlap", type=float, default=1.00)
    parser.add_argument("--min-anchor-overlap", type=float, default=1.00)
    args = parser.parse_args()

    python_pages = source_pages(args.python_html, args.source)
    rust_pages = source_pages(args.rust_html, args.source)
    missing = sorted(python_pages - rust_pages)
    extra = sorted(rust_pages - python_pages)
    shared_pages = sorted(python_pages & rust_pages)

    python_text: Counter[str] = Counter()
    rust_text: Counter[str] = Counter()
    python_links: Counter[str] = Counter()
    rust_links: Counter[str] = Counter()
    python_anchors: Counter[str] = Counter()
    rust_anchors: Counter[str] = Counter()
    parser_errors: list[str] = []
    page_scores: list[tuple[float, str, int, int]] = []

    for path in args.rust_html.rglob("*.html"):
        html = path.read_text(encoding="utf-8", errors="replace")
        for message in PARSER_ERRORS:
            if message in html:
                parser_errors.append(f"{path.relative_to(args.rust_html)}: {message}")

    for relative in shared_pages:
        python = parse_html(args.python_html / relative)
        rust = parse_html(args.rust_html / relative)
        python_text.update(python.text)
        rust_text.update(rust.text)
        python_links.update(python.links)
        rust_links.update(rust.links)
        python_anchors.update(python.anchors)
        rust_anchors.update(rust.anchors)
        score = overlap(python.text, rust.text)
        page_scores.append((score, relative, sum(python.text.values()), sum(rust.text.values())))

    text_score = overlap(python_text, rust_text)
    link_score = overlap(python_links, rust_links)
    anchor_score = overlap(python_anchors, rust_anchors)
    print(f"Semantic HTML parity: {len(shared_pages)} shared source pages")
    print(f"Visible text overlap: {text_score:.2%}")
    print(f"Link destination overlap: {link_score:.2%}")
    print(f"Heading/anchor overlap: {anchor_score:.2%}")
    print(f"Excluded known content-gap pages: {len(KNOWN_CONTENT_GAPS)} patterns")
    print_counter_gaps("link", python_links, rust_links)
    print_counter_gaps("heading/anchor", python_anchors, rust_anchors)

    if missing:
        print("Missing Rust source pages:")
        for path in missing:
            print(f"  {path}")
    if extra:
        print("Rust-only source pages:")
        for path in extra:
            print(f"  {path}")
    if parser_errors:
        print("Unresolved parser diagnostics:")
        for error in parser_errors:
            print(f"  {error}")

    print("Largest visible-text gaps:")
    for score, path, python_count, rust_count in sorted(page_scores)[:20]:
        print(f"  {score:.1%} {path} (python={python_count}, rust={rust_count})")

    if (
        len(shared_pages) < 100
        or text_score < args.min_overlap
        or link_score < args.min_link_overlap
        or anchor_score < args.min_anchor_overlap
        or missing
        or extra
        or parser_errors
    ):
        print(
            "Semantic parity gate failed (minimum overlap: "
            f"text {args.min_overlap:.1%}, links {args.min_link_overlap:.1%}, "
            f"anchors {args.min_anchor_overlap:.1%}).",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())