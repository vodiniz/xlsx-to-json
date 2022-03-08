use calamine::{open_workbook, Xlsx, Reader};
use serde_json::Value;
use serde::{Serialize, Deserialize};
use serde_derive::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Person {
    name:String,
    age: u8,
    is_male: bool
}



fn calamine() {

    
    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").unwrap();
    let sheets = excel.sheet_names().to_owned();
    
    if let Some(Ok(document)) = excel.worksheet_range(&sheets[0]) {

        for column in document.range((0,2),(28,2)).used_cells() {
            println!("Valor: {:?}", column);
            
        }
    }
}
        

fn json_test() {
    let json_str = r#"

    {
        "name" : "Vitor",
        "age" : 65,
        "is_male" : true
    }
    "#;

    let result = serde_json::from_str(json_str);
        if result.is_ok(){
            let p: serde_json::Value = result.unwrap();

            println!("The name is {}", p["name"].as_str().unwrap()); // not recommended can fail.

        } else {
            println!("Could not parse JSON");
        }
}

fn json_test2() {

    let json_str = r#"

    {
        "name" : "Vitor",
        "age" : 65,
        "is_male" : true
    }
    "#;

    let result = serde_json::from_str(json_str);
        if result.is_ok(){
            let p: Person = result.unwrap();

            println!("The name is {}", p.name);

        } else {
            println!("Could not parse JSON");
        }
}

fn main() {

    // calamine();
    json_test();

}