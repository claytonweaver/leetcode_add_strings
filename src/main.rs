fn main() {
    let str = add_strings(String::from("5858"), String::from("383"));

    println!("{}", str);
}

pub fn add_strings(num1: String, num2: String) -> String {
    let n1 = build_int(num1);
    let n2 = build_int(num2);

    (n1 + n2).to_string()
}

fn build_int(num_str: String) -> i32 {
    //1000

    let mut num = 0;
    let mut i = num_str.len() - 1;
    let mut j = 1;
    let arr = num_str.as_bytes();
    let chars = num_str.chars();

    loop {
        let c: char = arr[i].try_into().unwrap();
        let n = c as i32 - 0x30;
        num += n * j;
        j *= 10;

        if i == 0 {
            break;
        }

        i -= 1;
    }

    num
}
