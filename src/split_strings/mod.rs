pub fn solution(s: &str) -> Vec<String> {
    let chars = s.chars().collect::<Vec<_>>();
    let chunks = chars.chunks(2);

    let chunks = chunks.map(|chunk| {
        let chunk = chunk.iter().collect::<String>();

        format!("{:_<2}", chunk)
    });

    chunks.collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
