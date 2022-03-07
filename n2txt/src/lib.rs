#[allow(dead_code)]
#[allow(unused_variables)]
struct Split<'a> {
    val: &'a str,
    col: Vec<&'a str>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> Split<'a> {
    fn new(text: &'a str) -> Split<'a> {
        Split {
            val: text,
            col: Vec::new(),
        }
    }
    fn col(mut self) -> Vec<&'a str> {
        {
            const DIVIDER: usize = 3;
            let mut modulus = self.val.len()%DIVIDER;
            let group: i32 = self.val.len() as i32/DIVIDER as i32;

            if modulus==0 {
                for _i in 1..=group {
                    self.col.push(&self.val[modulus..modulus+DIVIDER]);
                    modulus+=DIVIDER;
                }
            }
            else {
                self.col.push(&self.val[0..modulus]);
                for _i in 1..=group {
                    self.col.push(&self.val[modulus..modulus+DIVIDER]);
                    modulus+=DIVIDER;
                }
            }
        }
        self.col
    }
}
