use std::{fs, error::Error};
use json::{self, JsonValue};
use clap::Parser;

/// navigate through JSON files and search for elements with basic SQL-like queries

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// source file with json content
    file: String,

    /// if you want to skip the first n elements of the query result.
    #[arg(short, long, default_value_t = 0)]
    skip: u32,
    
    /// if you want to limit the number of elements of the query result.
    #[arg(short, long, default_value_t = u32::MAX)]
    limit: u32,

    /// if you want to get only certain properties of the object(s) in your file; you can specify more properties separated by a comma (no whitespaces). It will return everything if empty.
    #[arg(short = 'S', long, default_value_t = String::from(""))]
    select: String
}

fn open_(path: String) -> Result<JsonValue, Box<dyn Error>>{
    let read_file: String = fs::read_to_string(path)?;
    let obj: JsonValue = json::parse(&read_file)?;
    Ok(obj.clone())
}

fn skip_n(obj: JsonValue, skip: &u32) -> Result<JsonValue, Box<dyn Error>>{
    let mut result: JsonValue = JsonValue::new_array();
    let mut i:u32 = 0;
    for elem in obj.members(){
        if &i >= skip{
            result.push(elem.clone())?;
        }
        i += 1;
    }
    Ok(result)
}

fn limit_n(obj: JsonValue, limit: &u32) -> Result<JsonValue, Box<dyn Error>>{
    let mut result: JsonValue = JsonValue::new_array();
    let mut i:u32 = 1;
    for elem in obj.members(){
        if &i <= limit{
            result.push(elem.clone())?;
        }
        i += 1;
    }
    Ok(result)
}

fn find_recursive(obj: &JsonValue, field: &str) -> Option<(String, JsonValue)>{
    let f: Vec<&str> = field.split(".").collect();
    if f.len() == 1 {
        if !obj[field].is_null(){
            return Some((String::from(field), obj[field].clone()));
        }
    }
    else{
        if !obj[f[0]].is_null(){
            let x = find_recursive(&obj[f[0]], &field[(f[0].len()+1)..])?;
            let mut o: JsonValue = json::object!();
            o[x.0] = x.1;
            return Some((f[0].to_string(), o));
        }
    }
    None
}

fn select_f(obj: JsonValue, filters: &str) -> Result<JsonValue, Box<dyn Error>>{
    let mut temp_arr: JsonValue = json::array![];
    for elem in obj.members() {
        let mut temp_obj: JsonValue = json::object![];
        for field in filters.split(','){
            match find_recursive(elem, field){
                Some(x) => temp_obj[x.0] = x.1,
                _ => temp_obj.insert(field, "not found")?
            }
        }
        if !temp_obj.is_empty(){
            temp_arr.push(temp_obj)?;
        }
    }
    Ok(temp_arr)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Args::parse();
    let mut return_obj: JsonValue = open_(args.file)?;

    // getting the right shape
    if !return_obj.is_array(){
        return_obj = json::array![return_obj];
    }

    // Select filter
    return_obj = select_f(return_obj, &args.select)?;
    

    // execution of skip
    return_obj = skip_n(return_obj, &args.skip)?;


    //execution of limit 
    return_obj = limit_n(return_obj, &args.limit)?;

    println!("Result: \n {:#}", return_obj);
    
    Ok(())
}

#[test]
fn open_file(){
    assert_eq!(open_(String::from("/home/giulio/Rust/navi-json/tests/array.json")).unwrap(), json::parse(r#"[
        {"name":"Object1"},
        {"name":"Object2"},
        {"name":"Object3"},
        {"name":"Object4"},
        {"name":"Object5"},
        {"name":"Object6"}
    ]"#).unwrap());
}
#[test]
fn skip_test(){
    let obj: JsonValue = open_(String::from("/home/giulio/Rust/navi-json/tests/array.json")).unwrap();
    let obj1: JsonValue = obj.clone();
    assert_eq!(skip_n(obj, &2).unwrap(), json::parse(r#"[{"name":"Object3"},
    {"name":"Object4"},
    {"name":"Object5"},
    {"name":"Object6"}]"#).unwrap());
    assert_eq!(skip_n(obj1, &9).unwrap(), json::parse(r#"[]"#).unwrap());
}

#[test]
fn limit_test(){
    let obj: JsonValue = open_(String::from("/home/giulio/Rust/navi-json/tests/array.json")).unwrap();
    let obj1: JsonValue = obj.clone();
    assert_eq!(limit_n(obj, &2).unwrap(), json::parse(r#"[{"name":"Object1"},
    {"name":"Object2"}]"#).unwrap());
    assert_eq!(limit_n(obj1, &9).unwrap(), json::parse(r#"[
        {"name":"Object1"},
        {"name":"Object2"},
        {"name":"Object3"},
        {"name":"Object4"},
        {"name":"Object5"},
        {"name":"Object6"}
    ]"#).unwrap());
}

#[test]
fn select_test(){
    let obj: JsonValue = open_(String::from("/home/giulio/Rust/navi-json/tests/array.json")).unwrap();
    let obj1: JsonValue = obj.clone();
    assert_eq!(limit_n(obj, &2).unwrap(), json::parse(r#"[{"name":"Object1"},
    {"name":"Object2"}]"#).unwrap());
    assert_eq!(limit_n(obj1, &9).unwrap(), json::parse(r#"[
        {"name":"Object1"},
        {"name":"Object2"},
        {"name":"Object3"},
        {"name":"Object4"},
        {"name":"Object5"},
        {"name":"Object6"}
    ]"#).unwrap());
}
