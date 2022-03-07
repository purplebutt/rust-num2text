
pub mod caller {
    pub fn totxt(val: &str) -> String {
        if val=="0" {return "nol".to_string();}
        if val=="1000" {return "seribu".to_string();}
        let valx = splitter(val);
        let mut result = String::new();
        let mut i = valx.len() as i32;
        for v in valx {
            i-=1;
            result.push_str(&super::iterator::num2text(&v, val, i));
            result = rjmtk(&v, i, result, &val);
        }
        result
    }

    fn rjmtk (v: &str, pos: i32, mut last: String, val: &str)-> String {
        last = last.trim().to_string();
        if v=="000" {return last;}
        match pos {
            1 => { 
                // println!(" === {}",&val[0..1]);
                if val.len()!=4 || &val[0..1]!="1" {
                    last.push_str(" ribu ");
                    return last
                }
                last.push_str("ribu ");
                last 
            },
            2 => { last.push_str(" juta "); last },
            3 => { last.push_str(" miliar "); last },
            4 => { last.push_str(" triliun "); last },
            5 => { last.push_str(" kuadriliun "); last },
            _ => last
        }
    }

    fn splitter (val: &str) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        const DIVIDER: usize = 3;
        let length = val.len(); 
        let mut modulus = length%DIVIDER;
        let group: i32 = length as i32/DIVIDER as i32;
    
        if modulus==0 {
            for _i in 1..=group {
                result.push(&val[modulus..modulus+3]);
                modulus+=DIVIDER;
            }
        }
        else {
            result.push(&val[0..modulus]);
            for _i in 1..=group {
                result.push(&val[modulus..modulus+3]);
                modulus+=DIVIDER;
            }
        }
        result
    }
}

mod iterator {
    use super::converter::{belas, ratpul, totext};
    pub fn num2text(val: &str, rval: &str, g: i32) -> String {
        let mut result = String::new();
   
        let mut i = val.len();
        for v in val.chars() {
            if val.len()==3 {
                if i==2 && v=='1' {
                    result += belas(&val[1..val.len()]); break;
                }
                else {
                    result += &ratpul(v, i, rval, g);
                }
            }
            else if val.len()==2 {
                if i==2 && v=='1' {
                    result += belas(val); break;
                } 
                else {
                    result += &ratpul(v, i, rval, g);
                }
            }
            else {
                result += &totext(v, i, rval, g)
            }
            i-=1
        }
        result
    }
}

mod converter {
    pub fn totext(val: char, pos: usize, rval: &str, g: i32) -> &str {
        match val {
            '1' => { 
                if pos==3{"se"}
                else if rval.len()==4&&g!=0{"se"}
                else{"satu"}
            },
            '2' => "dua",
            '3' => "tiga",
            '4' => "empat",
            '5' => "lima",
            '6' => "enam",
            '7' => "tujuh",
            '8' => "delapan",
            '9' => "sembilan",
            _ => ""     // else return empty string
        }
    }

    pub fn belas(val: &str) -> &str {
        match val {
           "11" => "sebelas",
           "12" => "dua belas",
           "13" => "tiga belas",
           "14" => "empat belas",
           "15" => "lima belas",
           "16" => "enam belas",
           "17" => "tujuh belas",
           "18" => "delapan belas",
           "19" => "sembilan belas",
           _ => "sepuluh"
        }
    }

    pub fn ratpul(val: char, pos: usize, rval: &str, g: i32) -> String {
        let mut x = "";
        if pos==3 && val!='0' {
            if val=='1' {x="ratus "}else{x=" ratus "}
        }
        if pos==2 && val!='0' {x=" puluh "}
        let mut result = totext(val, pos, rval, g).to_string();
        result = result + x;
        result
    }
}