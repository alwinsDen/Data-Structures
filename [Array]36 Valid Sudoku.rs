use std::collections::HashMap;
const RADIX : u32 = 10;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        //first we will do a row check
        'jester:for rws in 0..9{
            let mut r_map = HashMap::new();
            let mut c_map = HashMap::new();
            for cls in 0..9{
                if board[rws][cls]!='.' {
                    //this is a much easier char to i32 conversion. 0x30 is the ASCII '0'
                    let cont_digi : i32 = (board[rws][cls]) as i32 - 0x30;
                    if r_map.contains_key(&cont_digi){
                        return false;
                    }
                    r_map.insert(cont_digi,0);   
                } 
                if board[cls][rws]!='.'{
                    let col_digi : i32 = (board[cls][rws]) as i32 - 0x30;
                    if c_map.contains_key(&col_digi) {
                        return false;
                    }
                    c_map.insert(col_digi, 0);
                }
                //checking within the 3x3 boxes
                let r_coor : usize = 3 * (rws/3); //this will always be a multiple of 3
                let c_coor : usize = 3 * (cls/3);
                let mut t_map = HashMap::new();
                for c_vls in r_coor..r_coor+3{
                    for r_vls in c_coor..c_coor+3{
                        if (board[c_vls][r_vls])!='.' {
                            let c_val : i32 = (board[c_vls][r_vls]) as i32 - 0x30;
                            if t_map.contains_key(&c_val) {
                                println!("{} {} {}",c_val,c_vls,r_vls);
                                return false;
                            }
                            t_map.insert(c_val,0);
                        }
                    }
                }
            }
        }
    true
    }
}
