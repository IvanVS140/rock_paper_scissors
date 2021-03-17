mod functions;

#[allow(unused_imports)]
use functions::who_wins;

#[cfg(test)]
mod tests {
    #[test]
    fn round_draw() {
        assert_eq!(super::who_wins(0, 0), 0);
        assert_eq!(super::who_wins(1, 1), 0);
        assert_eq!(super::who_wins(2, 2), 0);
    }
    #[test]
    fn vega_wins() {
        assert_eq!(super::who_wins(0, 1), 1);
        assert_eq!(super::who_wins(1, 2), 1);
        assert_eq!(super::who_wins(2, 0), 1);
    }
    #[test]
    fn user_wins() {
        assert_eq!(super::who_wins(1, 0), 2);
        assert_eq!(super::who_wins(2, 1), 2);
        assert_eq!(super::who_wins(0, 2), 2);
    }
}
