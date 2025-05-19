

pub struct TomlDecodeError {
    msg: String,
    doc: String,
    pos: usize,
    lineno: usize,
    colno: usize,
}

impl TomlDecodeError {
    pub fn new(msg: String, doc: String, pos: usize) -> TomlDecodeError {
        let lineno = doc[..pos].matches('\n').count() + 1;
        let colno = pos - doc.rfind('\n', pos).unwrap_or(0);
        let emsg = format!("{} (line {} column {} char {})", msg, lineno, colno, pos);
        TomlDecodeError {
            msg,
            doc,
            pos,
            lineno,
            colno,
        }
    }
}

pub struct CommentValue {
    val: String,
    comment: String,
    _dict: std::collections::HashMap<String, String>, // Assuming _dict is a dictionary
}

impl CommentValue {
    pub fn new(val: String, comment: String, beginline: bool, _dict: std::collections::HashMap<String, String>) -> CommentValue {
        let separator = if beginline { "\n".to_string() } else { " ".to_string() };
        let comment = separator + &comment;
        CommentValue { val, comment, _dict }
    }
}

pub struct CommentValue {
    val: Vec<Any>, // Adjust type as necessary
    comment: String,
    _dict: HashMap<String, Any>, // Adjust type as necessary
}

impl CommentValue {
    pub fn __getitem__(&self, key: usize) -> &Any { // Adjust return type as necessary
        &self.val[key]
    }
}

impl CommentValue {
    pub fn __setitem__(&mut self, key: &str, value: impl std::fmt::Debug) {
        self.val.insert(key.to_string(), value);
    }
}

impl CommentValue {
    pub fn dump(&self, dump_value_func: &dyn Fn(&str) -> String) -> String {
        let retstr = dump_value_func(&self.val);
        if self.val.is_instance_of::<self._dict>() {
            return format!("{}\n{}", self.comment, retstr);
        } else {
            return format!("{}{}", retstr, self.comment);
        }
    }
}

pub fn load(f: &dyn std::io::Read, _dict: Option<Type> = None, decoder: Option<&TomlDecoder>) -> Result<HashMap<String, Value>, TomlDecodeError> {
    /// Parses named file or files as toml and returns a dictionary
    ///
    /// Args:
    ///     f: Path to the file to open, array of files to read into single dict
    ///        or a file descriptor
    ///     _dict: (optional) Specifies the class of the returned toml dictionary
    ///     decoder: The decoder to use
    ///
    /// Returns:
    ///     Parsed toml file represented as a dictionary
    ///
    /// Raises:
    ///     TypeError -- When f is invalid type
    ///     TomlDecodeError: Error while decoding toml
    ///     IOError / FileNotFoundError -- When an array with no valid (existing)
    ///     (Python 2 / Python 3)          file paths is passed

    if _ispath(f) {
        let ffile = std::fs::File::open(_getpath(f)?)?;
        return loads(ffile.read_to_string()?, _dict, decoder);
    } else if let Some(file_list) = f.downcast_ref::<Vec<String>>() {
        use std::fs;
        use std::path::Path;

        if !file_list.iter().any(|path| Path::new(path).exists()) {
            let error_msg = "Load expects a list to contain filenames only.\nThe list needs to contain the path of at least one existing file.";
            return Err(FNFError(error_msg.to_string()));
        }

        let mut decoder = decoder.unwrap_or_else(|| TomlDecoder::new(_dict));
        let mut d = decoder.get_empty_table();

        for l in file_list {
            if Path::new(l).exists() {
                d.extend(load(l, _dict, Some(&decoder))?);
            } else {
                warn("Non-existent filename in list with at least one valid filename");
            }
        }
        return Ok(d);
    } else {
        let mut reader = f;
        let content = reader.read_to_string()?;
        return loads(content, _dict, decoder);
    }
}

use std::collections::HashMap;
use std::io::BufReader;
use chrono::{NaiveDate, NaiveDateTime};

/// A decoder for TOML data.
#[derive(Debug, Clone)]
pub struct TomlDecoder {
    // You can add fields here if needed, for example:
    // dict: HashMap<String, String>,
}

impl TomlDecoder {
    /// Creates a new instance of TomlDecoder.
    pub fn new() -> Self {
        TomlDecoder {
            // Initialize fields if any
        }
    }

    /// Splits a line on quotes.
    pub fn get_split_on_quotes(&self, line: &str) -> Vec<String> {
        // Implementation goes here
        vec![]
    }

    /// Loads an array as a string array.
    pub fn load_array_isstrarray(&self, a: &str) {
        // Implementation goes here
    }

    /// Loads a multiline string from a line.
    pub fn load_line_multiline_str(&self, p: &str) -> (usize, usize) {
        // Implementation goes here
        (0, 0)
    }

    /// Checks if a bounded string is valid.
    pub fn bounded_string(&self, s: &str) -> bool {
        // Implementation goes here
        false
    }

    /// Embeds comments into the current level.
    pub fn embed_comments(&self, idx: usize, currentlevel: &mut HashMap<String, String>) {
        // Implementation goes here
    }

