use calamine::{open_workbook, Xlsx, Reader, Range};
use serde::{Serialize, Deserialize};
use serde_derive::{Serialize, Deserialize};
use serde_json::json;
use std::fs::File;
use std::io::Read;



#[derive(Serialize, Deserialize)]
struct Pokemonname{
    name:String,
    url:String
}

#[derive(Serialize, Deserialize)]
struct Pokemonlist {
    count:f64,
    next:String,
    previous:String,
    results:Vec<Pokemonname>
}



#[derive(Serialize, Deserialize)]
struct Pokemon {
    pokemon:String,
    encounterrate:f32
}

#[derive(Serialize, Deserialize)]
struct EncounterTime {
    always:Vec<Pokemon>,
    day:Vec<Pokemon>,
    night:Vec<Pokemon>

}

#[derive(Serialize, Deserialize)]
struct PokemonEncounters{
    route:String,
    encountertime:EncounterTime
}

#[derive(Serialize, Deserialize)]
struct EncounterMethod {

    encountermethod:String,
    routes:Vec<PokemonEncounters>
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    return input;

}


fn print_example_json() {


    let expected_json:&str = r#"

        {
            "encountermethod" : "Grass",
            "routes" : [
                {
                    "route" : "1",
                    "encounters" : {
                        "always" : [
                            {
                                "pokemon" : "Bidoof",
                                "encounterrate" : 0.1

                            },
                            {
                                "pokemon" : "Starly",
                                "encounterrate" : 0.1
                            }
                        ]
                        "day" : [
                            {
                                "pokemon" : "Bidoof",
                                "encounterrate" : 0.1
                            }
                        ]
                    }
                },
                {
                    "route" : "Viridian City"
                }

            ]
        }
        
    "#;

        println!("{}",expected_json );

}


fn get_pokemon_list () {
    
    let mut json_file = std::fs::File::open("all_pokemon.json").unwrap();
    let mut buff = String::new();
    json_file.read_to_string(&mut buff).unwrap();
    let json: Pokemonlist = serde_json::from_str(&buff).unwrap();
    let pokemons = json.results;

    // return pokemons;   -> COrrigir Erro
}


fn calamine_grass_extractor(column_number: u32) {

    
    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").unwrap();
    let sheets = excel.sheet_names().to_owned();
    
    if let Some(Ok(document)) = excel.worksheet_range(&sheets[0]) {

        for (column,rate) in (document.range((0,column_number),(28,column_number)).cells()).zip(document.range((0,0),(28,0)).used_cells()) {
            let pokemon_name = column.2;
            let pokemon_encounterrate = rate.2;

            println!("Pokemon name: {}, encounter rate: {}",pokemon_name, pokemon_encounterrate);


        }
                    
    }
    
}   



fn main() {
    let x:u32 = 1;
    //calamine_grass_extractor(x);
    get_pokemon_list();

}