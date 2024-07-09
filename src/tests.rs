#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_balance() {
        let address = "test_address".to_string();
        assert_eq!(get_balance(address), 0);
    }

    #[test]
    fn test_send_token() {
        let from = "sender".to_string();
        let to = "receiver".to_string();
        unsafe {
            let tokens = TOKENS.as_mut().unwrap();
            let sender = tokens.entry(from.clone()).or_insert(Token::new("IRCRC2".to_string()));
            sender.credit(100);
        }

        send_token(to.clone(), 50);
        assert_eq!(get_balance(from), 50);
        assert_eq!(get_balance(to), 50);
    }

    #[test]
    fn test_receive_token() {
        let from = "sender".to_string();
        let to = "receiver".to_string();
        unsafe {
            let tokens = TOKENS.as_mut().unwrap();
            let sender = tokens.entry(from.clone()).or_insert(Token::new("IRCRC2".to_string()));
            sender.credit(100);
        }

        receive_token(from.clone(), 50);
        assert_eq!(get_balance(from), 50);
        assert_eq!(get_balance(to), 50);
    }
}
