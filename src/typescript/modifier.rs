use anyhow::{Context, Result};
use oxc_span::Span;
use std::fs;
use std::path::Path;
use tracing::info;

pub(crate) fn apply_modifications_to_file(
    path: &Path,
    source: &str,
    spans: Vec<Span>,
) -> Result<()> {
    let mut new_source = source.to_string();

    for span in spans {
        let (expanded_start, expanded_end) =
            expand_span_for_removal(&new_source, span.start as usize, span.end as usize);

        new_source.replace_range(expanded_start..expanded_end, "");
        info!("Removed expanded span {}..{}", expanded_start, expanded_end);
    }

    fs::write(path, new_source).with_context(|| format!("Failed to write file {:?}", path))?;
    info!("Updated file: {:?}", path);
    Ok(())
}

fn expand_span_for_removal(source: &str, start: usize, end: usize) -> (usize, usize) {
    let mut s = start;
    let mut e = end;

    let bytes = source.as_bytes();

    // 1. Try to consume trailing comma and whitespace until newline
    let mut temp_e = e;
    while temp_e < bytes.len() {
        match bytes[temp_e] {
            b' ' | b'\t' | b'\r' => temp_e += 1,
            b',' => {
                temp_e += 1;
                e = temp_e;
                // After comma, also consume whitespace until next real char or newline
                while e < bytes.len()
                    && (bytes[e] == b' ' || bytes[e] == b'\t' || bytes[e] == b'\r')
                {
                    e += 1;
                }
                break;
            }
            b'\n' => {
                e = temp_e + 1; // Include newline
                break;
            }
            _ => break,
        }
    }

    // 2. If no trailing comma, try leading comma
    if s == start && e == end {
        let mut temp_s = s;
        while temp_s > 0 {
            temp_s -= 1;
            match bytes[temp_s] {
                b' ' | b'\t' | b'\r' => {}
                b',' => {
                    s = temp_s;
                    break;
                }
                _ => break,
            }
        }
    }

    // 3. If it's a whole line (or multiple), consume leading whitespace as well
    let mut temp_s = s;
    while temp_s > 0 && (bytes[temp_s - 1] == b' ' || bytes[temp_s - 1] == b'\t') {
        temp_s -= 1;
    }

    // Only expand if we are at the start of the line or only whitespace precedes
    if temp_s == 0 || bytes[temp_s - 1] == b'\n' {
        s = temp_s;
    }

    (s, e)
}