    /// Returns an empty table.
    pub fn get_empty_table(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    /// Loads an array from a string.
    pub fn load_array(&self, a: &str) -> Vec<Box<dyn std::any::Any>> {
        // Implementation goes here
        vec![]
    }

    /// Loads a value from a line.
    pub fn load_value(&self, v: &str, strictly_valid: bool) -> Box<dyn std::any::Any> {
        // Implementation goes here
        Box::new(())
    }

    /// Preserves a comment associated with a key.
    pub fn preserve_comment(&self, line_no: usize, key: &str, comment: &str, beginline: bool) {
        // Implementation goes here
    }
}

/// Detects if the given path is a pathlib path.
fn detect_pathlib_path(p: &BufReader<dyn std::io::Read>) {
    // Implementation goes here
}

/// Checks if the given path is valid.
fn is_path(p: &BufReader<dyn std::io::Read>) -> bool {
    // Implementation goes here
    false
}

/// Loads a date from a string.
fn load_date(val: &str) -> Result<NaiveDate, NaiveDateTime> {
    // Implementation goes here
    Err(NaiveDateTime::from_timestamp(0, 0))
}

/// Loads unicode escapes from a string.
fn load_unicode_escapes(v: &str, hexbytes: Vec<String>, prefix: &str) -> String {
    // Implementation goes here
    String::new()
}

/// Checks if a number is strictly valid.
fn strictly_valid_num(n: &str) {
    // Implementation goes here
}

/// Unescapes a string.
fn unescape(v: &str) -> String {
    // Implementation goes here
    String::new()
}

pub struct TomlDecoder {
    _dict: Box<dyn std::any::Any>, // Using Box<dyn Any> to represent Type[dict]
}

impl TomlDecoder {
    pub fn __init__(_dict: Box<dyn std::any::Any>) -> TomlDecoder {
        TomlDecoder { _dict }
    }
}

impl TomlDecoder {
    pub fn load_line(
        &self,
        line: &str,
        currentlevel: &mut std::collections::HashMap<String, Box<dyn std::any::Any>>,
        multikey: Option<&str>,
        multibackslash: bool,
    ) -> Result<(String, String, bool), String> {
        let mut i = 1;
        let quotesplits = self._get_split_on_quotes(line);
        let mut quoted = false;
        for quotesplit in quotesplits {
            if !quoted && quotesplit.contains('=') {
                break;
            }
            i += quotesplit.matches('=').count();
            quoted = !quoted;
        }
        let pair: Vec<&str> = line.splitn(i, '=').collect();
        let mut strictly_valid = _strictly_valid_num(pair.last().unwrap_or(&"")).unwrap_or(false);
        let mut pair_last = pair.last().unwrap_or(&"").to_string();
        if _number_with_underscores.is_match(&pair_last) {
            pair_last = pair_last.replace('_', "");
        }
        while !pair_last.is_empty() && !matches!(pair_last.chars().next().unwrap(), ' ' | '\t' | '\'' | '"' | '[' | '{') && pair_last.trim() != "true" && pair_last.trim() != "false" {
            if pair_last.parse::<f64>().is_ok() {
                break;
            }
            if _load_date(&pair_last).is_some() {
                break;
            }
            if TIME_RE.is_match(&pair_last) {
                break;
            }
            i += 1;
            let prev_val = pair_last.clone();
            let pair: Vec<&str> = line.splitn(i, '=').collect();
            pair_last = pair.last().unwrap_or(&"").to_string();
            if prev_val == pair_last {
                return Err("Invalid date or number".to_string());
            }
            if strictly_valid {
                strictly_valid = _strictly_valid_num(pair_last.as_str()).unwrap_or(false);
            }
        }
        let pair = vec![
            pair[..pair.len() - 1].join("=").trim().to_string(),
            pair_last.trim().to_string(),
        ];
        let mut key = pair[0].clone();
        if key.contains('.') {
            if key.contains('"') || key.contains('\'') {
                let quotesplits = self._get_split_on_quotes(&key);
                let mut quoted = false;
                let mut levels = Vec::new();
                for quotesplit in quotesplits {
                    if quoted {
                        levels.push(quotesplit);
                    } else {
                        levels.extend(quotesplit.split('.').map(|s| s.trim().to_string()));
                    }
                    quoted = !quoted;
                }
                key = levels.pop().unwrap_or_default();
            } else {
                key = key.split('.').last().unwrap_or("").trim().to_string();
            }
        } else if (key.chars().next().unwrap() == '"' || key.chars().next().unwrap() == '\'') && (key.chars().last().unwrap() == key.chars().next().unwrap()) {
            key = _unescape(&key[1..key.len()-1]);
        }
        let (k, koffset) = self._load_line_multiline_str(&pair[1])?;
        let mut multilinestr = String::new();
        if k > -1 {
            let mut k = k;
            while k > -1 && pair[1].chars().nth(k + koffset) == Some('\\') {
                multibackslash = !multibackslash;
                k -= 1;
            }
            multilinestr = if multibackslash {
                pair[1][..pair[1].len()-1].to_string()
            } else {
                format!("{}\n", pair[1])
            };
            multikey = Some(&pair[0]);
        } else {
            let (value, vtype) = self.load_value(&pair[1], strictly_valid)?;
        }
        if currentlevel.contains_key(&key) {
            return Err("Duplicate keys!".to_string());
        } else {
            if let Some(multikey) = multikey {
                return Ok((multikey.to_string(), multilinestr, multibackslash));
            } else {
                currentlevel.insert(key, value);
            }
        }
        Ok((String::new(), String::new(), false))
    }
}

impl TomlDecoder {
    pub fn _load_line_multiline_str(&self, p: &str) -> (i32, usize) {
        let mut poffset = 0;
        if p.len() < 3 {
            return (-1, poffset);
        }
        if p.chars().next().unwrap() == '[' && (p.trim().chars().last().unwrap() != ']' && self._load_array_isstrarray(p)) {
            let mut newp: Vec<&str> = p[1..].trim().split(',').collect();
            while newp.len() > 1 && !newp.last().unwrap().starts_with('"') && !newp.last().unwrap().starts_with('\'') {
                let last = newp.pop().unwrap();
                let second_last = newp.pop().unwrap();
                newp.push(&format!("{},{}", second_last, last));
            }
            let last = newp.pop().unwrap();
            poffset = p.len() - last.len();
            p = last;
        }
        if !p.starts_with('"') && !p.starts_with('\'') {
            return (-1, poffset);
        }
        if p.chars().nth(1).unwrap() != p.chars().next().unwrap() || p.chars().nth(2).unwrap() != p.chars().next().unwrap() {
            return (-1, poffset);
        }
        if p.len() > 5 && p.chars().last().unwrap() == p.chars().next().unwrap() && p.chars().nth_back(1).unwrap() == p.chars().next().unwrap() && p.chars().nth_back(2).unwrap() == p.chars().next().unwrap() {
            return (-1, poffset);
        }
        (p.len() as i32 - 1, poffset)
    }
}

impl TomlDecoder {
    pub fn load_array(&self, a: &str) -> Vec<Option<Box<dyn std::any::Any>>> {
        let mut retval: Vec<Option<Box<dyn std::any::Any>>> = Vec::new();
        let a = a.trim();
        if !a[1..a.len()-1].contains('[') || a[1..a.len()-1].split('[').next().unwrap().trim() != "" {
            let strarray = self._load_array_isstrarray(a);
            let mut a = if !a[1..a.len()-1].trim().starts_with('{') {
                a[1..a.len()-1].split(',').map(|s| s.to_string()).collect::<Vec<String>>()
            } else {
                let mut new_a: Vec<String> = Vec::new();
                let mut start_group_index = 1;
                let mut end_group_index = 2;
                let mut open_bracket_count = if a.chars().nth(start_group_index).unwrap() == '{' { 1 } else { 0 };
                let mut in_str = false;

                while end_group_index < a.len() - 1 {
                    if a.chars().nth(end_group_index).unwrap() == '"' || a.chars().nth(end_group_index).unwrap() == '\'' {
                        if in_str {
                            let mut backslash_index = end_group_index - 1;
                            while backslash_index > 0 && a.chars().nth(backslash_index).unwrap() == '\\' {
                                in_str = !in_str;
                                backslash_index -= 1;
                            }
                        }
                        in_str = !in_str;
                    }
                    if !in_str && a.chars().nth(end_group_index).unwrap() == '{' {
                        open_bracket_count += 1;
                    }
                    if in_str || a.chars().nth(end_group_index).unwrap() != '}' {
                        end_group_index += 1;
                        continue;
                    } else if a.chars().nth(end_group_index).unwrap() == '}' && open_bracket_count > 1 {
                        open_bracket_count -= 1;
                        end_group_index += 1;
                        continue;
                    }

                    end_group_index += 1;
                    new_a.push(a[start_group_index..end_group_index].to_string());
                    start_group_index = end_group_index + 1;
                    while start_group_index < a.len() - 1 && a.chars().nth(start_group_index).unwrap() != '{' {
                        start_group_index += 1;
                    }
                    end_group_index = start_group_index + 1;
                }
                new_a
            };

            let mut b = 0;
            if strarray {
                while b < a.len() - 1 {
                    let mut ab = a[b].trim().to_string();
                    while !self.bounded_string(&ab) || (ab.len() > 2 && ab.chars().nth(0).unwrap() == ab.chars().nth(1).unwrap() && ab.chars().nth(2).unwrap() == ab.chars().nth(0).unwrap() && ab.chars().nth(ab.len() - 2).unwrap() != ab.chars().nth(0).unwrap() && ab.chars().nth(ab.len() - 3).unwrap() != ab.chars().nth(0).unwrap()) {
                        a[b] = format!("{}{}", a[b], a[b + 1]);
                        ab = a[b].trim().to_string();
                        if b < a.len() - 2 {
                            a.remove(b + 1);
                        } else {
                            a.pop();
                        }
                    }
                    b += 1;
                }
            }
        } else {
            let al: Vec<char> = a[1..a.len()-1].chars().collect();
            let mut a: Vec<String> = Vec::new();
            let mut openarr = 0;
            let mut j = 0;
            for i in 0..al.len() {
                if al[i] == '[' {
                    openarr += 1;
                } else if al[i] == ']' {
                    openarr -= 1;
                } else if al[i] == ',' && openarr == 0 {
                    a.push(al[j..i].iter().collect());
                    j = i + 1;
                }
            }
            a.push(al[j..].iter().collect());
        }
        for i in 0..a.len() {
            let trimmed = a[i].trim();
            if !trimmed.is_empty() {
                let (nval, _ntype) = self.load_value(trimmed);
                retval.push(nval);
            }
        }
        retval
    }
}

impl TomlDecoder {
    pub fn preserve_comment(&self, line_no: i32, key: &str, comment: &str, beginline: bool) {
        // ...
    }
}

impl TomlDecoder {
    pub fn embed_comments(&self, idx: i32, currentlevel: &mut HashMap<String, Box<dyn Any>>) {
        // ...
    }
}

use std::collections::HashMap;

pub struct TomlPreserveCommentDecoder {
    saved_comments: HashMap<String, String>,
}

impl TomlPreserveCommentDecoder {
    pub fn __init__(_dict: Option<HashMap<String, String>>) -> TomlPreserveCommentDecoder {
        let saved_comments = HashMap::new();
        let decoder = TomlPreserveCommentDecoder { saved_comments };
        // Call to the superclass constructor would go here
        decoder
    }
}

impl TomlPreserveCommentDecoder {
    pub fn preserve_comment(&mut self, line_no: usize, key: String, comment: String, beginline: bool) {
        self.saved_comments.insert(line_no, (key, comment, beginline));
    }
}

impl TomlPreserveCommentDecoder {
    pub fn embed_comments(&self, idx: usize, currentlevel: &mut HashMap<String, CommentValue>) {
        if !self.saved_comments.contains_key(&idx) {
            return;
        }

        let (key, comment, beginline) = &self.saved_comments[&idx];
        currentlevel.insert(key.clone(), CommentValue::new(currentlevel[key], comment.clone(), *beginline, self._dict.clone()));
    }
}

pub struct TomlTz {
    _raw_offset: String,
    _sign: i32,
    _hours: i32,
    _minutes: i32,
}

impl TomlTz {
    pub fn __init__(toml_offset: &str) -> () {
        if toml_offset == "Z" {
            self._raw_offset = "+00:00".to_string();
        } else {
            self._raw_offset = toml_offset.to_string();
        }
        self._sign = if self._raw_offset.chars().next().unwrap() == '-' { -1 } else { 1 };
        self._hours = self._raw_offset[1..3].parse::<i32>().unwrap();
        self._minutes = self._raw_offset[4..6].parse::<i32>().unwrap();
    }
}

impl TomlTz {
    pub fn __getinitargs__(&self) -> (i32,) { // Assuming _raw_offset is of type i32
        (self._raw_offset,)
    }
}

impl TomlTz {
    pub fn __deepcopy__(&self, _memo: &mut std::collections::HashMap<*const Self, *const Self>) -> TomlTz {
        TomlTz {
            _raw_offset: self._raw_offset,
            _sign: self._sign,
            _hours: self._hours,
            _minutes: self._minutes,
        }
    }
}

impl TomlTz {
    pub fn tzname(&self, dt: &DateTime<FixedOffset>) -> String {
        format!("UTC{}", self._raw_offset)
    }
}

use chrono::{Duration, TimeZone, Utc};
use std::time::SystemTime;

pub struct TomlTz {
    _raw_offset: i32,
    _sign: i32,
    _hours: i32,
    _minutes: i32,
}

impl TomlTz {
    pub fn utcoffset(&self, dt: SystemTime) -> Option<Duration> {
        Some(Duration::hours(self._sign as i64 * self._hours as i64) + Duration::minutes(self._sign as i64 * self._minutes as i64))
    }
}

use chrono::Duration;

impl TomlTz {
    pub fn dst(&self, dt: &chrono::DateTime<chrono::Utc>) -> Duration {
        Duration::zero()
    }
}

pub fn _ispath(p: &dyn std::io::Read) -> bool {
    use std::io::Read;

    let mut buffer = [0; 1];
    if p.read(&mut buffer).is_ok() {
        return true;
    }
    _detect_pathlib_path(p)
}

pub fn _detect_pathlib_path(p: &dyn std::io::Read) -> bool {
    // Assuming we have a way to check if p is a PurePath
    // This is a placeholder as Rust does not have a direct equivalent to Python's pathlib
    false
}

pub fn _getpath(p: &dyn std::io::Read) -> Result<String, TomlDecodeError> {
    /// Returns the file system path from the given input.
    ///
    /// Args:
    ///     p: The input which can be a file descriptor or a path-like object.
    ///
    /// Returns:
    ///     A string representation of the file system path.
    ///
    /// Raises:
    ///     TomlDecodeError: If the input is not a valid path.
    
    if std::env::var("PYTHON_VERSION").unwrap_or_default() >= "3.6" {
        use std::os::unix::ffi::OsStrExt;
        return Ok(std::ffi::OsStr::from_bytes(p).to_string_lossy().into_owned());
    }
    if _detect_pathlib_path(p)? {
        return Ok(p.to_string());
    }
    return Ok(p.to_string());
}

pub fn loads(s: &str, _dict: Option<Type> = None, decoder: Option<&TomlDecoder>) -> Result<HashMap<String, Value>, TomlDecodeError> {
    /// Parses string as toml
    ///
    /// Args:
    ///     s: String to be parsed
    ///     _dict: (optional) Specifies the class of the returned toml dictionary
    ///
    /// Returns:
    ///     Parsed toml file represented as a dictionary
    ///
    /// Raises:
    ///     TypeError: When a non-string is passed
    ///     TomlDecodeError: Error while decoding toml

    let mut implicitgroups = Vec::new();
    let mut decoder = decoder.unwrap_or_else(|| TomlDecoder::new(_dict));
    let mut retval = decoder.get_empty_table();
    let mut currentlevel = &mut retval;

    if !s.is_string() {
        return Err(TypeError("Expecting something like a string".to_string()));
    }

    let original = s.to_string();
    let sl: Vec<char> = s.chars().collect();
    let mut openarr = 0;
    let mut openstring = false;
    let mut openstrchar = "";
    let mut multilinestr = false;
    let mut arrayoftables = false;
    let mut beginline = true;
    let mut keygroup = false;
    let mut dottedkey = false;
    let mut keyname = 0;
    let mut key = String::new();
    let mut prev_key = String::new();
    let mut line_no = 1;

    for (i, item) in sl.iter().enumerate() {
        if *item == '\r' && sl.get(i + 1).map_or(false, |&c| c == '\n') {
            sl[i] = ' ';
            continue;
        }
        if keyname != 0 {
            key.push(*item);
            if *item == '\n' {
                return Err(TomlDecodeError::new("Key name found without value. Reached end of line.", &original, i));
            }
            if openstring {
                if *item == openstrchar {
                    let mut oddbackslash = false;
                    let mut k = 1;
                    while i >= k && sl[i - k] == '\\' {
                        oddbackslash = !oddbackslash;
                        k += 1;
                    }
                    if !oddbackslash {
                        keyname = 2;
                        openstring = false;
                        openstrchar = "";
                    }
                }
                continue;
            } else if keyname == 1 {
                if item.is_whitespace() {
                    keyname = 2;
                    continue;
                } else if *item == '.' {
                    dottedkey = true;
                    continue;
                } else if item.is_alphanumeric() || *item == '_' || *item == '-' {
                    continue;
                } else if dottedkey && sl[i - 1] == '.' && (*item == '"' || *item == '\'') {
                    openstring = true;
                    openstrchar = *item;
                    continue;
                }
            } else if keyname == 2 {
                if item.is_whitespace() {
                    if dottedkey {
                        if !sl.get(i + 1).map_or(false, |&c| !c.is_whitespace() && c != '.') {
                            keyname = 1;
                        }
                    }
                    continue;
                }
                if *item == '.' {
                    dottedkey = true;
                    if !sl.get(i + 1).map_or(false, |&c| !c.is_whitespace() && c != '.') {
                        keyname = 1;
                    }
                    continue;
                }
            }
            if *item == '=' {
                keyname = 0;
                prev_key = key.trim_end().to_string();
                key.clear();
                dottedkey = false;
            } else {
                return Err(TomlDecodeError::new(format!("Found invalid character in key name: '{}'. Try quoting the key name.", item), &original, i));
            }
        }
        if *item == '\'' && openstrchar != '"' {
            let mut k = 1;
            while sl.get(i - k).map_or(false, |&c| c == '\'') {
                k += 1;
                if k == 3 {
                    break;
                }
            }
            if k == 3 {
                multilinestr = !multilinestr;
                openstring = multilinestr;
            } else {
                openstring = !openstring;
            }
            openstrchar = if openstring { "'" } else { "" };
        }
        if *item == '"' && openstrchar != '\'' {
            let mut oddbackslash = false;
            let mut k = 1;
            let mut tripquote = false;
            while sl.get(i - k).map_or(false, |&c| c == '"') {
                k += 1;
                if k == 3 {
                    tripquote = true;
                    break;
                }
            }
            if k == 1 || (k == 3 && tripquote) {
                while sl.get(i - k).map_or(false, |&c| c == '\\') {
                    oddbackslash = !oddbackslash;
                    k += 1;
                }
            }
            if !oddbackslash {
                if tripquote {
                    multilinestr = !multilinestr;
                    openstring = multilinestr;
                } else {
                    openstring = !openstring;
                }
            }
            openstrchar = if openstring { "\"" } else { "" };
        }
        if *item == '#' && !openstring && !keygroup && !arrayoftables {
            let mut j = i;
            let mut comment = String::new();
            while sl.get(j).map_or(false, |&c| c != '\n') {
                comment.push(s[j]);
                sl[j] = ' ';
                j += 1;
            }
            if openarr == 0 {
                decoder.preserve_comment(line_no, &prev_key, &comment, beginline);
            }
        }
        if *item == '[' && !openstring && !keygroup && !arrayoftables {
            if beginline {
                if sl.get(i + 1).map_or(false, |&c| c == '[') {
                    arrayoftables = true;
                } else {
                    keygroup = true;
                }
            } else {
                openarr += 1;
            }
        }
        if *item == ']' && !openstring {
            if keygroup {
                keygroup = false;
            } else if arrayoftables {
                if sl[i - 1] == ']' {
                    arrayoftables = false;
                }
            } else {
                openarr -= 1;
            }
        }
        if *item == '\n' {
            if openstring || multilinestr {
                if !multilinestr {
                    return Err(TomlDecodeError::new("Unbalanced quotes", &original, i));
                }
                if (sl[i - 1] == '\'' || sl[i - 1] == '"') && sl[i - 2] == sl[i - 1] {
                    sl[i] = sl[i - 1];
                    if sl[i - 3] == sl[i - 1] {
                        sl[i - 3] = ' ';
                    }
                }
            } else if openarr > 0 {
                sl[i] = ' ';
            } else {
                beginline = true;
            }
            line_no += 1;
        } else if beginline && *item != ' ' && *item != '\t' {
            beginline = false;
            if !keygroup && !arrayoftables {
                if *item == '=' {
                    return Err(TomlDecodeError::new("Found empty keyname.", &original, i));
                }
                keyname = 1;
                key.push(*item);
            }
        }
    }
    if keyname != 0 {
        return Err(TomlDecodeError::new("Key name found without value. Reached end of file.", &original, s.len()));
    }
    if openstring {
        return Err(TomlDecodeError::new("Unterminated string found. Reached end of file.", &original, s.len()));
    }
    let s = sl.iter().collect::<String>();
    let s: Vec<&str> = s.split('\n').collect();
    let mut multikey = None;
    let mut multilinestr = String::new();
    let mut multibackslash = false;
    let mut pos = 0;

    for (idx, line) in s.iter().enumerate() {
        if idx > 0 {
            pos += s[idx - 1].len() + 1;
        }

        decoder.embed_comments(idx, currentlevel);

        if !multilinestr.is_empty() && !multibackslash && !multilinestr.contains('\n') {
            line = line.trim();
        }
        if line.is_empty() && (multikey.is_none() || multibackslash) {
            continue;
        }
        if let Some(multikey) = multikey {
            if multibackslash {
                multilinestr.push_str(line);
            } else {
                multilinestr.push_str(line);
            }
            multibackslash = false;
            let closed = if multilinestr.starts_with('[') {
                line.ends_with(']')
            } else if multilinestr.len() > 2 {
                line.ends_with(multilinestr.chars().next().unwrap()) && line.chars().rev().nth(1) == Some(multilinestr.chars().next().unwrap()) && line.chars().rev().nth(2) == Some(multilinestr.chars().next().unwrap())
            } else {
                false
            };
            if closed {
                let value = decoder.load_value(&multilinestr)?;
                currentlevel.insert(multikey, value);
                multikey = None;
                multilinestr.clear();
            } else {
                let mut k = multilinestr.len() - 1;
                while k > 0 && multilinestr.chars().nth(k) == Some('\\') {
                    multibackslash = !multibackslash;
                    k -= 1;
                }
                if multibackslash {
                    multilinestr.pop();
                } else {
                    multilinestr.push('\n');
                }
            }
            continue;
        }
        if line.starts_with('[') {
            arrayoftables = false;
            if line.len() == 1 {
                return Err(TomlDecodeError::new("Opening key group bracket on line by itself.", &original, pos));
            }
            if line.chars().nth(1) == Some('[') {
                arrayoftables = true;
                line = &line[2..];
                let splitstr = "]]";
            } else {
                line = &line[1..];
                let splitstr = "]";
            }
            let mut i = 1;
            let quotesplits = decoder._get_split_on_quotes(line);
            let mut quoted = false;
            for quotesplit in quotesplits {
                if !quoted && quotesplit.contains(splitstr) {
                    break;
                }
                i += quotesplit.matches(splitstr).count();
                quoted = !quoted;
            }
            line = line.split(splitstr).take(i).collect::<Vec<&str>>().join(splitstr);
            if line.len() < i + 1 || line.trim() != "" {
                return Err(TomlDecodeError::new("Key group not on a line by itself.", &original, pos));
            }
            let groups: Vec<&str> = line.split('.').collect();
            let mut i = 0;
            while i < groups.len() {
                let group = groups[i].trim();
                if group.is_empty() {
                    return Err(TomlDecodeError::new("Can't have a keygroup with an empty name", &original, pos));
                }
                if group.starts_with('"') || group.starts_with('\'') {
                    let groupstr = group.to_string();
                    let mut j = i + 1;
                    while !(groupstr.chars().next().unwrap() == groupstr.chars().last().unwrap() && groupstr.len() > 1) {
                        j += 1;
                        if j > groups.len() + 2 {
                            return Err(TomlDecodeError::new(format!("Invalid group name '{}'. Something went wrong.", groupstr), &original, pos));
                        }
                    }
                    groups[i] = &groupstr[1..groupstr.len() - 1];
                    groups.splice(i + 1..j, std::iter::empty());
                } else {
                    if !_groupname_re.is_match(group) {
                        return Err(TomlDecodeError::new(format!("Invalid group name '{}'. Try quoting it.", group), &original, pos));
                    }
                }
                i += 1;
            }
            currentlevel = &mut retval;
            for i in 0..groups.len() {
                let group = groups[i];
                if group.is_empty() {
                    return Err(TomlDecodeError::new("Can't have a keygroup with an empty name", &original, pos));
                }
                if let Some(existing) = currentlevel.get(group) {
                    if i == groups.len() - 1 {
                        if implicitgroups.contains(&group) {
                            implicitgroups.retain(|g| g != group);
                            if arrayoftables {
                                return Err(TomlDecodeError::new("An implicitly defined table can't be an array", &original, pos));
                            }
                        } else if arrayoftables {
                            currentlevel.insert(group.to_string(), vec![decoder.get_empty_table()]);
                        } else {
                            return Err(TomlDecodeError::new(format!("What? {} already exists? {}", group, currentlevel), &original, pos));
                        }
                    }
                } else {
                    currentlevel.insert(group.to_string(), decoder.get_empty_table());
                    if i == groups.len() - 1 && arrayoftables {
                        currentlevel.insert(group.to_string(), vec![decoder.get_empty_table()]);
                    }
                }
                currentlevel = currentlevel.get_mut(group).unwrap();
                if arrayoftables {
                    currentlevel = currentlevel.last_mut().unwrap();
                }
            }
        } else if line.starts_with("{") {
            if !line.ends_with("}") {
                return Err(TomlDecodeError::new("Line breaks are not allowed in inline objects", &original, pos));
            }
            decoder.load_inline_object(line, currentlevel, multikey, multibackslash)?;
        } else if line.contains('=') {
            let ret = decoder.load_line(line, currentlevel, multikey, multibackslash)?;
            if let Some((mk, mls, mb)) = ret {
                multikey = Some(mk);
                multilinestr = mls;
                multibackslash = mb;
            }
        }
    }
    Ok(retval)
}

impl TomlDecoder {
    pub fn _get_split_on_quotes(&self, line: &str) -> Vec<String> {
        let doublequotesplits: Vec<&str> = line.split('"').collect();
        let mut quoted = false;
        let mut quotesplits = Vec::new();
        
        if doublequotesplits.len() > 1 && doublequotesplits[0].contains('\'') {
            let mut singlequotesplits: Vec<&str> = doublequotesplits[0].split('\'').collect();
            let mut doublequotesplits = &doublequotesplits[1..];
            while singlequotesplits.len() % 2 == 0 && !doublequotesplits.is_empty() {
                singlequotesplits.last_mut().unwrap().to_string() += &format!("\"{}", doublequotesplits[0]);
                doublequotesplits = &doublequotesplits[1..];
                if singlequotesplits.last().unwrap().contains('\'') {
                    singlequotesplits = singlequotesplits[..singlequotesplits.len()-1]
                        .iter()
                        .chain(singlequotesplits.last().unwrap().split('\''))
                        .map(|s| s.trim())
                        .collect();
                }
            }
            quotesplits.extend(singlequotesplits);
        }
        
        for doublequotesplit in doublequotesplits {
            if quoted {
                quotesplits.push(doublequotesplit.to_string());
            } else {
                quotesplits.extend(doublequotesplit.split('\'').map(|s| s.trim().to_string()));
            }
            quoted = !quoted;
        }
        quotesplits
    }
}

impl TomlDecoder {
    pub fn get_empty_table(&self) -> std::collections::HashMap<Box<dyn std::any::Any>> {
        (self._dict)()
    }
}

pub fn _strictly_valid_num(n: &str) -> bool {
    let n = n.trim();
    if n.is_empty() {
        return false;
    }
    if n.starts_with('_') {
        return false;
    }
    if n.ends_with('_') {
        return false;
    }
    if n.contains("._") || n.contains("_." ) {
        return false;
    }
    if n.len() == 1 {
        return true;
    }
    if n.starts_with('0') && !n.starts_with("0.") && !n.starts_with("0o") && !n.starts_with("0b") && !n.starts_with("0x") {
        return false;
    }
    let mut n = n.to_string();
    if n.starts_with('+') || n.starts_with('-') {
        n.remove(0);
        if n.len() > 1 && n.starts_with('0') && !n.starts_with("0.") {
            return false;
        }
    }
    if n.contains("__") {
        return false;
    }
    true
}

impl TomlDecoder {
    pub fn _load_array_isstrarray(&self, a: &str) -> bool {
        let a = a[1..a.len()-1].trim();
        if !a.is_empty() && (a.chars().next().unwrap() == '"' || a.chars().next().unwrap() == '\'') {
            return true;
        }
        false
    }
}

impl TomlDecoder {
    pub fn bounded_string(&self, s: &str) -> bool {
        if s.is_empty() {
            return true;
        }
        if s.chars().last().unwrap() != s.chars().next().unwrap() {
            return false;
        }
        let mut i = -2;
        let mut backslash = false;
        while s.len() as isize + i > 0 {
            if s.chars().nth((s.len() as isize + i) as usize).unwrap() == '\\' {
                backslash = !backslash;
                i -= 1;
            } else {
                break;
            }
        }
        !backslash
    }
}

impl TomlDecoder {
    pub fn load_value(&self, v: &str, strictly_valid: bool) -> Result<Box<dyn std::any::Any>, String> {
        if v.is_empty() {
            return Err("Empty value is invalid".to_string());
        }
        if v == "true" {
            return Ok(Box::new(true));
        } else if v.to_lowercase() == "true" {
            return Err("Only all lowercase booleans allowed".to_string());
        } else if v == "false" {
            return Ok(Box::new(false));
        } else if v.to_lowercase() == "false" {
            return Err("Only all lowercase booleans allowed".to_string());
        } else if v.starts_with('"') || v.starts_with('\'') {
            let quotechar = v.chars().next().unwrap();
            let testv: Vec<&str> = v[1..].split(quotechar).collect();
            let mut triplequote = false;
            let mut triplequotecount = 0;
            let mut closed = false;

            let mut i = 0;
            while i < testv.len() {
                if testv[i].is_empty() {
                    if triplequote {
                        triplequotecount += 1;
                    } else {
                        closed = true;
                    }
                } else {
                    let mut oddbackslash = false;
                    let mut j = testv[i].len() as isize - 1;
                    while j >= 0 {
                        if testv[i].chars().nth(j as usize).unwrap() == '\\' {
                            oddbackslash = !oddbackslash;
                        }
                        j -= 1;
                    }
                    if !oddbackslash {
                        if closed {
                            return Err("Found tokens after a closed string. Invalid TOML.".to_string());
                        } else {
                            if !triplequote || triplequotecount > 1 {
                                closed = true;
                            } else {
                                triplequotecount = 0;
                            }
                        }
                    }
                }
                i += 1;
            }

            let mut v = v.to_string();
            if quotechar == '"' {
                let escapeseqs: Vec<&str> = v.split('\\').collect();
                let mut backslash = false;
                for i in escapeseqs.iter().skip(1) {
                    if i.is_empty() {
                        backslash = !backslash;
                    } else {
                        if !_escapes.contains(&i.chars().next().unwrap()) && (i.chars().next().unwrap() != 'u' && i.chars().next().unwrap() != 'U' && !backslash) {
                            return Err("Reserved escape sequence used".to_string());
                        }
                        if backslash {
                            backslash = false;
                        }
                    }
                }
                for prefix in ["\\u", "\\U"].iter() {
                    if v.contains(*prefix) {
                        let hexbytes: Vec<&str> = v.split(*prefix).collect();
                        v = _load_unicode_escapes(hexbytes[0], hexbytes[1..].to_vec(), *prefix);
                    }
                }
                v = _unescape(&v);
            }
            if v.len() > 1 && v.chars().nth(1).unwrap() == quotechar && (v.len() < 3 || v.chars().nth(1).unwrap() == v.chars().nth(2).unwrap()) {
                v = v[2..v.len()-2].to_string();
            }
            return Ok(Box::new(v[1..v.len()-1].to_string()));
        } else if v.starts_with('[') {
            return Ok(Box::new(self.load_array(v)?));
        } else if v.starts_with('{') {
            let inline_object = self.get_empty_inline_table();
            self.load_inline_object(v, inline_object);
            return Ok(Box::new(inline_object));
        } else if let Some(captures) = TIME_RE.captures(v) {
            let h: i32 = captures[1].parse().unwrap();
            let m: i32 = captures[2].parse().unwrap();
            let s: i32 = captures[3].parse().unwrap();
            let ms: i32 = captures[4].parse().unwrap_or(0);
            let time = chrono::NaiveTime::from_hms_milli(h, m, s, ms);
            return Ok(Box::new(time));
        } else {
            if let Some(parsed_date) = _load_date(v) {
                return Ok(Box::new(parsed_date));
            }
            if !strictly_valid {
                return Err("Weirdness with leading zeroes or underscores in your number.".to_string());
            }
            let mut itype = "int";
            let mut neg = false;
            let mut v = v.to_string();
            if v.starts_with('-') {
                neg = true;
                v = v[1..].to_string();
            } else if v.starts_with('+') {
                v = v[1..].to_string();
            }
            v = v.replace('_', "");
            let lowerv = v.to_lowercase();
            if v.contains('.') || (!v.contains('x') && (v.contains('e') || v.contains('E'))) {
                if v.contains('.') && v.split('.').nth(1).unwrap().is_empty() {
                    return Err("This float is missing digits after the point".to_string());
                }
                if !v.chars().next().unwrap().is_digit(10) {
                    return Err("This float doesn't have a leading digit".to_string());
                }
                let v = v.parse::<f64>().unwrap();
                itype = "float";
            } else if lowerv.len() == 3 && (lowerv == "inf" || lowerv == "nan") {
                let v = v.parse::<f64>().unwrap();
                itype = "float";
            }
            if itype == "int" {
                let v = i64::from_str_radix(&v, 0).unwrap();
                if neg {
                    return Ok(Box::new(-v));
                }
                return Ok(Box::new(v));
            }
        }
        Err("Invalid value".to_string())
    }
}

use std::collections::HashMap;

/// A decoder for TOML files that preserves comments.
#[derive(Debug, Clone)]
pub struct TomlPreserveCommentDecoder {
    /// A map to store saved comments associated with line numbers.
    saved_comments: HashMap<usize, (String, String, bool)>,
}

impl TomlPreserveCommentDecoder {
    /// Creates a new instance of `TomlPreserveCommentDecoder`.
    pub fn new(_dict: Option<HashMap<String, String>>) -> TomlPreserveCommentDecoder {
        let saved_comments = HashMap::new();
        TomlPreserveCommentDecoder { saved_comments }
    }

