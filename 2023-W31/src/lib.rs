pub fn is_anagram(s: &str, t: &str) -> bool {
    s.len() == t.len() && {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut t = t.chars().collect::<Vec<_>>();
        s.sort();
        t.sort();
        s == t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_anagram("barbie", "oppenheimer"), false);
        assert_eq!(is_anagram("aab", "abb"), false);
        assert!(is_anagram("race", "care"));
    }
}
