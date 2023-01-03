use super::tree::*;

use unfinished::*;
mod unfinished {
    use super::super::tree as finished;

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub enum Unfinished {
        AtomicSrc(String),
        List(UnfinishedList),
        Map(UnfinishedMap),
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct UnfinishedList {
        pub elements: Vec<finished::Node>,
        pub needs_newline_before_next_element: bool,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct UnfinishedMap {
        pub entries: Vec<finished::MapEntry>,
        pub pending_entry: UnfinishedMapEntry,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct UnfinishedMapEntry {
        pub key: String,
        pub has_space_after_key: bool,
        pub has_equal: bool,
    }

    impl Default for UnfinishedMapEntry {
        fn default() -> Self {
            Self {
                key: "".to_string(),
                has_space_after_key: false,
                has_equal: false,
            }
        }
    }

    impl UnfinishedMapEntry {
        pub fn reset(&mut self) {
            *self = Self::default();
        }
    }
}

pub fn parse(src: &str) -> Result<Node, usize> {
    let unexpected_eoi_err = Err(src.len());
    let mut stack = vec![Unfinished::Map(UnfinishedMap {
        entries: vec![],
        pending_entry: UnfinishedMapEntry {
            key: "".to_string(),
            has_space_after_key: false,
            has_equal: false,
        },
    })];
    let mut remaining = src.char_indices();
    let mut is_current_line_all_whitespace = true;

    while let Some((i, c)) = remaining.next() {
        if c == '\n' {
            is_current_line_all_whitespace = true;
        } else if !c.is_whitespace() {
            is_current_line_all_whitespace = false;
        }

        match stack.last_mut().expect("Stack should never be empty") {
            Unfinished::AtomicSrc(atomic_src) => match c {
                '\n' => return Err(i),
                '"' => {
                    atomic_src.push(c);
                    let top = Node::Atomic(AtomicValue(
                        parse_atomic_value(atomic_src).expect("Atomic value should be well-formed"),
                    ));
                    stack.pop().unwrap();
                    if let Some(return_val) =
                        reduce(&mut stack, top).expect(REDUCE_SHOULD_SUCCEED_MSG)
                    {
                        return Ok(return_val);
                    }
                }
                '\\' => {
                    atomic_src.push(c);
                    let Some((next_i, next_c)) = remaining.next() else {
                        return unexpected_eoi_err;
                    };
                    match next_c {
                        '\\' | '"' | 'n' => atomic_src.push(next_c),
                        'u' => {
                            for _ in 0..4 {
                                let Some((next_i, next_c)) = remaining.next() else {
                                    return unexpected_eoi_err;
                                };
                                if !next_c.is_ascii_hexdigit() {
                                    return Err(next_i);
                                }
                                atomic_src.push(next_c);
                            }
                        }
                        _ => return Err(next_i),
                    }
                }
                _other_char => atomic_src.push(c),
            },

            Unfinished::List(UnfinishedList {
                elements,
                needs_newline_before_next_element,
            }) => match c {
                ']' => {
                    let top = Node::List(List {
                        elements: elements.clone(),
                    });
                    stack.pop().unwrap();
                    if let Some(return_val) =
                        reduce(&mut stack, top).expect(REDUCE_SHOULD_SUCCEED_MSG)
                    {
                        return Ok(return_val);
                    }
                }
                '\n' => {
                    *needs_newline_before_next_element = false;
                }
                '"' => {
                    stack.push(Unfinished::AtomicSrc("\"".to_string()));
                }
                '{' => {
                    stack.push(Unfinished::Map(UnfinishedMap {
                        entries: vec![],
                        pending_entry: UnfinishedMapEntry {
                            key: "".to_string(),
                            has_space_after_key: false,
                            has_equal: false,
                        },
                    }));
                }
                '[' => {
                    stack.push(Unfinished::List(UnfinishedList {
                        elements: vec![],
                        needs_newline_before_next_element: true,
                    }));
                }
                '/' if is_current_line_all_whitespace => {
                    let Some((next_i, next_c)) = remaining.next() else {
                        return unexpected_eoi_err;
                    };
                    match next_c {
                        '/' => {
                            while let Some((_, next_c)) = remaining.next() {
                                if next_c == '\n' {
                                    break;
                                }
                            }
                        }
                        _ => return Err(next_i),
                    }
                }
                c if c.is_whitespace() => {}
                _ => return Err(i),
            },

            Unfinished::Map(UnfinishedMap {
                entries,
                pending_entry,
            }) => match c {
                '}' => {
                    if !pending_entry.key.is_empty() {
                        return Err(i);
                    }
                    let top = Node::Map(Map {
                        entries: entries.clone(),
                    });
                    stack.pop().unwrap();
                    if let Some(return_val) =
                        reduce(&mut stack, top).expect(REDUCE_SHOULD_SUCCEED_MSG)
                    {
                        return Ok(return_val);
                    }
                }
                '\n' => {
                    if !pending_entry.key.is_empty() {
                        return Err(i);
                    }
                }
                '=' => {
                    if pending_entry.has_equal {
                        return Err(i);
                    }
                    pending_entry.has_equal = true;
                }
                c if c.is_whitespace() => {
                    if !pending_entry.key.is_empty() {
                        pending_entry.has_space_after_key = true;
                    }
                }
                c if is_identifier_char(c) => {
                    let can_push = !pending_entry.has_space_after_key && !pending_entry.has_equal;
                    if !can_push {
                        return Err(i);
                    }
                    pending_entry.key.push(c);
                }
                '"' => {
                    if !pending_entry.has_equal {
                        return Err(i);
                    }
                    stack.push(Unfinished::AtomicSrc("\"".to_string()));
                }
                '{' => {
                    if !pending_entry.has_equal {
                        return Err(i);
                    }
                    stack.push(Unfinished::Map(UnfinishedMap {
                        entries: vec![],
                        pending_entry: UnfinishedMapEntry {
                            key: "".to_string(),
                            has_space_after_key: false,
                            has_equal: false,
                        },
                    }));
                }
                '[' => {
                    if !pending_entry.has_equal {
                        return Err(i);
                    }
                    stack.push(Unfinished::List(UnfinishedList {
                        elements: vec![],
                        needs_newline_before_next_element: true,
                    }));
                }
                '/' if is_current_line_all_whitespace => {
                    let Some((next_i, next_c)) = remaining.next() else {
                        return unexpected_eoi_err;
                    };
                    match next_c {
                        '/' => {
                            while let Some((_, next_c)) = remaining.next() {
                                if next_c == '\n' {
                                    break;
                                }
                            }
                        }
                        _ => return Err(next_i),
                    }
                }
                _ => return Err(i),
            },
        }
    }

    let last = stack.pop();
    let new_len = stack.len();
    match (last, new_len) {
        (
            Some(Unfinished::Map(UnfinishedMap {
                entries,
                pending_entry:
                    UnfinishedMapEntry {
                        key,
                        has_equal: _,
                        has_space_after_key: _,
                    },
            })),
            0,
        ) if key.is_empty() => Ok(Node::Map(Map { entries })),
        _ => unexpected_eoi_err,
    }
}

fn reduce(stack: &mut Vec<Unfinished>, top: Node) -> Result<Option<Node>, ()> {
    match stack.last_mut() {
        None => Ok(Some(top)),
        Some(Unfinished::AtomicSrc(_)) => Err(()),
        Some(Unfinished::List(UnfinishedList {
            elements,
            needs_newline_before_next_element,
        })) => {
            elements.push(top);
            *needs_newline_before_next_element = true;
            Ok(None)
        }
        Some(Unfinished::Map(UnfinishedMap {
            entries,
            pending_entry,
        })) => {
            if pending_entry.has_equal {
                entries.push(MapEntry {
                    key: Identifier::new(pending_entry.key.clone())
                        .expect("Pending key should always be valid"),
                    value: top,
                });

                pending_entry.reset();

                Ok(None)
            } else {
                Err(())
            }
        }
    }
}

/// Leading or trailing whitespace is forbidden.
fn parse_atomic_value(src_including_quotes: &str) -> Result<String, usize> {
    let unexpected_eoi_error = Err(src_including_quotes.len());
    let mut out = "".to_string();
    let mut remaining = src_including_quotes.chars().enumerate();

    loop {
        match remaining.next() {
            Some((_, '"')) => break,
            Some((_, c)) if c.is_whitespace() => {}
            _ => return unexpected_eoi_error,
        }
    }

    while let Some((i, c)) = remaining.next() {
        match c {
            '\n' => return Err(i),
            '"' => {
                if i != src_including_quotes.len() - 1 {
                    return Err(i);
                }
                return Ok(out);
            }
            '\\' => {
                let Some((next_i, next_c)) = remaining.next() else {
                    return unexpected_eoi_error;
                };
                match next_c {
                    '"' => out.push('"'),
                    '\\' => out.push('\\'),
                    'n' => out.push('\n'),
                    'u' => {
                        let mut hex = String::with_capacity(4);
                        for _ in 0..4 {
                            let Some((next_i, next_c)) = remaining.next() else {
                                return unexpected_eoi_error;
                            };
                            if !c.is_ascii_hexdigit() {
                                return Err(next_i);
                            }
                            hex.push(next_c);
                        }
                        let codepoint =
                            u32::from_str_radix(&hex, 16).expect("Hex code should always be valid");
                        let Some(encoded_char) = std::char::from_u32(codepoint) else {
                            return Err(next_i + 4);
                        };
                        out.push(encoded_char);
                    }
                    _ => return Err(next_i),
                }
            }
            _ => out.push(c),
        }
    }

    unexpected_eoi_error
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

const REDUCE_SHOULD_SUCCEED_MSG: &str = "Reduce should never fail, since we only ever push a node to the stack when the item under it is ready for it.";
