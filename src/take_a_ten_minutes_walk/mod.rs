pub fn is_valid_walk(walk: &[char]) -> bool {
    if (walk.into_iter().len() != 10) {
        return false;
    }

    let mut x = 0;
    let mut y = 0;

    for each in walk {
        match each {
            'n' => y += 1,
            's' => y -= 1,
            'e' => x += 1,
            'w' => x -= 1,
            _ => panic!(),
        }
    }

    x == 0 && y == 0
}

#[cfg(test)]
mod tests {
    use super::is_valid_walk;

    #[test]
    fn sample_tests() {
        assert!(is_valid_walk(&[
            'n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e'
        ]));
        assert!(!is_valid_walk(&['w']));
        assert!(!is_valid_walk(&[
            'n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
        ]));
        assert!(!is_valid_walk(&[
            'e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's'
        ]))
    }
}
