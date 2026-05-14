pub fn format_minor(minor: i64) -> String {
    format_minor_with_symbol(minor, "\u{20BD}")
}

pub fn format_minor_with_symbol(minor: i64, symbol: &str) -> String {
    let abs = minor.unsigned_abs();
    let rub = abs / 100;
    let kop = abs % 100;
    let sign = if minor < 0 { "-" } else { "" };
    let rub_str = group_thousands(rub);
    format!("{}{},{:02}\u{00A0}{}", sign, rub_str, kop, symbol)
}

fn group_thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    let bytes = s.as_bytes();
    let len = bytes.len();
    for (i, b) in bytes.iter().enumerate() {
        let rem = len - i;
        if i > 0 && rem % 3 == 0 {
            out.push('\u{00A0}');
        }
        out.push(*b as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_basic() {
        assert_eq!(format_minor(0), "0,00\u{00A0}\u{20BD}");
        assert_eq!(format_minor(123), "1,23\u{00A0}\u{20BD}");
        assert_eq!(format_minor(123456), "1\u{00A0}234,56\u{00A0}\u{20BD}");
        assert_eq!(
            format_minor(1234567890),
            "12\u{00A0}345\u{00A0}678,90\u{00A0}\u{20BD}"
        );
    }

    #[test]
    fn formats_negative() {
        assert_eq!(format_minor(-50), "-0,50\u{00A0}\u{20BD}");
        assert_eq!(format_minor(-123456), "-1\u{00A0}234,56\u{00A0}\u{20BD}");
    }
}
