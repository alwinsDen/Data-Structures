use std::cell::RefCell;
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let lts: Vec<&str> = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz",
        ];
        let mut a_str: RefCell<Vec<String>> = RefCell::new(vec![]);
        //defining a closure struct to support recursion(Closure doesnt support by default!)
        struct Btrack<'a> {
            f: &'a dyn Fn(&Btrack, String, usize),
        }
        if digits != "".to_string() {
            let backtrack = Btrack {
                f: &|backtrack, curr_str, idx| {
                    if curr_str.len() == digits.len() {
                        a_str.borrow_mut().push(curr_str.clone());
                    }
                    if idx < digits.len() {
                        let inx: usize = digits.chars().nth(idx).unwrap() as usize - 0x30;
                        for vls in lts[inx].chars() {
                            (backtrack.f)(
                                backtrack,
                                curr_str.clone() + &(vls.to_string()),
                                idx + 1,
                            );
                        }
                    }
                },
            };
            (backtrack.f)(&backtrack, "".to_string(), 0);
        }
        // println!("{:?}", a_str.borrow());
        return a_str.borrow().to_vec();
    }
}
