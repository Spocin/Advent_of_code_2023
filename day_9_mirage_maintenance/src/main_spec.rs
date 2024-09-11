use crate::compute_next_num;

#[cfg(test)]
mod day_9_tests {
    use super::*;

    #[test]
    fn should_compute_to_18() {
        let input = vec![0,3,6,9,12,15];

        let res = compute_next_num(input);

        assert_eq!(res, 18);
    }

    #[test]
    fn should_compute_to_28() {
        let input = vec![1,3,6,10,15,21];

        let res = compute_next_num(input);

        assert_eq!(res, 28);
    }

    #[test]
    fn should_compute_to_68() {
        let input = vec![10,13,16,21,30,45];

        let res = compute_next_num(input);

        assert_eq!(res, 68);
    }
}