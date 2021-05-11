use regex::Regex;
use std::io::{self, BufRead, Write};

struct Row {
    cols: Vec<(String, usize)>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let in_handle = stdin.lock();

    // Keep each processed line in a row, owning the data as the column (String) and the display width (usize)
    // Display width is the width of the text as it will appear in a tty with colour support.
    let mut rows: Vec<Row> = Vec::new();
    let matcher = Regex::new(r"\{")?;
    let ansi_matcher = Regex::new(
        r"[\u001B\u009B][\[\]()#;?]*(?:(?:(?:[a-zA-Z\d]*(?:;[-a-zA-Z\d/#&.:=?%@~_]*)*)?\u0007)|(?:(?:\d{1,4}(?:;\d{0,4})*)?[\dA-PR-TZcf-ntqry=><~]))",
    )?;
    // Keep a list of max column widths
    let mut col_widths: Vec<usize> = Vec::new();
    let pad_char = ' ';

    // Process each incoming line and store the results in `rows`
    for line in in_handle.lines().map(|l| l.unwrap()) {
        let find_iter = matcher.split(&line);
        let mut cols = Vec::new();
        for (idx, col_str) in find_iter.enumerate() {
            let col_width = ansi_matcher.replace_all(col_str, "").len();
            if let Some(&col) = col_widths.get(idx) {
                col_widths[idx] = std::cmp::max(col, col_width);
            } else {
                col_widths.insert(idx, col_width);
            }

            cols.push((col_str.to_string(), col_width));
        }
        rows.push(Row { cols });
    }

    // Display each row with correct column padding
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    'print: for row in rows {
        {
            let cols = row.cols;
            for (col_idx, (col, col_width)) in cols.into_iter().enumerate() {
                let pad_width = col_widths[col_idx] - col_width;
                match write!(handle, "{}{}", col, {
                    let mut padding: String = String::with_capacity(pad_width + 1);
                    for _ in 0..pad_width {
                        padding.push(pad_char);
                    }
                    padding.push(pad_char);
                    padding
                }) {
                    Err(_) => break 'print,
                    _ => (),
                };
            }
        }
        match write!(handle, "{}", '\n') {
            Err(_) => break 'print,
            _ => (),
        };
    }

    Ok(())
}