    /// Preserves a comment associated with a specific line number and key.
    pub fn preserve_comment(&mut self, line_no: usize, key: String, comment: String, beginline: bool) {
        self.saved_comments.insert(line_no, (key, comment, beginline));
    }

    /// Embeds comments into the current level based on the saved comments.
    pub fn embed_comments(&self, idx: usize, currentlevel: &mut HashMap<String, CommentValue>) {
        if !self.saved_comments.contains_key(&idx) {
            return;
        }

        let (key, comment, beginline) = &self.saved_comments[&idx];
        currentlevel.insert(key.clone(), CommentValue::new(currentlevel[key], comment.clone(), *beginline, _dict.clone()));
    }
}

use std::collections::HashMap;

/// A struct representing a value with an associated comment and a dictionary.
#[derive(Debug, Clone)]
pub struct CommentValue {
    val: Vec<String>, // Adjust type as necessary
    comment: String,
    _dict: HashMap<String, String>, // Adjust type as necessary
}

impl CommentValue {
    /// Creates a new `CommentValue` instance.
    pub fn new(val: Vec<String>, comment: String, beginline: bool, _dict: HashMap<String, String>) -> CommentValue {
        let separator = if beginline { "\n".to_string() } else { " ".to_string() };
        let comment = separator + &comment;
        CommentValue { val, comment, _dict }
    }

