//! Color literals for `textDocument/documentColor` and `colorPresentation`.
//!
//! Detects color literals in config values:
//! - hex `#rgb` / `#rrggbb` (uppercase tolerated, `#rrggbbaa` read as RGB+alpha)
//! - CSS named colors in color-bearing keys (`color`-ish, `theme`-ish,
//!   `background`/`foreground` statusLine-style keys)
//! - ANSI SGR style strings (embedded `ESC[...m` sequences, raw ESC or JSON
//!   `\u001b` escape form)
//!
//! Tolerance law: anything unparseable is ignored, never reported as a color,
//! so plain strings produce no false positives.

use lsp_max::lsp_types::Position;
use lsp_max::lsp_types::{Color, ColorInformation, ColorPresentation, Range, TextEdit};

/// CSS named colors (subset with exact `#rrggbb` equivalents for round-trip).
pub fn named_color(name: &str) -> Option<Color> {
    let c: (u8, u8, u8) = match name.to_ascii_lowercase().as_str() {
        "red" => (255, 0, 0),
        "green" => (0, 128, 0),
        "lime" => (0, 255, 0),
        "blue" => (0, 0, 255),
        "yellow" => (255, 255, 0),
        "cyan" | "aqua" => (0, 255, 255),
        "magenta" | "fuchsia" => (255, 0, 255),
        "black" => (0, 0, 0),
        "white" => (255, 255, 255),
        "gray" | "grey" => (128, 128, 128),
        "orange" => (255, 165, 0),
        "purple" => (128, 0, 128),
        "pink" => (255, 192, 203),
        "brown" => (165, 42, 42),
        _ => return None,
    };
    Some(color255(c.0, c.1, c.2, 255))
}

/// Build a `Color` from 0-255 components.
pub fn color255(r: u8, g: u8, b: u8, a: u8) -> Color {
    let f = |v: u8| v as f32 / 255.0;
    Color {
        red: f(r),
        green: f(g),
        blue: f(b),
        alpha: f(a),
    }
}

/// Parse a hex color literal (with `#`) into a `Color`.
pub fn parse_hex(s: &str) -> Option<Color> {
    let h = s.strip_prefix('#')?;
    if !h.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let hexval = |chunk: &str| u8::from_str_radix(chunk, 16).ok();
    match h.len() {
        3 => Some(color255(
            hexval(&h[0..1])? * 17,
            hexval(&h[1..2])? * 17,
            hexval(&h[2..3])? * 17,
            255,
        )),
        6 => Some(color255(
            hexval(&h[0..2])?,
            hexval(&h[2..4])?,
            hexval(&h[4..6])?,
            255,
        )),
        8 => Some(color255(
            hexval(&h[0..2])?,
            hexval(&h[2..4])?,
            hexval(&h[4..6])?,
            hexval(&h[6..8])?,
        )),
        _ => None,
    }
}

/// Detect whether a JSON key is color-bearing: contains "color" or "theme",
/// or is `background`/`foreground` (statusLine-style keys).
fn color_bearing_key(key: &str) -> bool {
    let k = key.to_ascii_lowercase();
    k.contains("color") || k.contains("theme") || k == "background" || k == "foreground"
}

/// Which detector produced a color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorKind {
    Hex,
    Named,
    Ansi,
}

/// A detected color literal.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedColor {
    pub range: Range,
    pub color: Color,
    pub raw: String,
    pub kind: ColorKind,
}

/// The nearest `"key":` name preceding byte offset `pos` on the line, if any.
fn preceding_key(line: &str, pos: usize) -> Option<String> {
    let bytes = line.as_bytes();
    let mut i = 0usize;
    let mut last: Option<String> = None;
    while i < pos.min(bytes.len()) {
        if bytes[i] == b'"' {
            let Some(close) = line[i + 1..].find('"') else {
                break;
            };
            let close = i + 1 + close;
            if line[close + 1..].trim_start().starts_with(':') {
                last = Some(line[i + 1..close].to_string());
            }
            i = close + 1;
            continue;
        }
        i += 1;
    }
    last
}

