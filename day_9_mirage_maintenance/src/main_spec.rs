#[cfg(test)]
mod day_9_tests {
    use test::Bencher;
    use crate::{compute_next_num, main};

    #[test]
    fn test_1() {
        let input = vec![0,3,6,9,12,15];

        let res = compute_next_num(input);

        assert_eq!(res, -3);
    }

    #[test]
    fn test_2() {
        let input = vec![1,3,6,10,15,21];

        let res = compute_next_num(input);

        assert_eq!(res, 0);
    }

    #[test]
    fn test_3() {
        let input = vec![10,13,16,21,30,45];

        let res = compute_next_num(input);

        assert_eq!(res, 5);
    }
}