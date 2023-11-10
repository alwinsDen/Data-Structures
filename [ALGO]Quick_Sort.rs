//quick sort Rust : GPT 4 Turbo

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test1(){
        let mut tst = vec![9,8,7,6,5,4,3];
        quick_sort(&mut tst);
        assert_eq!(tst, vec![3,4,5,6,7,8,9]);
    }
}

fn quick_sort<T : Ord>(arr : &mut[T]) {
    let len : usize = arr.len();
    if len < 2 {
        return;
    }
    //get the partition arrays
    let p_key = partition(arr);
    quick_sort(&mut arr[0..p_key]);
    quick_sort(&mut arr[p_key+1..len]);
}

fn partition<T : Ord>(arr : &mut[T]) -> usize {
    let p_point = arr.len() - 1;
    //swap to the last
    arr.swap(p_point, arr.len() - 1);
    let mut c = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] < arr[arr.len() - 1] {
            arr.swap(c, j);
            c+=1;
        }
    }
    arr.swap(c, arr.len() - 1 );
    return c;
}