/// Scan document text for color literals. Results are sorted by position and
/// deduplicated by range (leftmost detection wins when two detectors overlap).
pub fn detect_colors(text: &str) -> Vec<DetectedColor> {
    let mut out: Vec<DetectedColor> = Vec::new();
    for (line_no, line) in text.lines().enumerate() {
        // Hex literals: '#' followed by exactly 3/6/8 hex digits.
        let bytes = line.as_bytes();
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] != b'#' {
                i += 1;
                continue;
            }
            let mut end = i + 1;
            while end < bytes.len() && bytes[end].is_ascii_hexdigit() {
                end += 1;
            }
            let span = end - (i + 1);
            if span == 3 || span == 6 || span == 8 {
                let raw = &line[i..end];
                if let Some(color) = parse_hex(raw) {
                    out.push(DetectedColor {
                        range: range_at(line_no, i, end, line),
                        color,
                        raw: raw.to_string(),
                        kind: ColorKind::Hex,
                    });
                }
                i = end;
            } else {
                i = end.max(i + 1);
            }
        }

        // Named colors, only when the nearest preceding quoted `"key":` is
        // color-bearing (handles nested lines like
        // `"statusLine": { "background": "red" }`).
        for (start, word) in words(line) {
            if let Some(key) = preceding_key(line, start) {
                if color_bearing_key(&key) {
                    if let Some(color) = named_color(word) {
                        out.push(DetectedColor {
                            range: range_at(line_no, start, start + word.len(), line),
                            color,
                            raw: word.to_string(),
                            kind: ColorKind::Named,
                        });
                    }
                }
            }
        }

        // ANSI SGR sequences in string values.
        for (start, seq) in ansi_sgr_spans(line) {
            if let Some(color) = parse_ansi(&seq) {
                out.push(DetectedColor {
                    range: range_at(line_no, start, start + seq.len(), line),
                    color,
                    raw: seq.clone(),
                    kind: ColorKind::Ansi,
                });
            }
        }
    }
    out.sort_by_key(|d| (d.range.start.line, d.range.start.character));
    out.dedup_by(|a, b| a.range == b.range);
    out
}

/// Range for byte span [start, end) on `line`, in UTF-16 columns per LSP.
fn range_at(line_no: usize, start: usize, end: usize, line: &str) -> Range {
    let u16col = |b: usize| line[..b].encode_utf16().count() as u32;
    Range::new(
        Position::new(line_no as u32, u16col(start)),
        Position::new(line_no as u32, u16col(end)),
    )
}

/// (utf16-start, word) pairs of maximal alphabetic words on the line.
fn words(line: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    for (i, c) in line.char_indices() {
        if c.is_ascii_alphabetic() {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(s) = start.take() {
            out.push((utf16_len(&line[..s]), &line[s..i]));
        }
    }
    if let Some(s) = start {
        out.push((utf16_len(&line[..s]), &line[s..]));
    }
    out
}

fn utf16_len(s: &str) -> usize {
    s.encode_utf16().count()
}

/// SGR sequences on a line as (byte-start, sequence-text) pairs. Matches raw
/// ESC (`\x1b`) and JSON-escaped (`\u001b`) forms, both ending in `m`.
fn ansi_sgr_spans(line: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let (esc_len, is_esc) = if bytes[i] == 0x1b {
            (1usize, true)
        } else if line[i..].starts_with("\\u001b") || line[i..].starts_with("\\x1b") {
            (6usize, true)
        } else {
            (0usize, false)
        };
        if !is_esc {
            i += 1;
            continue;
        }
        // Walk to the terminating 'm' over CSI param bytes.
        let mut end = i + esc_len + 1; // skip '['
        if end <= bytes.len() && !line[i + esc_len..].starts_with('[') {
            i += esc_len;
            continue;
        }
        while end < bytes.len()
            && (bytes[end].is_ascii_digit() || bytes[end] == b';' || bytes[end] == b'?')
        {
            end += 1;
        }
        if end < bytes.len() && bytes[end] == b'm' {
            let seq = line[i..=end]
                .replace("\\u001b", "\u{1b}")
                .replace("\\x1b", "\u{1b}");
            out.push((i, seq));
            i = end + 1;
        } else {
            i += esc_len;
        }
    }
    out
}

