use std::collections::HashMap;
use std::fs;

pub type NumT = i64;
pub type LevelRegister<'a> = HashMap<NumT, Vec<Vec<&'a NumT>>>;


pub fn read_from_file(path: &str) -> String {
    let e = format!("Error reading the file!: {}",  path);
    fs::read_to_string(path).expect(&e)
}

pub fn evaluate<'a>(key_num: NumT, num: &'a NumT, level_register: &'a LevelRegister) -> Option<Vec<Vec<&'a NumT>>> {
    let key = key_num - num;
    if let Some(x) = level_register.get(&key) {
        let mut out_lists: Vec<Vec<&NumT>> = Vec::new(); 
        for same_sum in x {
            let mut out = same_sum.clone();
            if out.iter().any(|x|std::ptr::eq(*x, num)) {
                continue;
            }
            out.push(num);
            out_lists.push(out);
        }
        return Some(out_lists);
    }
    None
}

pub fn update_cache<'a, 'b>(num: &'a NumT, register: &'b mut Vec<HashMap<i64, Vec<Vec<&'a i64>>>>) 
    {
    //insert to the first level of cache
    register[0].insert(*num, vec![vec![num]]);

    let len = register.len();

    for i in 0..len-1 {
        let (low, upper) = register[i..].split_at_mut(1);
        let low = &(low[0]);
        for (sum_to_point, ref mut low_elems) in low {
            let mut new_lists=  Vec::new();
            for same_sum_vec in *low_elems {
                let mut new_list = same_sum_vec.to_vec();
                if new_list.iter().any(|x|std::ptr::eq(*x, num)) {
                    continue;
                }
                new_list.push(num);
                new_lists.push(new_list);
            }
            if !new_lists.is_empty() {
                if let Some(already_in) = upper[0].get_mut(&(sum_to_point + num)) {
                    already_in.append(&mut new_lists);
                }
                else {
                    upper[0].insert(sum_to_point+num, new_lists);
                }
            }
        }
    }
}
