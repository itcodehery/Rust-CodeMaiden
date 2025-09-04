impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut str: String = String::new();
        let mut res: Vec<i32> = vec![];
        for i in 0..digits.len() {
            str.push_str(digits[i].to_string().as_str());
        }
        println!("{}", str);
        for (index, ch) in str.chars().enumerate() {
            res.push(ch.to_string().as_str().parse::u32());
        }

        res
    }
}
