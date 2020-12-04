
pub fn question7<T: Ord>(a: &mut [T]){
    for i in 0..a.len() {
        if let Some((j, _)) = a.iter()
                                  .enumerate()
                                  .skip(i)
                                  .min_by_key(|x| x.1) {
                                      a.swap(i, j);
        }
    }
}
