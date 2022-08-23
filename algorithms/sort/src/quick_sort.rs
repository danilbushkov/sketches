



pub fn quick_sort<T>(low: usize, high: usize, vec: &mut Vec<T>, comp: fn (T, T) -> bool) 
    where T: PartialEq + PartialOrd + Copy 
{
    
    let mut k = low;
    for i in low..high {
        if comp(vec[high], vec[i]) || vec[i] == vec[high] {
            let tmp = vec[i];
            vec[i] = vec[k];
            vec[k] = tmp;

            k += 1;
        }
    }
    let tmp = vec[high];
    vec[high] = vec[k];
    vec[k] = tmp;

    
    if k+1 < high {
        quick_sort(k+1, high, vec, comp);
    }

    if k > low && (k-low) != 1 {
        quick_sort(low, k-1, vec, comp);
    }
    
}