    /// Gets the value at the specified index.
    pub fn get_item(&self, key: usize) -> &String { // Adjust return type as necessary
        &self.val[key]
    }

    /// Sets the value at the specified key.
    pub fn set_item(&mut self, key: usize, value: String) { // Adjust parameter type as necessary
        self.val[key] = value;
    }

    /// Dumps the value and comment as a formatted string.
    pub fn dump(&self, dump_value_func: &dyn Fn(&Vec<String>) -> String) -> String {
        let retstr = dump_value_func(&self.val);
        if self._dict.contains_key(&self.val[0]) { // Adjust condition as necessary
            format!("{}\n{}", self.comment, retstr)
        } else {
            format!("{}{}", retstr, self.comment)
        }
    }
}

/// A struct representing a TOML decoder that preserves comments.
pub struct TomlPreserveCommentDecoder {
    saved_comments: HashMap<usize, (String, String, bool)>, // Adjust types as necessary
    _dict: HashMap<String, String>, // Adjust type as necessary
}

impl TomlPreserveCommentDecoder {
    /// Embeds comments into the current level based on the index.
    pub fn embed_comments(&self, idx: usize, currentlevel: &mut HashMap<String, CommentValue>) {
        if !self.saved_comments.contains_key(&idx) {
            return;
        }

        let (key, comment, beginline) = &self.saved_comments[&idx];
        currentlevel.insert(key.clone(), CommentValue::new(currentlevel[key].val.clone(), comment.clone(), *beginline, self._dict.clone()));
    }
}

pub fn _detect_pathlib_path(p: &dyn std::io::Read) -> bool {
    // Check if the Python version is 3.4 or higher
    if std::env::var("PYTHON_VERSION").unwrap_or_default() >= "3.4" {
        // Assuming we have a way to check if p is a PurePath
        // This is a placeholder as Rust does not have a direct equivalent to Python's pathlib
        // You would need to implement the logic to check for PurePath here
        return false; // Replace with actual check
    }
    false
}

#[derive(Debug, Clone)]
pub struct TomlDecodeError {
    msg: String,
    doc: String,
    pos: usize,
    lineno: usize,
    colno: usize,
}

impl TomlDecodeError {
    /// Creates a new TomlDecodeError.
    ///
    /// # Arguments
    ///
    /// * `msg` - A message describing the error.
    /// * `doc` - The document where the error occurred.
    /// * `pos` - The position in the document where the error occurred.
    pub fn new(msg: String, doc: String, pos: usize) -> TomlDecodeError {
        let lineno = doc[..pos].matches('\n').count() + 1;
        let colno = pos - doc.rfind('\n', pos).unwrap_or(0);
        let emsg = format!("{} (line {} column {} char {})", msg, lineno, colno, pos);
        TomlDecodeError {
            msg,
            doc,
            pos,
            lineno,
            colno,
        }
    }
}

pub fn loads(s: &str, _dict: Option<Type>, decoder: Option<&TomlDecoder>) -> Result<HashMap<String, Value>, TomlDecodeError> {
    /// Parses string as toml
    ///
    /// # Arguments
    ///
    /// * `s` - String to be parsed.
    /// * `_dict` - (optional) Specifies the class of the returned toml dictionary.
    ///
    /// # Returns
    ///
    /// Parsed toml file represented as a dictionary.
    ///
    /// # Errors
    ///
    /// * `TypeError` - When a non-string is passed.
    /// * `TomlDecodeError` - Error while decoding toml.
    
    let mut implicitgroups = Vec::new();
    let mut decoder = decoder.unwrap_or_else(|| TomlDecoder::new(_dict));
    let mut retval = decoder.get_empty_table();
    let mut currentlevel = &mut retval;

    if !s.is_string() {
        return Err(TomlDecodeError::new("Expecting something like a string".to_string(), s.to_string(), 0));
    }

    let original = s.to_string();
    let sl: Vec<char> = s.chars().collect();
    let mut openarr = 0;
    let mut openstring = false;
    let mut openstrchar = "";
    let mut multilinestr = false;
    let mut arrayoftables = false;
    let mut beginline = true;
    let mut keygroup = false;
    let mut dottedkey = false;
    let mut keyname = 0;
    let mut key = String::new();
    let mut prev_key = String::new();
    let mut line_no = 1;

    for (i, item) in sl.iter().enumerate() {
        if *item == '\r' && sl.get(i + 1).map_or(false, |&c| c == '\n') {
            sl[i] = ' ';
            continue;
        }
        if keyname != 0 {
            key.push(*item);
            if *item == '\n' {
                return Err(TomlDecodeError::new("Key name found without value. Reached end of line.".to_string(), &original, i));
            }
            if openstring {
                if *item == openstrchar {
                    let mut oddbackslash = false;
                    let mut k = 1;
                    while i >= k && sl[i - k] == '\\' {
                        oddbackslash = !oddbackslash;
                        k += 1;
                    }
                    if !oddbackslash {
                        keyname = 2;
                        openstring = false;
                        openstrchar = "";
                    }
                }
                continue;
            } else if keyname == 1 {
                if item.is_whitespace() {
                    keyname = 2;
                    continue;
                } else if *item == '.' {
                    dottedkey = true;
                    continue;
                } else if item.is_alphanumeric() || *item == '_' || *item == '-' {
                    continue;
                } else if dottedkey && sl[i - 1] == '.' && (*item == '"' || *item == '\'') {
                    openstring = true;
                    openstrchar = *item;
                    continue;
                }
            } else if keyname == 2 {
                if item.is_whitespace() {
                    if dottedkey {
                        if !sl.get(i + 1).map_or(false, |&c| !c.is_whitespace() && c != '.') {
                            keyname = 1;
                        }
                    }
                    continue;
                }
                if *item == '.' {
                    dottedkey = true;
                    if !sl.get(i + 1).map_or(false, |&c| !c.is_whitespace() && c != '.') {
                        keyname = 1;
                    }
                    continue;
                }
            }
            if *item == '=' {
                keyname = 0;
                prev_key = key.trim_end().to_string();
                key.clear();
                dottedkey = false;
            } else {
                return Err(TomlDecodeError::new(format!("Found invalid character in key name: '{}'. Try quoting the key name.", item), &original, i));
            }
        }
        if *item == '\'' && openstrchar != '"' {
            let mut k = 1;
            while sl.get(i - k).map_or(false, |&c| c == '\'') {
                k += 1;
                if k == 3 {
                    break;
                }
            }
            if k == 3 {
                multilinestr = !multilinestr;
                openstring = multilinestr;
            } else {
                openstring = !openstring;
            }
            openstrchar = if openstring { "'" } else { "" };
        }
        if *item == '"' && openstrchar != '\'' {
            let mut oddbackslash = false;
            let mut k = 1;
            let mut tripquote = false;
            while sl.get(i - k).map_or(false, |&c| c == '"') {
                k += 1;
                if k == 3 {
                    tripquote = true;
                    break;
                }
            }
            if k == 1 || (k == 3 && tripquote) {
                while sl.get(i - k).map_or(false, |&c| c == '\\') {
                    oddbackslash = !oddbackslash;
                    k += 1;
                }
            }
            if !oddbackslash {
                if tripquote {
                    multilinestr = !multilinestr;
                    openstring = multilinestr;
                } else {
                    openstring = !openstring;
                }
            }
            openstrchar = if openstring { "\"" } else { "" };
        }
        if *item == '#' && !openstring && !keygroup && !arrayoftables {
            let mut j = i;
            let mut comment = String::new();
            while sl.get(j).map_or(false, |&c| c != '\n') {
                comment.push(s[j]);
                sl[j] = ' ';
                j += 1;
            }
            if openarr == 0 {
                decoder.preserve_comment(line_no, &prev_key, &comment, beginline);
            }
        }
        if *item == '[' && !openstring && !keygroup && !arrayoftables {
            if beginline {
                if sl.get(i + 1).map_or(false, |&c| c == '[') {
                    arrayoftables = true;
                } else {
                    keygroup = true;
                }
            } else {
                openarr += 1;
            }
        }
        if *item == ']' && !openstring {
            if keygroup {
                keygroup = false;
            } else if arrayoftables {
                if sl[i - 1] == ']' {
                    arrayoftables = false;
                }
            } else {
                openarr -= 1;
            }
        }
        if *item == '\n' {
            if openstring || multilinestr {
                if !multilinestr {
                    return Err(TomlDecodeError::new("Unbalanced quotes".to_string(), &original, i));
                }
                if (sl[i - 1] == '\'' || sl[i - 1] == '"') && sl[i - 2] == sl[i - 1] {
                    sl[i] = sl[i - 1];
                    if sl[i - 3] == sl[i - 1] {
                        sl[i - 3] = ' ';
                    }
                }
            } else if openarr > 0 {
                sl[i] = ' ';
            } else {
                beginline = true;
            }
            line_no += 1;
        } else if beginline && *item != ' ' && *item != '\t' {
            beginline = false;
            if !keygroup && !arrayoftables {
                if *item == '=' {
                    return Err(TomlDecodeError::new("Found empty keyname.".to_string(), &original, i));
                }
                keyname = 1;
                key.push(*item);
            }
        }
    }
    if keyname != 0 {
        return Err(TomlDecodeError::new("Key name found without value. Reached end of file.".to_string(), &original, s.len()));
    }
    if openstring {
        return Err(TomlDecodeError::new("Unterminated string found. Reached end of file.".to_string(), &original, s.len()));
    }
    let s = sl.iter().collect::<String>();
    let s: Vec<&str> = s.split('\n').collect();
    let mut multikey = None;
    let mut multilinestr = String::new();
    let mut multibackslash = false;
    let mut pos = 0;

    for (idx, line) in s.iter().enumerate() {
        if idx > 0 {
            pos += s[idx - 1].len() + 1;
        }

        decoder.embed_comments(idx, currentlevel);

        if !multilinestr.is_empty() && !multibackslash && !multilinestr.contains('\n') {
            line = line.trim();
        }
        if line.is_empty() && (multikey.is_none() || multibackslash) {
            continue;
        }
        if let Some(multikey) = multikey {
            if multibackslash {
                multilinestr.push_str(line);
            } else {
                multilinestr.push_str(line);
            }
            multibackslash = false;
            let closed = if multilinestr.starts_with('[') {
                line.ends_with(']')
            } else if multilinestr.len() > 2 {
                line.ends_with(multilinestr.chars().next().unwrap()) && line.chars().rev().nth(1) == Some(multilinestr.chars().next().unwrap()) && line.chars().rev().nth(2) == Some(multilinestr.chars().next().unwrap())
            } else {
                false
            };
            if closed {
                let value = decoder.load_value(&multilinestr)?;
                currentlevel.insert(multikey, value);
                multikey = None;
                multilinestr.clear();
            } else {
                let mut k = multilinestr.len() - 1;
                while k > 0 && multilinestr.chars().nth(k) == Some('\\') {
                    multibackslash = !multibackslash;
                    k -= 1;
                }
                if multibackslash {
                    multilinestr.pop();
                } else {
                    multilinestr.push('\n');
                }
            }
            continue;
        }
        if line.starts_with('[') {
            arrayoftables = false;
            if line.len() == 1 {
                return Err(TomlDecodeError::new("Opening key group bracket on line by itself.".to_string(), &original, pos));
            }
            if line.chars().nth(1) == Some('[') {
                arrayoftables = true;
                line = &line[2..];
                let splitstr = "]]";
            } else {
                line = &line[1..];
                let splitstr = "]";
            }
            let mut i = 1;
            let quotesplits = decoder._get_split_on_quotes(line);
            let mut quoted = false;
            for quotesplit in quotesplits {
                if !quoted && quotesplit.contains(splitstr) {
                    break;
                }
                i += quotesplit.matches(splitstr).count();
                quoted = !quoted;
            }
            line = line.split(splitstr).take(i).collect::<Vec<&str>>().join(splitstr);
            if line.len() < i + 1 || line.trim() != "" {
                return Err(TomlDecodeError::new("Key group not on a line by itself.".to_string(), &original, pos));
            }
            let groups: Vec<&str> = line.split('.').collect();
            let mut i = 0;
            while i < groups.len() {
                let group = groups[i].trim();
                if group.is_empty() {
                    return Err(TomlDecodeError::new("Can't have a keygroup with an empty name".to_string(), &original, pos));
                }
                if group.starts_with('"') || group.starts_with('\'') {
                    let groupstr = group.to_string();
                    let mut j = i + 1;
                    while !(groupstr.chars().next().unwrap() == groupstr.chars().last().unwrap() && groupstr.len() > 1) {
                        j += 1;
                        if j > groups.len() + 2 {
                            return Err(TomlDecodeError::new(format!("Invalid group name '{}'. Something went wrong.", groupstr), &original, pos));
                        }
                    }
                    groups[i] = &groupstr[1..groupstr.len() - 1];
                    groups.splice(i + 1..j, std::iter::empty());
                } else {
                    if !_groupname_re.is_match(group) {
                        return Err(TomlDecodeError::new(format!("Invalid group name '{}'. Try quoting it.", group), &original, pos));
                    }
                }
                i += 1;
            }
            currentlevel = &mut retval;
            for i in 0..groups.len() {
                let group = groups[i];
                if group.is_empty() {
                    return Err(TomlDecodeError::new("Can't have a keygroup with an empty name".to_string(), &original, pos));
                }
                if let Some(existing) = currentlevel.get(group) {
                    if i == groups.len() - 1 {
                        if implicitgroups.contains(&group) {
                            implicitgroups.retain(|g| g != group);
                            if arrayoftables {
                                return Err(TomlDecodeError::new("An implicitly defined table can't be an array".to_string(), &original, pos));
                            }
                        } else if arrayoftables {
                            currentlevel.insert(group.to_string(), vec![decoder.get_empty_table()]);
                        } else {
                            return Err(TomlDecodeError::new(format!("What? {} already exists? {}", group, currentlevel), &original, pos));
                        }
                    }
                } else {
                    currentlevel.insert(group.to_string(), decoder.get_empty_table());
                    if i == groups.len() - 1 && arrayoftables {
                        currentlevel.insert(group.to_string(), vec![decoder.get_empty_table()]);
                    }
                }
                currentlevel = currentlevel.get_mut(group).unwrap();
                if arrayoftables {
                    currentlevel = currentlevel.last_mut().unwrap();
                }
            }
        } else if line.starts_with("{") {
            if !line.ends_with("}") {
                return Err(TomlDecodeError::new("Line breaks are not allowed in inline objects".to_string(), &original, pos));
            }
            decoder.load_inline_object(line, currentlevel, multikey, multibackslash)?;
        } else if line.contains('=') {
            let ret = decoder.load_line(line, currentlevel, multikey, multibackslash)?;
            if let Some((mk, mls, mb)) = ret {
                multikey = Some(mk);
                multilinestr = mls;
                multibackslash = mb;
            }
        }
    }
    Ok(retval)
}

impl TomlDecoder {
    pub fn get_empty_inline_table(&self) -> DynamicInlineTableDict {
        /// Concrete sentinel subclass for inline tables.
        /// It is a subclass of _dict which is passed in dynamically at load time.
        /// It is also a subclass of InlineTableDict.
        struct DynamicInlineTableDict;

        DynamicInlineTableDict
    }
}

impl TomlDecoder {
    pub fn load_inline_object(
        &self,
        line: &str,
        currentlevel: &mut std::collections::HashMap<String, Box<dyn std::any::Any>>,
        multikey: bool,
        multibackslash: bool,
    ) -> Result<(), String> {
        let candidate_groups: Vec<&str> = line[1..line.len()-1].split(',').collect();
        let mut groups: Vec<&str> = Vec::new();
        let mut candidate_groups = candidate_groups;

        if candidate_groups.len() == 1 && candidate_groups[0].trim().is_empty() {
            candidate_groups.pop();
        }

        while !candidate_groups.is_empty() {
            let candidate_group = candidate_groups.remove(0);
            let parts: Vec<&str> = candidate_group.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err("Invalid inline table encountered".to_string());
            }
            let value = parts[1].trim();

            if (value.chars().next().unwrap() == value.chars().last().unwrap() && 
                (value.chars().next().unwrap() == '"' || value.chars().next().unwrap() == '\'')) ||
                value.chars().next().unwrap().is_digit(10) ||
                value == "true" || value == "false" ||
                (value.starts_with('[') && value.ends_with(']')) ||
                (value.starts_with('{') && value.ends_with('}')) {
                groups.push(candidate_group);
            } else if !candidate_groups.is_empty() {
                candidate_groups[0] = format!("{}{}", candidate_group, candidate_groups[0]);
            } else {
                return Err("Invalid inline table value encountered".to_string());
            }
        }

