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

    #[test]
    fn test_moving_average_convergence_divergence() {
        let data_set = vec![
            5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
        ];

        let result = moving_average_convergence_divergence(&data_set, 12, 26, 9);
        assert_eq!(None, result);

        let result = moving_average_convergence_divergence(&data_set, 3, 6, 2).unwrap();
        assert_eq!(8, result.macd.len());
        assert_eq!(
            vec![
                -1.5,
                -1.0178571428571432,
                -0.48596938775510257,
                -0.1194424198250732,
                0.02852327155351908,
                0.18443626539537084,
                0.32091429671097904,
                0.4309544083649852
            ],
            result.macd
        );
        assert_eq!(7, result.signal.len());
        assert_eq!(
            vec![
                -1.2589285714285716,
                -0.7436224489795923,
                -0.32750242954324627,
                -0.09015196214540272,
                0.09290685621511298,
                0.24491181654569036,
                0.36894021109188696
            ],
            result.signal
        );
    }

    #[test]
    fn test_bollinger_bands() {
        let data_set = vec![
            5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
        ];

        let result = bollinger_bands(&data_set, 20, 2.0);
        assert_eq!(None, result);

        let result = bollinger_bands(&data_set, 8, 2.0).unwrap();

        assert_eq!(6, result.middle_bound.len());
        assert_eq!(
            vec![3.0625, 2.875, 2.5625, 2.5625, 2.875, 3.3125],
            result.middle_bound
        );

        assert_eq!(6, result.upper_bound.len());
        assert_eq!(
            vec![
                6.395572906493346,
                5.906088913245535,
                4.589659342528357,
                4.589659342528357,
                5.206844763272204,
                5.758798223847616
            ],
            result.upper_bound
        );

        assert_eq!(6, result.lower_bound.len());
        assert_eq!(
            vec![
                -0.27057290649334576,
                -0.1560889132455352,
                0.535340657471643,
                0.535340657471643,
                0.5431552367277961,
                0.8662017761523844
            ],
            result.lower_bound
        );
    }

    #[test]
    fn test_relative_strength_index() {
        let data_set = vec![
            5.0, 6.0, 4.0, 2.0, 1.5, 1.0, 2.0, 3.0, 3.5, 3.5, 4.0, 4.5, 5.0,
        ];

        let result = relative_strength_index(&data_set, 14);
        assert_eq!(None, result);

        let result = relative_strength_index(&data_set, 8).unwrap();

        assert_eq!(5, result.len());
        assert_eq!(
            vec![
                56.852791878172596,
                56.852791878172596,
                59.17295654731064,
                61.256328819550575,
                63.16578540011347
            ],
            result
        );

        let data_set = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42, 45.84, 46.08, 45.89, 46.03,
            45.61, 46.28, 46.28, 46.00, 46.03,
        ];

        let result = relative_strength_index(&data_set, 14).unwrap();

        assert_eq!(3, result.len());
        assert_eq!(
            vec![70.53539393736207, 66.436571546019, 66.66146763681454],
            result
        );
    }
}
