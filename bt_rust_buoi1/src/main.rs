use std::io;

fn main() {
    println!();
    bai1(); 
    println!();
    bai2();
    println!();
}

fn bai1() {
    println!("Bai 1: ");

    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    let mut i = 0;
    let mut temp_arr;
    let mut compare = false;

    loop {
        // neu do dai org_arr < sub_arr thi khong the xet
        if sub_arr.len() > org_arr.len() {
            break;
        }

        // temp_arr duoc cat 3 phan tu 1 lan tu org_arr
        // set gia tri cho temp_arr
        temp_arr = &org_arr[i..=(i + 2)];

        if sub_arr == temp_arr {
            compare = true;
            break;
        }

        if i >= org_arr.len() - 3 {
            break;
        }

        i += 1;
    }


    println!("org_arr: {:?}", org_arr);
    println!("sub_arr: {:?}", sub_arr);
    if compare {
        println!("=>> sub_arr la mang con cua org_arr");
    } else {
        println!("=>> sub_arr KHONG la mang con sua org_arr");
    }
}

fn bai2() {
    println!("Bai 2: ");

    let str = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.");
    println!("Chuoi goc: \n{}\n", &str);
    println !("Nhap vao chuoi muon tim kiem: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop();

    println!("=>> So luong \"{}\" la: {}", &input, str.matches(&input).count());
}