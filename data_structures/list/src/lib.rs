pub mod list;














#[cfg(test)]
mod tests {

    use crate::list::List;


    #[test]
    fn list() {
        let mut list = List::from(&vec![1, 2 ,3 ,4]);
        assert_eq!(list.to_vec(), vec![1, 2 ,3 ,4]);


        list.push_front(0);
        assert_eq!(list.to_vec(), vec![0, 1, 2 ,3 ,4]);


        let a = list.pop_front();
        assert_eq!(list.to_vec(), vec![1, 2 ,3 ,4]);
        assert_eq!(a, Some(0));


        list.back_to_front();
        assert_eq!(list.to_vec(), vec![4, 3 ,2 ,1]);


        let mut list = List::from(&vec![6, 2 ,3 ,4, 10, 2, 0, 1]);
        list.back_to_front();
        assert_eq!(list.to_vec(), vec![1, 0 ,2 ,10, 4, 3, 2, 6]);


        let mut list = List::<i32>::from(&vec![]);
        list.back_to_front();
        assert_eq!(list.to_vec(), vec![]);


        let mut list = List::from(&vec![1]);
        list.back_to_front();
        assert_eq!(list.to_vec(), vec![1]);
    }
}
