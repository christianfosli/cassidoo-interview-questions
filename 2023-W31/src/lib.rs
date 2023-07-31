pub fn is_anagram(s: &str, t: &str) -> bool {
    s.len() == t.len() && s.chars().all(|c| t.contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(is_anagram("barbie", "oppenheimer"), false);
        assert!(is_anagram("race", "care"));
    }
}
