pub mod report;

#[cfg(test)]
mod tests {
    use crate::report::{RawReport, Safety};

    fn get_reports_by_str(str: &str) -> Vec<RawReport> {
        str.trim().lines().map(|l| l.parse().unwrap()).collect()
    }

    const TEST_DATA: &str = r"7 6 4 2 1
                                1 2 7 8 9
                                9 7 6 2 1
                                1 3 2 4 5
                                8 6 4 4 1
                                1 3 6 7 9";

    #[test]
    fn can_classify_report() {
        let mut raw_report = "7 6 4 2 1".parse::<RawReport>().unwrap();
        assert!(matches!(
            raw_report.process_report(false).safety,
            Safety::Safe
        ));
    }

    #[test]
    fn test_part_1() {
        let raw_reports = get_reports_by_str(TEST_DATA);
        let count_safe = raw_reports
            .into_iter()
            .map(|mut r| r.process_report(false))
            .filter(|r| matches!(r.safety, Safety::Safe))
            .count();
        assert_eq!(count_safe, 2);
    }

    #[test]
    fn test_part_2() {
        let raw_reports = get_reports_by_str(TEST_DATA);
        let count_safe = raw_reports
            .into_iter()
            .map(|mut r| r.process_report(true))
            .filter(|r| matches!(r.safety, Safety::Safe))
            .count();
        assert_eq!(count_safe, 4);
    }
}
