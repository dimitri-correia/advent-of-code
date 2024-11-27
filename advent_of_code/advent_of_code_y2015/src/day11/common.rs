use std::collections::HashSet;

pub fn next_valid_password(p: Vec<u8>) -> Vec<u8> {
    let mut password = p;

    while !is_correct_password(&password) {
        inc_password(&mut password);
    }

    password
}

fn is_correct_password(pass: &Vec<u8>) -> bool {
    !pass.iter().any(|c| c == &b'i' || c == &b'o' || c == &b'l')
        && pass
            .windows(3)
            .any(|w| w[0] as u8 + 1 == w[1] as u8 && w[1] as u8 + 1 == w[2] as u8)
        && pass
            .windows(2)
            .filter_map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .collect::<HashSet<u8>>()
            .len()
            >= 2
}

pub fn inc_password(pass: &mut Vec<u8>) {
    let mut i = pass.len() - 1;
    while i != 0 {
        if pass[i] == b'z' {
            pass[i] = b'a';
            i -= 1;
        } else {
            pass[i] += 1;
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_correct_password() {
        assert!(!is_correct_password(&"hijklmmn".to_string().into_bytes()));
        assert!(!is_correct_password(&"abbceffg".to_string().into_bytes()));
        assert!(!is_correct_password(&"abbcegjk".to_string().into_bytes()));
        assert!(is_correct_password(&"abcdffaa".to_string().into_bytes()));
        assert!(is_correct_password(&"ghjaabcc".to_string().into_bytes()));
        assert!(!is_correct_password(
            &"abcdebbgjkbb".to_string().into_bytes()
        ));
    }
}
