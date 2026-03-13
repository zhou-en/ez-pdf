use crate::error::EzPdfError;

pub fn parse(_input: &str, _page_count: u32) -> Result<Vec<u32>, EzPdfError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: assert parse succeeds and returns the expected pages
    fn ok(input: &str, page_count: u32, expected: &[u32]) {
        let result = parse(input, page_count).expect("expected Ok");
        assert_eq!(result, expected, "input={:?} page_count={}", input, page_count);
    }

    // Helper: assert parse returns an InvalidSyntax error
    fn err_syntax(input: &str, page_count: u32) {
        match parse(input, page_count) {
            Err(EzPdfError::InvalidSyntax { .. }) => {}
            other => panic!(
                "expected InvalidSyntax for {:?}, got {:?}",
                input, other
            ),
        }
    }

    // Helper: assert parse returns a PageOutOfRange error for a specific page
    fn err_out_of_range(input: &str, page_count: u32, expected_page: u32) {
        match parse(input, page_count) {
            Err(EzPdfError::PageOutOfRange { page, total }) => {
                assert_eq!(page, expected_page, "wrong out-of-range page for {:?}", input);
                assert_eq!(total, page_count, "wrong total for {:?}", input);
            }
            other => panic!(
                "expected PageOutOfRange for {:?}, got {:?}",
                input, other
            ),
        }
    }

    #[test]
    fn single_page() {
        ok("3", 10, &[3]);
    }

    #[test]
    fn range() {
        ok("1-5", 10, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn list() {
        ok("1,3,5", 10, &[1, 3, 5]);
    }

    #[test]
    fn combined() {
        ok("1-3,5,7-9", 10, &[1, 2, 3, 5, 7, 8, 9]);
    }

    #[test]
    fn open_ended_from_middle() {
        ok("3-", 5, &[3, 4, 5]);
    }

    #[test]
    fn open_ended_from_first() {
        ok("1-", 5, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn single_page_equals_last() {
        ok("5", 5, &[5]);
    }

    #[test]
    fn out_of_range_single() {
        err_out_of_range("15", 10, 15);
    }

    #[test]
    fn out_of_range_in_list() {
        err_out_of_range("1,12,3", 10, 12);
    }

    #[test]
    fn out_of_range_end_of_range() {
        err_out_of_range("8-11", 10, 11);
    }

    #[test]
    fn invalid_syntax_alpha() {
        err_syntax("abc", 10);
    }

    #[test]
    fn invalid_syntax_empty() {
        err_syntax("", 10);
    }

    #[test]
    fn reversed_range_is_error() {
        err_syntax("7-3", 10);
    }

    #[test]
    fn zero_page_is_error() {
        err_syntax("0", 10);
    }

    #[test]
    fn zero_in_range_is_error() {
        err_syntax("0-3", 10);
    }

    // Display message tests for EzPdfError

    #[test]
    fn out_of_range_display_contains_page_count() {
        let err = EzPdfError::PageOutOfRange { page: 15, total: 10 };
        let msg = err.to_string();
        assert!(
            msg.contains("10"),
            "expected total page count in error message, got: {msg}"
        );
        assert!(
            msg.contains("15"),
            "expected page number in error message, got: {msg}"
        );
    }

    #[test]
    fn invalid_syntax_display_contains_input() {
        let err = EzPdfError::InvalidSyntax {
            input: "abc".to_string(),
            hint: "expected a number".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("abc"), "expected input in error message, got: {msg}");
    }
}
