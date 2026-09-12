//! A tiny TOML subset parser.
//!
//! `aicfwc` avoids a `toml`/`serde` dependency (offline builds), so image
//! manifests are parsed with this small parser. Supported syntax:
//!
//! * `key = value` with basic strings, integers (decimal/hex/`_`), booleans
//!   and arrays,
//! * `[table]` and `[table.sub]` headers,
//! * `[[array.of.tables]]`,
//! * `#` comments (also inside arrays).
//!
//! Not supported: inline tables, multi-line basic strings, datetimes.

use std::collections::BTreeMap;

/// A parsed TOML value.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Basic string.
    Str(String),
    /// Integer.
    Int(i64),
    /// Boolean.
    Bool(bool),
    /// Array.
    Array(Vec<Value>),
    /// Table.
    Table(Table),
}

/// A TOML table.
pub type Table = BTreeMap<String, Value>;

impl Value {
    /// Borrow as a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Borrow as an integer.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    /// Borrow as a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Borrow as an array.
    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Borrow as a table.
    pub fn as_table(&self) -> Option<&Table> {
        match self {
            Value::Table(t) => Some(t),
            _ => None,
        }
    }

    fn as_table_mut(&mut self) -> Option<&mut Table> {
        match self {
            Value::Table(t) => Some(t),
            _ => None,
        }
    }
}

/// Parse a TOML document into a table.
pub fn parse(input: &str) -> Result<Table, String> {
    let mut root = Table::new();
    let mut current: Vec<String> = Vec::new();
    let mut array_path: Option<Vec<String>> = None;

    for logical in logical_lines(input)? {
        let line = logical.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(inner) = line.strip_prefix("[[").and_then(|s| s.strip_suffix("]]")) {
            let path = split_key(inner)?;
            push_array_table(&mut root, &path)?;
            current = path.clone();
            array_path = Some(path);
            continue;
        }
        if let Some(inner) = strip_single_brackets(line) {
            let path = split_key(inner)?;
            ensure_table_mut_path(&mut root, &path)?;
            current = path;
            array_path = None;
            continue;
        }

        let Some((key, raw_value)) = split_assignment(line)? else {
            return Err(format!("invalid TOML line: {line:?}"));
        };
        let value = parse_value(raw_value)?;
        let segments = split_key(key)?;
        let (last, parents) = segments
            .split_last()
            .ok_or_else(|| "empty key".to_string())?;

        let mut cursor: &mut Table = if let Some(path) = &array_path {
            last_array_element(&mut root, path)?
        } else if current.is_empty() {
            &mut root
        } else {
            navigate_mut(&mut root, &current)?
        };
        for parent in parents {
            cursor = ensure_table_mut(cursor, parent);
        }
        cursor.insert(last.clone(), value);
    }

    Ok(root)
}

fn logical_lines(input: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut pending = String::new();
    for physical in input.lines() {
        let stripped = strip_comment(physical);
        if pending.is_empty() {
            if stripped.trim().is_empty() {
                continue;
            }
            pending = stripped;
        } else {
            pending.push('\n');
            pending.push_str(&stripped);
        }
        if brackets_balanced(&pending)? && quotes_closed(&pending) {
            out.push(std::mem::take(&mut pending));
        }
    }
    if !pending.trim().is_empty() {
        return Err(format!("unbalanced value: {pending:?}"));
    }
    Ok(out)
}

fn strip_comment(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut in_string = false;
    let mut escaped = false;
    for ch in line.chars() {
        if escaped {
            out.push(ch);
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => {
                out.push(ch);
                escaped = true;
            }
            '"' => {
                in_string = !in_string;
                out.push(ch);
            }
            '#' if !in_string => break,
            _ => out.push(ch),
        }
    }
    out
}

fn brackets_balanced(text: &str) -> Result<bool, String> {
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;
    for ch in text.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '[' if !in_string => depth += 1,
            ']' if !in_string => depth -= 1,
            _ => {}
        }
    }
    if depth < 0 {
        return Err(format!("unbalanced ']' in {text:?}"));
    }
    Ok(depth == 0)
}

