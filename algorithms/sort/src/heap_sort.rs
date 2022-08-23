

pub fn heap_sort<T>(vec: &mut Vec<T>) 
    where T: PartialEq + PartialOrd + Copy 
{
    let n = vec.len();
    if n == 0 {
        return;
    }

    let n = n-1;
    let mut el = 0;
    if n % 2 == 1 {
        el = (n-1)/2;
    } else {
        el = (n-2)/2;
    }

    sorting_tree(vec, el);
    swap(vec, 0, n);

    for i in (1..n).rev() {
        check_tree(vec, 0, i);
        swap(vec, 0, i);
    }


}



fn sorting_tree<T>(vec: &mut Vec<T>, el: usize) 
    where T: PartialEq + PartialOrd + Copy 
{
    if el == 0 {
        check_tree(vec, el, vec.len()-1);
        return
    }

    check_tree(vec, el, vec.len()-1);
    sorting_tree(vec, el-1);
}


fn check_tree<T>(vec: &mut Vec<T>, el: usize, n: usize)
    where T: PartialEq + PartialOrd + Copy 
{

    if (el*2+1) <= n {

        if (el*2+2) <= n {
            if vec[el*2+2] > vec[el*2+1] && vec[el*2+2] > vec[el] {
                swap(vec, el*2+2, el);
                check_tree(vec, el*2+2, n);

            } else if vec[el*2+1] > vec[el] {
                swap(vec, el*2+1, el);
                check_tree(vec, el*2+1, n);
            }

        } else {
            if vec[el*2+1] > vec[el] {
                swap(vec, el*2+1, el);
                check_tree(vec, el*2+1, n);
            }
        }
    
    } 
}

fn swap<T>(vec: &mut Vec<T>, a: usize, b: usize) 
    where T: PartialEq + PartialOrd + Copy 
{
    let tmp = vec[a];
    vec[a] = vec[b];
    vec[b] = tmp;
}




#[test]
fn test_heap_sort() {

    let mut v = vec![1, 3, 6];
    heap_sort(&mut v);
    let mut res = vec![1, 3, 6];
    res.sort();
    assert_eq!(v, res);



    let mut v = vec![1, 3, 6, 5];
    heap_sort(&mut v);
    let mut res = vec![1, 3, 6, 5];
    res.sort();
    assert_eq!(v, res);

    let mut v = vec![1, 3, 6, 5, 0, 2, 1];
    heap_sort(&mut v);
    let mut res = vec![1, 3, 6, 5, 0, 2, 1];
    res.sort();
    assert_eq!(v, res);
}