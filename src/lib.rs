/// No check of word length, i.e., `output_line_len` must be greater or equal to every word len.
pub fn transform(input: &str, output_line_len: usize) -> String {
    let spaces = vec![' '; output_line_len];
    let spaces: String = spaces.into_iter().collect();

    let mut lines = Vec::with_capacity(input.chars().count() / 4);
    let mut words_iter = input.split(' ');

    let mut last_line_parts = Vec::with_capacity(output_line_len / 3);
    let mut last_line_len = 0;

    if let Some(first_word) = words_iter.next() {
        if !first_word.is_empty() {
            last_line_parts.push(first_word);
            last_line_len = last_line_parts[0].len();

            for new_word in words_iter {
                // In theory: `min_spaces_count = (last_line_parts.len() - 1) + 1`
                let min_spaces_count = last_line_parts.len();
                let new_word_chars_count = new_word.chars().count();

                if last_line_len + min_spaces_count + new_word_chars_count > output_line_len {
                    // `last_line_parts` is full. There we transform it to a string.

                    let new_line = process_last_line_parts(
                        &last_line_parts,
                        last_line_len,
                        output_line_len,
                        &spaces,
                    );
                    lines.push(new_line);

                    last_line_parts.clear();
                    last_line_len = 0;
                }

                last_line_parts.push(new_word);
                last_line_len += new_word_chars_count;
            }

            let new_line =
                process_last_line_parts(&last_line_parts, last_line_len, output_line_len, &spaces);
            lines.push(new_line);
        }
    }

    lines.join("\n")
}

fn process_last_line_parts(
    last_line_parts: &[&str],
    last_line_len: usize,
    output_line_len: usize,
    spaces: &str,
) -> String {
    let space_chars_count = output_line_len - last_line_len;

    if last_line_parts.len() == 1 {
        let spaces = &spaces[..space_chars_count];

        format!("{}{}", last_line_parts[0], spaces)
    } else {
        let gaps_count = last_line_parts.len() - 1;

        // The compiler will optimize it so that only a single instruction is emitted
        let (space_chars_per_gap, extra_space_chars_count) = (
            space_chars_count / gaps_count,
            space_chars_count % gaps_count,
        );
        let spaces = &spaces[..space_chars_per_gap];

        let mut new_line = String::with_capacity(output_line_len);

        for (i, last_line_part) in last_line_parts.iter().enumerate() {
            new_line.push_str(last_line_part);

            // If is not last
            if i < last_line_parts.len() - 1 {
                new_line.push_str(spaces);
            }

            // If is needed to add extra space
            if i < extra_space_chars_count {
                new_line.push(' ');
            }
        }

        new_line
    }
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn simple() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
             "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      ")
        ];

        for &(input, output_line_len, expected) in &test_cases {
            println!("input: '{:#?}'", input.split(' ').collect::<Vec<_>>());
            let output = transform(input, output_line_len);
            println!("output: '{:#?}'", output.split('\n').collect::<Vec<_>>());
            println!(
                "expected: '{:#?}'",
                expected.split('\n').collect::<Vec<_>>()
            );
            assert_eq!(output, expected);
        }
    }
}
