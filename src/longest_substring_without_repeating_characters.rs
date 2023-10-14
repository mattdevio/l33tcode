/// # Longest substring without repeating characters
///
/// Given a string `s`, find the length of the *longest substring* without repeating characters.

pub fn length_of_longest_substring(s: String) -> i32 {
    let c = s.chars();
    let mut vec: Vec<char> = vec![];
    let mut longest = 0;
    for char in c {
        if vec.contains(&char) {
            let len = vec.len() as i32;
            if len > longest {
                longest = len;
            }
            let pos = vec.iter().position(|x| x == &char).unwrap();
            vec.drain(..=pos);
        }
        vec.push(char);
    }
    let len = vec.len() as i32;
    if len > longest {
        longest = len;
    }
    longest
}

#[cfg(test)]
mod length_of_longest_substring_test {

    use super::*;

    #[test]
    fn test_a() {
        // Arrange
        let s = String::from("abcabcbb");
        let answer = 3;
        // Act
        let result = length_of_longest_substring(s);
        // Assert
        assert_eq!(result, answer);
    }
    #[test]
    fn test_b() {
        // Arrange
        let s = String::from("bbbbb");
        let answer = 1;
        // Act
        let result = length_of_longest_substring(s);
        // Assert
        assert_eq!(result, answer);
    }
    #[test]
    fn test_c() {
        // Arrange
        let s = String::from("pwwkew");
        let answer = 3;
        // Act
        let result = length_of_longest_substring(s);
        // Assert
        assert_eq!(result, answer);
    }
}
