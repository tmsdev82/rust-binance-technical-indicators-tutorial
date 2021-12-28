#[cfg(test)]
mod test_statistics {
    use super::super::statistics::*;

    #[test]
    fn test_simple_moving_average() {
        let data_set = vec![5.0, 6.0, 4.0, 2.0];

        // test with window size 2
        let result = simple_moving_average(&data_set, 2).unwrap();
        assert_eq!(3, result.len());
        assert_eq!(vec![5.5, 5.0, 3.0], result);

        // test with window size 3
        let result = simple_moving_average(&data_set, 3).unwrap();
        assert_eq!(2, result.len());
        assert_eq!(vec![5.0, 4.0], result);

        // test with window size 4
        let result = simple_moving_average(&data_set, 4).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(vec![4.25], result);

        // test with window size bigger than data size, should return None
        let result = simple_moving_average(&data_set, 5);
        assert_eq!(None, result);
    }

    #[test]
    fn test_exponential_moving_average() {
        let data_set = vec![5.0, 6.0, 4.0, 2.0];

        let result = exponential_moving_average(&data_set, 2).unwrap();
        assert_eq!(3, result.len());
        assert_eq!(vec![5.5, 4.5, 2.8333333333333335], result);

        let result = exponential_moving_average(&data_set, 4).unwrap();
        assert_eq!(1, result.len());
        assert_eq!(vec![4.25], result);

        let result = exponential_moving_average(&data_set, 5);
        assert_eq!(None, result);

        let data_set = vec![
            22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29, 22.15, 22.39,
        ];

        let result = exponential_moving_average(&data_set, 10).unwrap();
        assert_eq!(3, result.len());
        assert_eq!(
            vec![22.220999999999997, 22.208090909090906, 22.241165289256195],
            result
        );
    }
}
