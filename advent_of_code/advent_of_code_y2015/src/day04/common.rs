pub fn find_hash_starting_with_pre(secret_key: &str, pre: &str) -> String {
    let mut number = 0;
    loop {
        let input = format!("{}{}", secret_key, number);
        let hash = format!("{:x}", md5::compute(input));

        if hash.starts_with(pre) {
            return number.to_string();
        }

        number += 1;
    }
}
