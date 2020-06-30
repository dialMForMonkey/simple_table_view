#[cfg(test)]
mod test {
    use crate::auxiliaries::{Pad, get_bigger};

    #[test]
    fn test_padding_right() {
        let mut a = String::with_capacity(10);
        let  stri = String::from("t");
        let  c = stri.chars();
        a.padding_right(c, '_');
        assert_eq!(a,"t_________");
    }

    #[test]
    fn test_padding_left() {
        let mut a = String::with_capacity(10);
        let  stri = String::from("t");
        let  c = stri.chars();
        a.padding_left(c, '_');
        assert_eq!(a,"_________t");
    }

    #[test]
    fn test_get_bigger() {
        let  a =get_bigger(&40, &45);
        assert_eq!(a, &45usize);
    }
}


trait Pad {
    fn padding_left(&mut self, value: Chars , repeat: char);
    fn padding_right(&mut self, value: Chars, repeat: char);
}

impl Pad for String {
    fn padding_left(&mut self, mut value: Chars, repeat: char) {
        let (count,_) =  value.size_hint();
        if count < self.capacity()   {
            let range = 0..self.capacity();
            let init_pos_for_write = self.capacity() - count;
            for i in range {
                if i >= init_pos_for_write {
                    let char_for_value = value.next().unwrap_or(repeat);
                    self.push(char_for_value)
                } else {
                    self.push(repeat);
                }
            }

        }

    }

    fn padding_right(&mut self, mut value: Chars, repeat: char) {
        let (count,_) =  value.size_hint();
        if count < self.capacity() {
            let range = 0..self.capacity();
            for _ in range {
                let char_for_value = value.next().unwrap_or(repeat);
                self.push(char_for_value)
            }
        }
    }
}

use std::str::Chars;

pub fn get_bigger<'a>(number_a: &'a usize, number_b: &'a usize) -> &'a usize {
    if number_a > number_b {
        number_a
    } else {
        number_b
    }
}

pub fn print_cell_with_size(value:String, size: usize)  {
    let mut new_string = String::with_capacity(size);
    new_string.padding_right(value.chars(),' ');


    print!("{} | ", new_string);
}

pub fn print_new_line() {
    println!();
}