fn quotes_closed(text: &str) -> bool {
    let mut in_string = false;
    let mut escaped = false;
    for ch in text.chars() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            _ => {}
        }
    }
    !in_string
}

fn strip_single_brackets(line: &str) -> Option<&str> {
    if line.starts_with("[[") {
        return None;
    }
    line.strip_prefix('[')?.strip_suffix(']')
}

fn split_assignment(line: &str) -> Result<Option<(&str, &str)>, String> {
    let mut in_string = false;
    let mut escaped = false;
    for (index, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => escaped = true,
            '"' => in_string = !in_string,
            '=' if !in_string => {
                let key = line[..index].trim();
                if key.is_empty() {
                    return Err(format!("missing key before '=': {line:?}"));
                }
                return Ok(Some((key, line[index + 1..].trim())));
            }
            _ => {}
        }
    }
    Ok(None)
}

fn split_key(key: &str) -> Result<Vec<String>, String> {
    let mut segments = Vec::new();
    for segment in key.split('.') {
        let segment = segment.trim();
        let segment = if let Some(stripped) = segment.strip_prefix('"') {
            let (decoded, _) = unquote(segment)?;
            if !stripped.ends_with('"') {
                return Err(format!("invalid quoted key: {key:?}"));
            }
            decoded
        } else {
            segment.to_string()
        };
        if segment.is_empty() {
            return Err(format!("invalid key: {key:?}"));
        }
        segments.push(segment);
    }
    Ok(segments)
}

fn unquote(text: &str) -> Result<(String, usize), String> {
    let Some(rest) = text.strip_prefix('"') else {
        return Ok((text.to_string(), text.len()));
    };
    let mut out = String::new();
    let mut escaped = false;
    for (index, ch) in rest.char_indices() {
        if escaped {
            let decoded = match ch {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '"' => '"',
                '\\' => '\\',
                other => return Err(format!("unsupported escape \\{other}")),
            };
            out.push(decoded);
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '"' => return Ok((out, index + 2)),
            _ => out.push(ch),
        }
    }
    Err(format!("unterminated string: {text:?}"))
}

fn parse_value(text: &str) -> Result<Value, String> {
    let text = text.trim();
    if text.starts_with('"') {
        let (s, consumed) = unquote(text)?;
        if !text[consumed..].trim().is_empty() {
            return Err(format!("trailing characters after string: {text:?}"));
        }
        return Ok(Value::Str(s));
    }
    if text.starts_with('[') {
        return parse_array(text).map(Value::Array);
    }
    match text {
        "true" => return Ok(Value::Bool(true)),
        "false" => return Ok(Value::Bool(false)),
        _ => {}
    }
    parse_int(text).map(Value::Int)
}

fn parse_array(text: &str) -> Result<Vec<Value>, String> {
    let inner = text
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .ok_or_else(|| format!("invalid array: {text:?}"))?;
    let mut values = Vec::new();
    for item in split_top_level(inner, ',') {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }
        values.push(parse_value(item)?);
    }
    Ok(values)
}

/// Split `text` at `separator` characters not nested in brackets or strings.
fn split_top_level(text: &str, separator: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escaped = false;
    for ch in text.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        match ch {
            '\\' if in_string => {
                current.push(ch);
                escaped = true;
            }
            '"' => {
                in_string = !in_string;
                current.push(ch);
            }
            '[' if !in_string => {
                depth += 1;
                current.push(ch);
            }
            ']' if !in_string => {
                depth -= 1;
                current.push(ch);
            }
            c if c == separator && depth == 0 && !in_string => {
                parts.push(std::mem::take(&mut current));
            }
            _ => current.push(ch),
        }
    }
    parts.push(current);
    parts
}

