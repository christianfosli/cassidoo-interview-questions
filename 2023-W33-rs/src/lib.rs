pub fn faulty_keeb(s: &str) -> String {
    s.chars().fold(String::from(""), |acc, x| match x {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' => acc.chars().rev().collect(),
        _ => acc + &x.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(faulty_keeb("string"), "rtsng");
        assert_eq!(faulty_keeb("hello world!"), "w hllrld!");
    }
}
