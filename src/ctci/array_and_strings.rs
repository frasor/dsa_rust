// Implement is unique problem
pub fn is_unique(s: &str) -> bool {
    println!("{}", s);
    return false;
    
}

#[cfg(test)]
mod tests {
   use super::is_unique; 

    #[test]
    fn test_is_unique() {
        assert_eq!(is_unique("hello"), false);
    }
}
