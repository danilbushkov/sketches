pub mod quick_sort;





#[cfg(test)]
mod tests {

    use crate::quick_sort::quick_sort as sort;


    #[test]
    fn it_works() {
        

        let mut v = vec![1];
        sort(0, v.len()-1, &mut v, |a, b| a > b);
        assert_eq!(v, vec![1]);

        let mut v = vec![2, 1];
        sort(0, v.len()-1, &mut v, |a, b| a > b);
        assert_eq!(v, vec![1, 2]);

        let mut v = vec![1, 2];
        sort(0, v.len()-1, &mut v, |a, b| a > b);
        assert_eq!(v, vec![1, 2]);

        let mut v = vec![1, 3, 2];
        sort(0, v.len()-1, &mut v, |a, b| a > b);
        assert_eq!(v, vec![1, 2, 3]);

        let mut v = vec![1, 3, 2];
        sort(0, v.len()-1, &mut v, |a, b| a < b);
        assert_eq!(v, vec![3, 2, 1]);

        let mut v = vec![11, 3, 2, 5, 1];
        sort(0, v.len()-1, &mut v, |a, b| a > b);
        assert_eq!(v, vec![1, 2, 3, 5, 11]);

        
        let mut v = vec![1, 17, 10, 2, 1, 5, 6, 7];
        sort(0, v.len()-1, &mut v, |a, b| a > b);
        assert_eq!(v, vec![1, 1, 2, 5, 6, 7, 10, 17]);


        let mut v = vec![1, 17, 10, 2, 1, 5, 6, 7, -100, 5, 8, 0, 6, 3 ,2 ,100, 0, -8];
        sort(0, v.len()-1, &mut v, |a, b| a > b);
        let mut res = vec![1, 17, 10, 2, 1, 5, 6, 7, -100, 5, 8, 0, 6, 3 ,2 ,100, 0, -8];
        res.sort();
        assert_eq!(v, res);

        let mut v = vec![1, 17, 10, 2, 1, 5, 6, 7, -100, 5, 8, 0, 6, 3 ,2 ,100, 0, -8];
        sort(0, v.len()-1, &mut v, |a, b| a < b);
        let mut res = vec![1, 17, 10, 2, 1, 5, 6, 7, -100, 5, 8, 0, 6, 3 ,2 ,100, 0, -8];
        res.sort_by(|a, b| b.cmp(a));
        assert_eq!(v, res);
    }
}