        for group in groups {
            let status = self.load_line(group, currentlevel, multikey, multibackslash);
            if status.is_ok() {
                break;
            }
        }
        Ok(())
    }
}

pub fn _load_unicode_escapes(v: &str, hexbytes: Vec<&str>, prefix: &str) -> Result<String, String> {
    let mut skip = false;
    let mut i = v.len() as isize - 1;
    while i > -1 && v.chars().nth(i as usize).unwrap() == '\\' {
        skip = !skip;
        i -= 1;
    }
    let mut v = v.to_string();
    for hx in hexbytes {
        if skip {
            skip = false;
            i = hx.len() as isize - 1;
            while i > -1 && hx.chars().nth(i as usize).unwrap() == '\\' {
                skip = !skip;
                i -= 1;
            }
            v.push_str(prefix);
            v.push_str(hx);
            continue;
        }
        let mut hxb = String::new();
        let mut i = 0;
        let hxblen = if prefix == "\\U" { 8 } else { 4 };
        hxb.push_str(&hx[0..hxblen].to_lowercase());
        if hxb.chars().any(|c| !c.is_digit(16)) {
            return Err(format!("Invalid escape sequence: {}", hxb));
        }
        if hxb.chars().nth(0).unwrap() == 'd' && hxb.chars().nth(1).unwrap().is_digit(8) {
            return Err(format!("Invalid escape sequence: {}. Only scalar unicode points are allowed.", hxb));
        }
        let codepoint = u32::from_str_radix(&hxb, 16).map_err(|_| "Invalid escape sequence".to_string())?;
        v.push(char::from_u32(codepoint).ok_or("Invalid unicode character".to_string())?);
        v.push_str(&hx[hxblen..]);
    }
    Ok(v)
}