/// Parse an SGR sequence (starting at ESC, params, final `m`) into a `Color`.
/// Handles `38;2;r;g;b` truecolor and the 16-color palette (30-37, 90-97).
/// Unknown codes are tolerated and skipped.
fn parse_ansi(seq: &str) -> Option<Color> {
    let body = seq.strip_prefix('\u{1b}')?.strip_prefix('[')?;
    let body = body.strip_suffix('m')?;
    let codes: Vec<Option<u8>> = body
        .split(&[';', ':'][..])
        .map(|p| p.parse::<u8>().ok())
        .collect();
    let mut k = 0usize;
    while k < codes.len() {
        let code = codes[k]?;
        match code {
            38 | 48 if k + 1 < codes.len() => {
                let mode = codes[k + 1];
                if mode == Some(2) && k + 4 < codes.len() {
                    return Some(color255(codes[k + 2]?, codes[k + 3]?, codes[k + 4]?, 255));
                }
                // 256-color / colon-form: tolerate, skip the family.
                k += 4;
            }
            30..=37 => {
                return Some(palette_color(code - 30));
            }
            90..=97 => {
                return Some(palette_color(code - 90 + 8));
            }
            _ => k += 1,
        }
    }
    None
}

/// The 16-color ANSI palette (VGA-ish values).
fn palette_color(idx: u8) -> Color {
    const P: [(u8, u8, u8); 16] = [
        (0, 0, 0),
        (205, 0, 0),
        (0, 205, 0),
        (205, 205, 0),
        (0, 0, 238),
        (205, 0, 205),
        (0, 205, 205),
        (229, 229, 229),
        (127, 127, 127),
        (255, 0, 0),
        (0, 255, 0),
        (255, 255, 0),
        (92, 92, 255),
        (255, 0, 255),
        (0, 255, 255),
        (255, 255, 255),
    ];
    let (r, g, b) = P[(idx & 0x0f) as usize];
    color255(r, g, b, 255)
}

/// Clamp a 0..1 float component to a 0-255 byte.
pub fn to255(v: f32) -> u8 {
    (v * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Render a color as hex, or as a CSS name when `prefer_named` and an exact
/// named equivalent exists.
pub fn render_color(c: &Color, prefer_named: bool) -> String {
    let hex = format!(
        "#{:02x}{:02x}{:02x}",
        to255(c.red),
        to255(c.green),
        to255(c.blue)
    );
    if prefer_named {
        if let Some(name) = css_name(c) {
            return name;
        }
    }
    hex
}

/// Exact CSS name for a color, if one exists in the table.
pub fn css_name(c: &Color) -> Option<String> {
    let probe = color255(to255(c.red), to255(c.green), to255(c.blue), 255);
    for name in [
        "red", "green", "lime", "blue", "yellow", "cyan", "magenta", "black", "white", "gray",
        "orange", "purple", "pink", "brown",
    ] {
        if named_color(name) == Some(probe) {
            return Some(name.to_string());
        }
    }
    None
}

/// `textDocument/documentColor` result item.
pub fn color_information(d: &DetectedColor) -> ColorInformation {
    ColorInformation {
        range: d.range,
        color: d.color,
    }
}

/// `textDocument/colorPresentation`: replacement suggestions for the color at
/// `range` — hex↔named swaps. The presentation matching the current raw form
/// comes first; hex literals additionally get the CSS name when an exact one
/// exists, named colors always get the hex form.
pub fn color_presentations(
    detected: &[DetectedColor],
    _color: &Color,
    range: &Range,
) -> Vec<ColorPresentation> {
    let d = match detected.iter().find(|d| d.range == *range) {
        Some(d) => d,
        None => return Vec::new(),
    };
    let mut out = Vec::new();
    match d.kind {
        ColorKind::Hex | ColorKind::Ansi => {
            out.push(presentation(&d.raw, range));
            if let Some(name) = css_name(&d.color) {
                out.push(presentation(&name, range));
            }
        }
        ColorKind::Named => {
            out.push(presentation(&d.raw, range));
            let hex = format!(
                "#{:02x}{:02x}{:02x}",
                to255(d.color.red),
                to255(d.color.green),
                to255(d.color.blue)
            );
            out.push(presentation(&hex, range));
        }
    }
    out
}

fn presentation(label: &str, range: &Range) -> ColorPresentation {
    ColorPresentation {
        label: label.to_string(),
        text_edit: Some(TextEdit {
            range: *range,
            new_text: label.to_string(),
        }),
        additional_text_edits: None,
    }
}
