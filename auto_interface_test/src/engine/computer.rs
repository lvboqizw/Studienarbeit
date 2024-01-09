// use std::fs;
// use std::path::PathBuf;
use std::process::{Command, Output};

use super::ValueType;

// struct Value {
//     // path: Box<PathBuf>,
//     value: Vec<f32>
// }

pub fn ent_compute(file: &String) {
    let result = Command::new("deps/ent/ent")
        .args(["-t", file])
        .output()
        .unwrap();

    let mut values: Vec<f32>= vec![0.0; ValueType::_LAST_ as usize];
    get_ent_value(&mut values, &result);
    println!("{:?}", values);
}

fn get_ent_value(values:&mut Vec<f32>, result: &Output) {
    let res = String::from_utf8(result.stdout.clone()).unwrap();
    let v: Vec<&str> = res.split("\n").collect();
    let data = v[1].to_string();
    // let v_data: Vec<&str> = data.split(",").collect();
    // let _file_bytes_s = v_data[1].to_string();
    println!("*******************************");
    println!("res: {:?}", res);
    println!("data: {:?}", data);
    println!("*******************************");

    // for i in 2 .. v_data.len() {
    //     values[i - 2] = str_2_f32(v_data[i].to_string());
    // }
}

fn str_2_f32(data: String) ->f32 {
    if data.contains("nan") {
        return 0.0;
    }
    data.parse::<f32>().unwrap()
}