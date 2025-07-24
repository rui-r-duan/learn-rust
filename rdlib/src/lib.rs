pub fn substring_v1(s: &str, i: usize, j: usize) -> &str {
    let n = s.chars().count();
    if i > n || j > n || i > j {
        return "";
    }

    let byte_indices: Vec<usize> = s
        .char_indices()
        .enumerate()
        .filter(|(char_index, _)| *char_index == i || *char_index == j)
        .map(|(_, (byte_index, _))| byte_index)
        .collect();

    match byte_indices.len() {
        0 => "",
        1 => &s[byte_indices[0]..],
        2 => {
            let start_byte = byte_indices[0];
            let end_byte = byte_indices[1];
            &s[start_byte..end_byte]
        }
        _ => unreachable!(),
    }
}

pub fn substring_v2(s: &str, i: usize, j: usize) -> &str {
    // To get non-empty content, i and j must satisfy:
    //     i <= j && i < n
    // Its opposite is:
    //     i > j || i >= n
    let n = s.chars().count();
    if i >= n || i > j {
        return "";
    }

    let start = s.char_indices().nth(i).map(|(b, _)| b);
    let end = s.char_indices().nth(j).map(|(b, _)| b);

    match (start, end) {
        (Some(b), Some(e)) => &s[b..e], // i < j <= n-1
        (Some(b), None) => &s[b..],     // j >= n
        (None, None) => "",             // both i and j point to the last char
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring_v1() {
        let text = "Hello, 世界!";
        for (i, c) in text.chars().enumerate() {
            print!("{}:'{}', ", i, c);
        }
        println!();
        assert_eq!(substring_v1(text, 0, 9), "Hello, 世界");
        assert_eq!(substring_v1(text, 7, 9), "世界");
        assert_eq!(substring_v1(text, 2, 5), "llo");
        assert_eq!(substring_v1(text, 0, 10), "Hello, 世界!");
        assert_eq!(substring_v1(text, 9, 10), "!");
        assert_eq!(substring_v1(text, 10, 10), "");
        assert_eq!(substring_v1(text, 10, 11), "");
        assert_eq!(substring_v1(text, 5, 4), "");
        assert_eq!(substring_v1(text, 80, 90), "");
    }

    #[test]
    fn test_substring_v2() {
        let text = "Hello, 世界!";
        assert_eq!(substring_v2(text, 0, 9), "Hello, 世界");
        assert_eq!(substring_v2(text, 7, 9), "世界");
        assert_eq!(substring_v2(text, 2, 5), "llo");
        assert_eq!(substring_v2(text, 0, 10), "Hello, 世界!");
        assert_eq!(substring_v2(text, 9, 10), "!");
        assert_eq!(substring_v2(text, 10, 10), "");
        assert_eq!(substring_v2(text, 10, 11), "");
        assert_eq!(substring_v2(text, 5, 4), "");
        assert_eq!(substring_v2(text, 80, 90), "");
    }
}
