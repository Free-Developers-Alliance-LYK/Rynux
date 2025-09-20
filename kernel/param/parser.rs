//! Parameter parsing

/// Parameter parser
#[allow(dead_code)]
pub struct ParamParser<'a> {
    args: &'a str,
}

impl<'a> ParamParser<'a> {
    /// Create a new parameter parser
    pub fn new(args: &'a str) -> Self {
        ParamParser { args }
    }
}

impl<'a> Iterator for ParamParser<'a> {
    type Item = (&'a str, Option<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.args.is_empty() {
            return None;
        }

        let (rest, param, val) = next_arg(self.args);

        // if param is empty, val will also be dropped
        if param.is_empty() {
            if !rest.is_empty() {
                panic!("rest is not empty");
            }
            return None;
        }

        self.args = rest;

        Some((param, val))
    }
}

fn next_arg(mut args: &str) -> (&str, &str, Option<&str>) {
    args = args.trim_start();
    if args.is_empty() {
        return ("", "", None);
    }

    let mut in_quote = false;
    let mut quoted = false;

    if args.starts_with('"') {
        in_quote = true;
        quoted = true;
        args = &args[1..]
    }

    let bytes = args.as_bytes();
    let mut pair_end = bytes.len();
    let mut equals: Option<usize> = None;

    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        if b.is_ascii_whitespace() && !in_quote {
            pair_end = i;
            break;
        }

        if equals.is_none() && b == b'=' {
            equals = Some(i);
        }

        if b == b'"' {
            in_quote = !in_quote;
        }
        i += 1;
    }

    let (param, val) = match equals {
        None => {
            let mut end = pair_end;
            if quoted && end > 0 && args.ends_with('"') {
                end -= 1;
            }
            (&args[..end], None)
        }
        Some(eq) => {
            let param = &args[..eq];
            let mut val = &args[eq + 1..pair_end];

            // not includes " in val
            if val.starts_with('"') {
                val = &val[1..];
                if !val.is_empty() && val.as_bytes()[val.len() - 1] == b'"' {
                    val = &val[..val.len() - 1];
                } else if quoted && pair_end > 0 && args.as_bytes()[pair_end - 1] == b'"' {
                    if !val.is_empty() {
                        val = &val[..val.len() - 1];
                    }
                }
            }

            if val.is_empty() {
                (param, None)
            } else {
                (param, Some(val))
            }
        }
    };

    let rest = if pair_end < bytes.len() {
        let mut rest = &args[pair_end + 1..];
        rest = rest.trim_start();
        rest
    } else {
        ""
    };

    (rest, param, val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_arg() {
        let args = "foo=bar,bar2 baz=fuz wiz";
        let mut parser = ParamParser { args };
        let next = parser.next();
        assert_eq!(next, Some(("foo", Some("bar,bar2"))));
        let next = parser.next();
        assert_eq!(next, Some(("baz", Some("fuz"))));
        let next = parser.next();
        assert_eq!(next, Some(("wiz", None)));
        let next = parser.next();
        assert_eq!(next, None);
    }

    #[test]
    fn test_empty() {
        let args = "";
        let mut parser = ParamParser { args };
        let next = parser.next();
        assert_eq!(next, None);

        let args = " ";
        let mut parser = ParamParser { args };
        let next = parser.next();
        assert_eq!(next, None);

        let args = "\"";
        let mut parser = ParamParser { args };
        let next = parser.next();
        assert_eq!(next, None);

        // in quotes its raw value
        let args = "\"  ";
        let mut parser = ParamParser { args };
        let next = parser.next();
        assert_eq!(next, Some(("  ", None)));
        let next = parser.next();
        assert_eq!(next, None);
    }

    #[test]
    fn test_next_arg_no_value() {
        let args = "foo=";
        let mut parser = ParamParser { args };
        let next = parser.next();
        assert_eq!(next, Some(("foo", None)));
        let next = parser.next();
        assert_eq!(next, None);
    }

    #[test]
    fn test_next_no_param() {
        // if only have val, param is None, result is None
        let args = "=zoo";
        let mut parser = ParamParser { args };
        let next = parser.next();
        assert_eq!(next, None);
    }

    #[test]
    fn test_next_arg_quoted() {
        let args = "foo=\"bar,bar2\" baz=fuz \"wiz\"";
        let mut parser = ParamParser { args };
        let next = parser.next();
        assert_eq!(next, Some(("foo", Some("bar,bar2"))));
        let next = parser.next();
        assert_eq!(next, Some(("baz", Some("fuz"))));
        let next = parser.next();
        assert_eq!(next, Some(("wiz", None)));
        let next = parser.next();
        assert_eq!(next, None);

        let args = "\"bar,bar2 ";
        let mut parser = ParamParser { args };
        let next = parser.next();
        assert_eq!(next, Some(("bar,bar2 ", None)));
    }

    #[test]
    fn test_space() {
        let args =
            r#"foo="bar baz" hello=world debug="yes" single "standalone" keyonly= path="/a b/c""#;
        let mut parser = ParamParser { args };

        let next = parser.next();
        assert_eq!(next, Some(("foo", Some("bar baz"))));
        let next = parser.next();
        assert_eq!(next, Some(("hello", Some("world"))));

        let next = parser.next();
        assert_eq!(next, Some(("debug", Some("yes"))));
        let next = parser.next();
        assert_eq!(next, Some(("single", None)));

        let next = parser.next();
        assert_eq!(next, Some(("standalone", None)));

        let next = parser.next();
        assert_eq!(next, Some(("keyonly", None)));

        let next = parser.next();
        assert_eq!(next, Some(("path", Some("/a b/c"))));

        let next = parser.next();
        assert_eq!(next, None);
    }
}
