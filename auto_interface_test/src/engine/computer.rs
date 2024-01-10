// use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use super::ValueType;

pub fn ent_compute(file: &String) ->Vec<f32> {
    let result = Command::new("build/ent/ent")
        .args(["-t", file])
        .output()
        .unwrap();

    let mut values: Vec<f32>= vec![0.0; ValueType::_LAST_ as usize];
    get_ent_value(&mut values, &result);
    values
}

fn get_ent_value(values:&mut Vec<f32>, result: &Output) {
    let res = String::from_utf8(result.stdout.clone()).unwrap();
    let v: Vec<&str> = res.split("\n").collect();
    let data = v[1].to_string();
    let v_data: Vec<&str> = data.split(",").collect();
    let _file_bytes_s = v_data[0].to_string();

    for i in 1 .. v_data.len() {
        values[i - 1] = str_2_f32(v_data[i].to_string());
    }
    // println!("*******************************");
    // println!("res: {:?}", v[0]);
    // println!("data: {:?}", v[1]);
    // println!("values: {:?}", values);
    // println!("*******************************");
}

fn str_2_f32(data: String) ->f32 {
    if data.contains("nan") {
        return 0.0;
    }
    data.parse::<f32>().unwrap()
}