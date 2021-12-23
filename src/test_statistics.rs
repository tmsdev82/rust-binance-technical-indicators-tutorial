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
}
