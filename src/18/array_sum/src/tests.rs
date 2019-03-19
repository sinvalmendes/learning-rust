#[cfg(test)]
mod tests {
    use crate::main;
    use crate::Sum;

    #[test]
    fn test_populate_array() {
        let mut sum = Sum::new();
        sum.populate_array();
        assert_eq!(9999, sum.array.len());
    }

    #[test]
    fn test_sum_iter() {
        let result = main();
        println!("{:?}", result);
        let mut sum = Sum::new();
        sum.populate_array();
        assert_eq!(9999, sum.array.len());

        let sum_result = sum.sum_iter();
        assert_eq!(49995000, sum_result);
    }

    #[test]
    fn test_sum_concurrent() {
        let result = main();
        println!("{:?}", result);
        let mut sum = Sum::new();
        sum.populate_array();
        assert_eq!(9999, sum.array.len());

        let sum_result = sum.sum_concurrent();
        // assert_eq!(49995000, sum_result);
    }

}