pub fn _unescape(v: &str) -> String {
    /// Unescape characters in a TOML string.
    let mut i = 0;
    let mut backslash = false;
    let mut result = String::new();
    
    while i < v.len() {
        let c = v.chars().nth(i).unwrap();
        if backslash {
            backslash = false;
            if _escapes.contains(&c) {
                result.push_str(&_escape_to_escapedchars[c]);
            } else if c == '\\' {
                result.push(c);
            } else if c == 'u' || c == 'U' {
                i += 1;
            } else {
                panic!("Reserved escape sequence used");
            }
        } else if c == '\\' {
            backslash = true;
        } else {
            result.push(c);
        }
        i += 1;
    }
    result
}

pub fn _load_date(val: &str) -> Option<chrono::NaiveDateTime> {
    let mut microsecond = 0;
    let mut tz = None;
    if val.len() > 19 {
        if val.chars().nth(19) == Some('.') {
            let (subsecondval, tzval) = if val.ends_with('Z') {
                let subsecondval = &val[20..val.len()-1];
                (subsecondval, "Z")
            } else {
                let subsecondvalandtz = &val[20..];
                if let Some(splitpoint) = subsecondvalandtz.find('+') {
                    let subsecondval = &subsecondvalandtz[..splitpoint];
                    let tzval = &subsecondvalandtz[splitpoint..];
                    (subsecondval, tzval)
                } else if let Some(splitpoint) = subsecondvalandtz.find('-') {
                    let subsecondval = &subsecondvalandtz[..splitpoint];
                    let tzval = &subsecondvalandtz[splitpoint..];
                    (subsecondval, tzval)
                } else {
                    (subsecondvalandtz, "")
                }
            };
            if !tzval.is_empty() {
                tz = Some(TomlTz::new(tzval));
            }
            microsecond = (subsecondval.parse::<i64>().ok()? * 10_i64.pow(6 - subsecondval.len() as u32)) as u32;
        } else {
            tz = Some(TomlTz::new(&val[19..].to_uppercase()));
        }
    }
    if !val[1..].contains('-') {
        return None;
    }
    if val.len() == 10 {
        let d = chrono::NaiveDate::from_ymd(
            val[..4].parse().ok()?,
            val[5..7].parse().ok()?,
            val[8..10].parse().ok()?,
        );
        return Some(d.and_hms(0, 0, 0));
    } else {
        let d = chrono::NaiveDateTime::parse_from_str(val, "%Y-%m-%dT%H:%M:%S%.f").ok()?;
        return Some(d);
    }
}

use std::collections::HashMap;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct TomlDecoder {
    _dict: Box<dyn Any>, // Using Box<dyn Any> to represent Type[dict]
}

impl TomlDecoder {
    /// Creates a new TomlDecoder with the specified dictionary type.
    pub fn new(_dict: Box<dyn Any>) -> TomlDecoder {
        TomlDecoder { _dict }
    }

    /// Returns an empty table as a HashMap.
    pub fn get_empty_table(&self) -> HashMap<String, Box<dyn Any>> {
        // Assuming _dict can be cast to a HashMap
        self._dict.downcast_ref::<HashMap<String, Box<dyn Any>>>().unwrap().clone()
    }

    /// Returns a new inline table dictionary.
    pub fn get_empty_inline_table(&self) -> DynamicInlineTableDict {
        /// Concrete sentinel subclass for inline tables.
        /// It is a subclass of _dict which is passed in dynamically at load time.
        struct DynamicInlineTableDict;

        DynamicInlineTableDict
    }

