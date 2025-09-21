pub fn random_string(len: usize) -> String {
    let char_list = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";
    let mut ret = String::new();

    for _i in 0..len {
        match char_list.chars().nth(rand::random_range(0..=char_list.len()-1)) {
            Some(ch) => ret.push(ch),
            None => ret.push('c'),
        }
    }

    ret
}

#[cfg(test)]
mod tests {

    #[test]
    fn random_string() {
        let rand_string = super::random_string(5);

        println!("{}", rand_string);

        assert_ne!(rand_string, "test-".to_string());
    }
}