fn parse_int(text: &str) -> Result<i64, String> {
    let cleaned = text.replace('_', "");
    let (digits, radix) = if let Some(rest) = cleaned.strip_prefix("0x") {
        (rest, 16)
    } else if let Some(rest) = cleaned.strip_prefix("0X") {
        (rest, 16)
    } else {
        (cleaned.as_str(), 10)
    };
    i64::from_str_radix(digits, radix).map_err(|_| format!("invalid integer: {text:?}"))
}

fn ensure_table_mut<'a>(table: &'a mut Table, key: &str) -> &'a mut Table {
    let entry = table
        .entry(key.to_string())
        .or_insert_with(|| Value::Table(Table::new()));
    if !matches!(entry, Value::Table(_)) {
        *entry = Value::Table(Table::new());
    }
    entry.as_table_mut().expect("just inserted table")
}

fn ensure_table_mut_path<'a>(
    root: &'a mut Table,
    path: &[String],
) -> Result<&'a mut Table, String> {
    let mut cursor = root;
    for segment in path {
        cursor = ensure_table_mut(cursor, segment);
    }
    Ok(cursor)
}

fn navigate_mut<'a>(table: &'a mut Table, path: &[String]) -> Result<&'a mut Table, String> {
    let mut cursor = table;
    for segment in path {
        let entry = cursor
            .get_mut(segment)
            .ok_or_else(|| format!("unknown table: {segment}"))?;
        cursor = entry
            .as_table_mut()
            .ok_or_else(|| format!("{segment} is not a table"))?;
    }
    Ok(cursor)
}

fn push_array_table(root: &mut Table, path: &[String]) -> Result<(), String> {
    let (last, parents) = path
        .split_last()
        .ok_or_else(|| "empty array table name".to_string())?;
    let cursor = ensure_table_mut_path(root, parents)?;
    let entry = cursor
        .entry(last.clone())
        .or_insert_with(|| Value::Array(Vec::new()));
    match entry {
        Value::Array(items) => {
            items.push(Value::Table(Table::new()));
            Ok(())
        }
        other => Err(format!("{last} is not an array of tables (got {other:?})")),
    }
}

fn last_array_element<'a>(root: &'a mut Table, path: &[String]) -> Result<&'a mut Table, String> {
    let (last, parents) = path
        .split_last()
        .ok_or_else(|| "empty array path".to_string())?;
    let cursor = navigate_mut(root, parents)?;
    let entry = cursor
        .get_mut(last)
        .ok_or_else(|| format!("unknown array table: {last}"))?;
    match entry {
        Value::Array(items) => items
            .last_mut()
            .and_then(Value::as_table_mut)
            .ok_or_else(|| format!("array {last} has no elements")),
        other => Err(format!("{last} is not an array of tables (got {other:?})")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_manifest_shape() {
        let doc = r#"
# a comment
[image]
platform = "d13x"
product  = "general-board"   # inline comment
media    = "spi-nor"
media_dev_id = 0

[[component]]
name = "bootloader"
file = "app-bootloader.bin"
attr = "mtd;required"
updater = true
ram = 0x40100000
"#;
        let root = parse(doc).unwrap();
        let image = root.get("image").unwrap().as_table().unwrap();
        assert_eq!(image.get("platform").unwrap().as_str(), Some("d13x"));
        assert_eq!(image.get("media_dev_id").unwrap().as_int(), Some(0));
        let components = root.get("component").unwrap().as_array().unwrap();
        assert_eq!(components.len(), 1);
        let first = components[0].as_table().unwrap();
        assert_eq!(first.get("name").unwrap().as_str(), Some("bootloader"));
        assert_eq!(first.get("updater").unwrap().as_bool(), Some(true));
        assert_eq!(first.get("ram").unwrap().as_int(), Some(0x4010_0000));
    }

    #[test]
    fn parses_arrays() {
        let root = parse("a = [ \"x\", \"y\", 3 ]\n").unwrap();
        let array = root.get("a").unwrap().as_array().unwrap();
        assert_eq!(array.len(), 3);
        assert_eq!(array[0].as_str(), Some("x"));
        assert_eq!(array[2].as_int(), Some(3));
    }
}