    /// Loads an inline object from a string line into the current level.
    pub fn load_inline_object(
        &self,
        line: &str,
        currentlevel: &mut HashMap<String, Box<dyn Any>>,
        multikey: bool,
        multibackslash: bool,
    ) -> Result<(), String> {
        let candidate_groups: Vec<&str> = line[1..line.len()-1].split(',').collect();
        let mut groups: Vec<&str> = Vec::new();
        let mut candidate_groups = candidate_groups;

        if candidate_groups.len() == 1 && candidate_groups[0].trim().is_empty() {
            candidate_groups.pop();
        }

        while !candidate_groups.is_empty() {
            let candidate_group = candidate_groups.remove(0);
            let parts: Vec<&str> = candidate_group.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err("Invalid inline table encountered".to_string());
            }
            let value = parts[1].trim();

            if (value.chars().next().unwrap() == value.chars().last().unwrap() && 
                (value.chars().next().unwrap() == '"' || value.chars().next().unwrap() == '\'')) ||
                value.chars().next().unwrap().is_digit(10) ||
                value == "true" || value == "false" ||
                (value.starts_with('[') && value.ends_with(']')) ||
                (value.starts_with('{') && value.ends_with('}')) {
                groups.push(candidate_group);
            } else if !candidate_groups.is_empty() {
                candidate_groups[0] = format!("{}{}", candidate_group, candidate_groups[0]);
            } else {
                return Err("Invalid inline table value encountered".to_string());
            }
        }

        for group in groups {
            let status = self.load_line(group, currentlevel, multikey, multibackslash);
            if status.is_ok() {
                break;
            }
        }
        Ok(())
    }

    /// Splits a line on quotes and returns the resulting parts.
    pub fn _get_split_on_quotes(&self, line: &str) -> Vec<String> {
        let doublequotesplits: Vec<&str> = line.split('"').collect();
        let mut quoted = false;
        let mut quotesplits = Vec::new();
        
        if doublequotesplits.len() > 1 && doublequotesplits[0].contains('\'') {
            let mut singlequotesplits: Vec<&str> = doublequotesplits[0].split('\'').collect();
            let mut doublequotesplits = &doublequotesplits[1..];
            while singlequotesplits.len() % 2 == 0 && !doublequotesplits.is_empty() {
                singlequotesplits.last_mut().unwrap().to_string() += &format!("\"{}", doublequotesplits[0]);
                doublequotesplits = &doublequotesplits[1..];
                if singlequotesplits.last().unwrap().contains('\'') {
                    singlequotesplits = singlequotesplits[..singlequotesplits.len()-1]
                        .iter()
                        .chain(singlequotesplits.last().unwrap().split('\''))
                        .map(|s| s.trim())
                        .collect();
                }
            }
            quotesplits.extend(singlequotesplits);
        }
        
        for doublequotesplit in doublequotesplits {
            if quoted {
                quotesplits.push(doublequotesplit.to_string());
            } else {
                quotesplits.extend(doublequotesplit.split('\'').map(|s| s.trim().to_string()));
            }
            quoted = !quoted;
        }
        quotesplits
    }

    /// Loads a line into the current level.
    pub fn load_line(
        &self,
        line: &str,
        currentlevel: &mut HashMap<String, Box<dyn Any>>,
        multikey: Option<&str>,
        multibackslash: bool,
    ) -> Result<(String, String, bool), String> {
        let mut i = 1;
        let quotesplits = self._get_split_on_quotes(line);
        let mut quoted = false;
        for quotesplit in quotesplits {
            if !quoted && quotesplit.contains('=') {
                break;
            }
            i += quotesplit.matches('=').count();
            quoted = !quoted;
        }
        let pair: Vec<&str> = line.splitn(i, '=').collect();
        let mut strictly_valid = _strictly_valid_num(pair.last().unwrap_or(&"")).unwrap_or(false);
        let mut pair_last = pair.last().unwrap_or(&"").to_string();
        if _number_with_underscores.is_match(&pair_last) {
            pair_last = pair_last.replace('_', "");
        }
        while !pair_last.is_empty() && !matches!(pair_last.chars().next().unwrap(), ' ' | '\t' | '\'' | '"' | '[' | '{') && pair_last.trim() != "true" && pair_last.trim() != "false" {
            if pair_last.parse::<f64>().is_ok() {
                break;
            }
            if _load_date(&pair_last).is_some() {
                break;
            }
            if TIME_RE.is_match(&pair_last) {
                break;
            }
            i += 1;
            let prev_val = pair_last.clone();
            let pair: Vec<&str> = line.splitn(i, '=').collect();
            pair_last = pair.last().unwrap_or(&"").to_string();
            if prev_val == pair_last {
                return Err("Invalid date or number".to_string());
            }
            if strictly_valid {
                strictly_valid = _strictly_valid_num(pair_last.as_str()).unwrap_or(false);
            }
        }
        let pair = vec![
            pair[..pair.len() - 1].join("=").trim().to_string(),
            pair_last.trim().to_string(),
        ];
        let mut key = pair[0].clone();
        if key.contains('.') {
            if key.contains('"') || key.contains('\'') {
                let quotesplits = self._get_split_on_quotes(&key);
                let mut quoted = false;
                let mut levels = Vec::new();
                for quotesplit in quotesplits {
                    if quoted {
                        levels.push(quotesplit);
                    } else {
                        levels.extend(quotesplit.split('.').map(|s| s.trim().to_string()));
                    }
                    quoted = !quoted;
                }
                key = levels.pop().unwrap_or_default();
            } else {
                key = key.split('.').last().unwrap_or("").trim().to_string();
            }
        } else if (key.chars().next().unwrap() == '"' || key.chars().next().unwrap() == '\'') && (key.chars().last().unwrap() == key.chars().next().unwrap()) {
            key = _unescape(&key[1..key.len()-1]);
        }
        let (k, koffset) = self._load_line_multiline_str(&pair[1])?;
        let mut multilinestr = String::new();
        if k > -1 {
            let mut k = k;
            while k > -1 && pair[1].chars().nth(k + koffset) == Some('\\') {
                multibackslash = !multibackslash;
                k -= 1;
            }
            multilinestr = if multibackslash {
                pair[1][..pair[1].len()-1].to_string()
            } else {
                format!("{}\n", pair[1])
            };
            multikey = Some(&pair[0]);
        } else {
            let (value, vtype) = self.load_value(&pair[1], strictly_valid)?;
        }
        if currentlevel.contains_key(&key) {
            return Err("Duplicate keys!".to_string());
        } else {
            if let Some(multikey) = multikey {
                return Ok((multikey.to_string(), multilinestr, multibackslash));
            } else {
                currentlevel.insert(key, value);
            }
        }
        Ok((String::new(), String::new(), false))
    }

    /// Loads a multiline string from a given string.
    pub fn _load_line_multiline_str(&self, p: &str) -> (i32, usize) {
        let mut poffset = 0;
        if p.len() < 3 {
            return (-1, poffset);
        }
        if p.chars().next().unwrap() == '[' && (p.trim().chars().last().unwrap() != ']' && self._load_array_isstrarray(p)) {
            let mut newp: Vec<&str> = p[1..].trim().split(',').collect();
            while newp.len() > 1 && !newp.last().unwrap().starts_with('"') && !newp.last().unwrap().starts_with('\'') {
                let last = newp.pop().unwrap();
                let second_last = newp.pop().unwrap();
                newp.push(&format!("{},{}", second_last, last));
            }
            let last = newp.pop().unwrap();
            poffset = p.len() - last.len();
            p = last;
        }
        if !p.starts_with('"') && !p.starts_with('\'') {
            return (-1, poffset);
        }
        if p.chars().nth(1).unwrap() != p.chars().next().unwrap() || p.chars().nth(2).unwrap() != p.chars().next().unwrap() {
            return (-1, poffset);
        }
        if p.len() > 5 && p.chars().last().unwrap() == p.chars().next().unwrap() && p.chars().nth_back(1).unwrap() == p.chars().next().unwrap() && p.chars().nth_back(2).unwrap() == p.chars().next().unwrap() {
            return (-1, poffset);
        }
        (p.len() as i32 - 1, poffset)
    }

    /// Loads a value from a string.
    pub fn load_value(&self, v: &str, strictly_valid: bool) -> Result<Box<dyn Any>, String> {
        if v.is_empty() {
            return Err("Empty value is invalid".to_string());
        }
        if v == "true" {
            return Ok(Box::new(true));
        } else if v.to_lowercase() == "true" {
            return Err("Only all lowercase booleans allowed".to_string());
        } else if v == "false" {
            return Ok(Box::new(false));
        } else if v.to_lowercase() == "false" {
            return Err("Only all lowercase booleans allowed".to_string());
        } else if v.starts_with('"') || v.starts_with('\'') {
            let quotechar = v.chars().next().unwrap();
            let testv: Vec<&str> = v[1..].split(quotechar).collect();
            let mut triplequote = false;
            let mut triplequotecount = 0;
            let mut closed = false;

            let mut i = 0;
            while i < testv.len() {
                if testv[i].is_empty() {
                    if triplequote {
                        triplequotecount += 1;
                    } else {
                        closed = true;
                    }
                } else {
                    let mut oddbackslash = false;
                    let mut j = testv[i].len() as isize - 1;
                    while j >= 0 {
                        if testv[i].chars().nth(j as usize).unwrap() == '\\' {
                            oddbackslash = !oddbackslash;
                        }
                        j -= 1;
                    }
                    if !oddbackslash {
                        if closed {
                            return Err("Found tokens after a closed string. Invalid TOML.".to_string());
                        } else {
                            if !triplequote || triplequotecount > 1 {
                                closed = true;
                            } else {
                                triplequotecount = 0;
                            }
                        }
                    }
                }
                i += 1;
            }

            let mut v = v.to_string();
            if quotechar == '"' {
                let escapeseqs: Vec<&str> = v.split('\\').collect();
                let mut backslash = false;
                for i in escapeseqs.iter().skip(1) {
                    if i.is_empty() {
                        backslash = !backslash;
                    } else {
                        if !_escapes.contains(&i.chars().next().unwrap()) && (i.chars().next().unwrap() != 'u' && i.chars().next().unwrap() != 'U' && !backslash) {
                            return Err("Reserved escape sequence used".to_string());
                        }
                        if backslash {
                            backslash = false;
                        }
                    }
                }
                for prefix in ["\\u", "\\U"].iter() {
                    if v.contains(*prefix) {
                        let hexbytes: Vec<&str> = v.split(*prefix).collect();
                        v = _load_unicode_escapes(hexbytes[0], hexbytes[1..].to_vec(), *prefix);
                    }
                }
                v = _unescape(&v);
            }
            if v.len() > 1 && v.chars().nth(1).unwrap() == quotechar && (v.len() < 3 || v.chars().nth(1).unwrap() == v.chars().nth(2).unwrap()) {
                v = v[2..v.len()-2].to_string();
            }
            return Ok(Box::new(v[1..v.len()-1].to_string()));
        } else if v.starts_with('[') {
            return Ok(Box::new(self.load_array(v)?));
        } else if v.starts_with('{') {
            let inline_object = self.get_empty_inline_table();
            self.load_inline_object(v, inline_object);
            return Ok(Box::new(inline_object));
        } else if let Some(captures) = TIME_RE.captures(v) {
            let h: i32 = captures[1].parse().unwrap();
            let m: i32 = captures[2].parse().unwrap();
            let s: i32 = captures[3].parse().unwrap();
            let ms: i32 = captures[4].parse().unwrap_or(0);
            let time = chrono::NaiveTime::from_hms_milli(h, m, s, ms);
            return Ok(Box::new(time));
        } else {
            if let Some(parsed_date) = _load_date(v) {
                return Ok(Box::new(parsed_date));
            }
            if !strictly_valid {
                return Err("Weirdness with leading zeroes or underscores in your number.".to_string());
            }
            let mut itype = "int";
            let mut neg = false;
            let mut v = v.to_string();
            if v.starts_with('-') {
                neg = true;
                v = v[1..].to_string();
            } else if v.starts_with('+') {
                v = v[1..].to_string();
            }
            v = v.replace('_', "");
            let lowerv = v.to_lowercase();
            if v.contains('.') || (!v.contains('x') && (v.contains('e') || v.contains('E'))) {
                if v.contains('.') && v.split('.').nth(1).unwrap().is_empty() {
                    return Err("This float is missing digits after the point".to_string());
                }
                if !v.chars().next().unwrap().is_digit(10) {
                    return Err("This float doesn't have a leading digit".to_string());
                }
                let v = v.parse::<f64>().unwrap();
                itype = "float";
            } else if lowerv.len() == 3 && (lowerv == "inf" || lowerv == "nan") {
                let v = v.parse::<f64>().unwrap();
                itype = "float";
            }
            if itype == "int" {
                let v = i64::from_str_radix(&v, 0).unwrap();
                if neg {
                    return Ok(Box::new(-v));
                }
                return Ok(Box::new(v));
            }
        }
        Err("Invalid value".to_string())
    }

    /// Checks if a string is bounded.
    pub fn bounded_string(&self, s: &str) -> bool {
        if s.is_empty() {
            return true;
        }
        if s.chars().last().unwrap() != s.chars().next().unwrap() {
            return false;
        }
        let mut i = -2;
        let mut backslash = false;
        while s.len() as isize + i > 0 {
            if s.chars().nth((s.len() as isize + i) as usize).unwrap() == '\\' {
                backslash = !backslash;
                i -= 1;
            } else {
                break;
            }
        }
        !backslash
    }

    /// Checks if a string array is a string array.
    pub fn _load_array_isstrarray(&self, a: &str) -> bool {
        let a = a[1..a.len()-1].trim();
        if !a.is_empty() && (a.chars().next().unwrap() == '"' || a.chars().next().unwrap() == '\'') {
            return true;
        }
        false
    }

    /// Loads an array from a string.
    pub fn load_array(&self, a: &str) -> Vec<Option<Box<dyn Any>>> {
        let mut retval: Vec<Option<Box<dyn Any>>> = Vec::new();
        let a = a.trim();
        if !a[1..a.len()-1].contains('[') || a[1..a.len()-1].split('[').next().unwrap().trim() != "" {
            let strarray = self._load_array_isstrarray(a);
            let mut a = if !a[1..a.len()-1].trim().starts_with('{') {
                a[1..a.len()-1].split(',').map(|s| s.to_string()).collect::<Vec<String>>()
            } else {
                let mut new_a: Vec<String> = Vec::new();
                let mut start_group_index = 1;
                let mut end_group_index = 2;
                let mut open_bracket_count = if a.chars().nth(start_group_index).unwrap() == '{' { 1 } else { 0 };
                let mut in_str = false;

                while end_group_index < a.len() - 1 {
                    if a.chars().nth(end_group_index).unwrap() == '"' || a.chars().nth(end_group_index).unwrap() == '\'' {
                        if in_str {
                            let mut backslash_index = end_group_index - 1;
                            while backslash_index > 0 && a.chars().nth(backslash_index).unwrap() == '\\' {
                                in_str = !in_str;
                                backslash_index -= 1;
                            }
                        }
                        in_str = !in_str;
                    }
                    if !in_str && a.chars().nth(end_group_index).unwrap() == '{' {
                        open_bracket_count += 1;
                    }
                    if in_str || a.chars().nth(end_group_index).unwrap() != '}' {
                        end_group_index += 1;
                        continue;
                    } else if a.chars().nth(end_group_index).unwrap() == '}' && open_bracket_count > 1 {
                        open_bracket_count -= 1;
                        end_group_index += 1;
                        continue;
                    }

                    end_group_index += 1;
                    new_a.push(a[start_group_index..end_group_index].to_string());
                    start_group_index = end_group_index + 1;
                    while start_group_index < a.len() - 1 && a.chars().nth(start_group_index).unwrap() != '{' {
                        start_group_index += 1;
                    }
                    end_group_index = start_group_index + 1;
                }
                new_a
            };

            let mut b = 0;
            if strarray {
                while b < a.len() - 1 {
                    let mut ab = a[b].trim().to_string();
                    while !self.bounded_string(&ab) || (ab.len() > 2 && ab.chars().nth(0).unwrap() == ab.chars().nth(1).unwrap() && ab.chars().nth(2).unwrap() == ab.chars().nth(0).unwrap() && ab.chars().nth(ab.len() - 2).unwrap() != ab.chars().nth(0).unwrap() && ab.chars().nth(ab.len() - 3).unwrap() != ab.chars().nth(0).unwrap()) {
                        a[b] = format!("{}{}", a[b], a[b + 1]);
                        ab = a[b].trim().to_string();
                        if b < a.len() - 2 {
                            a.remove(b + 1);
                        } else {
                            a.pop();
                        }
                    }
                    b += 1;
                }
            }
        } else {
            let al: Vec<char> = a[1..a.len()-1].chars().collect();
            let mut a: Vec<String> = Vec::new();
            let mut openarr = 0;
            let mut j = 0;
            for i in 0..al.len() {
                if al[i] == '[' {
                    openarr += 1;
                } else if al[i] == ']' {
                    openarr -= 1;
                } else if al[i] == ',' && openarr == 0 {
                    a.push(al[j..i].iter().collect());
                    j = i + 1;
                }
            }
            a.push(al[j..].iter().collect());
        }
        for i in 0..a.len() {
            let trimmed = a[i].trim();
            if !trimmed.is_empty() {
                let (nval, _ntype) = self.load_value(trimmed);
                retval.push(nval);
            }
        }
        retval
    }

    /// Preserves a comment associated with a key.
    pub fn preserve_comment(&self, line_no: i32, key: &str, comment: &str, beginline: bool) {
        // Implementation omitted
    }

    /// Embeds comments into the current level.
    pub fn embed_comments(&self, idx: i32, currentlevel: &mut HashMap<String, Box<dyn Any>>) {
        // Implementation omitted
    }
}

#[derive(Debug, Clone)]
pub struct TomlTz {
    _raw_offset: String,
    _sign: i32,
    _hours: i32,
    _minutes: i32,
}

impl TomlTz {
    /// Creates a new TomlTz instance from a toml_offset string.
    pub fn new(toml_offset: &str) -> Self {
        let mut raw_offset = String::new();
        if toml_offset == "Z" {
            raw_offset = "+00:00".to_string();
        } else {
            raw_offset = toml_offset.to_string();
        }
        let sign = if raw_offset.chars().next().unwrap() == '-' { -1 } else { 1 };
        let hours = raw_offset[1..3].parse::<i32>().unwrap();
        let minutes = raw_offset[4..6].parse::<i32>().unwrap();

        TomlTz {
            _raw_offset: raw_offset,
            _sign: sign,
            _hours: hours,
            _minutes: minutes,
        }
    }

    /// Returns the raw offset as a string.
    pub fn get_raw_offset(&self) -> &String {
        &self._raw_offset
    }

    /// Returns the initialization arguments for the TomlTz instance.
    pub fn get_init_args(&self) -> (String,) {
        (self._raw_offset.clone(),)
    }

    /// Creates a deep copy of the TomlTz instance.
    pub fn deepcopy(&self) -> TomlTz {
        TomlTz {
            _raw_offset: self._raw_offset.clone(),
            _sign: self._sign,
            _hours: self._hours,
            _minutes: self._minutes,
        }
    }

    /// Returns the timezone name in UTC format.
    pub fn tzname(&self) -> String {
        format!("UTC{}", self._raw_offset)
    }

    /// Returns the UTC offset as a Duration.
    pub fn utcoffset(&self) -> chrono::Duration {
        chrono::Duration::hours(self._sign as i64 * self._hours as i64) +
        chrono::Duration::minutes(self._sign as i64 * self._minutes as i64)
    }

    /// Returns the daylight saving time offset, which is always zero.
    pub fn dst(&self) -> chrono::Duration {
        chrono::Duration::zero()
    }
}

pub fn _load_date(val: &str) -> Option<chrono::NaiveDateTime> {
    let mut microsecond = 0;
    let mut tz = None;
    if val.len() > 19 {
        if val.chars().nth(19) == Some('.') {
            let (subsecondval, tzval) = if val.ends_with('Z') {
                let subsecondval = &val[20..val.len()-1];
                (subsecondval, "Z")
            } else {
                let subsecondvalandtz = &val[20..];
                if let Some(splitpoint) = subsecondvalandtz.find('+') {
                    let subsecondval = &subsecondvalandtz[..splitpoint];
                    let tzval = &subsecondvalandtz[splitpoint..];
                    (subsecondval, tzval)
                } else if let Some(splitpoint) = subsecondvalandtz.find('-') {
                    let subsecondval = &subsecondvalandtz[..splitpoint];
                    let tzval = &subsecondvalandtz[splitpoint..];
                    (subsecondval, tzval)
                } else {
                    (subsecondvalandtz, "")
                }
            };
            if !tzval.is_empty() {
                tz = Some(TomlTz::new(tzval));
            }
            microsecond = (subsecondval.parse::<i64>().ok()? * 10_i64.pow(6 - subsecondval.len() as u32)) as u32;
        } else {
            tz = Some(TomlTz::new(&val[19..].to_uppercase()));
        }
    }
    if !val[1..].contains('-') {
        return None;
    }
    if val.len() == 10 {
        let d = chrono::NaiveDate::from_ymd(
            val[..4].parse().ok()?,
            val[5..7].parse().ok()?,
            val[8..10].parse().ok()?,
        );
        return Some(d.and_hms(0, 0, 0));
    } else {
        let d = chrono::NaiveDateTime::parse_from_str(val, "%Y-%m-%dT%H:%M:%S%.f").ok()?;
        return Some(d);
    }
}