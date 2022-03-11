use calamine::{open_workbook, Xlsx, Reader, Range};
use serde::{Serialize, Deserialize};
use serde_derive::{Serialize, Deserialize};
use serde_json::json;
use std::fs::File;
use std::io::Read;
use differ::{Differ, Tag};

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
    encounterrate:f64
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

fn get_day_only_pokemons<T>(day_vec: Vec<Pokemon>, night_vec: Vec<Pokemon>) -> Vec<Pokemon>{

    let day_only = day_vec.into_iter()
        .zip(night_vec.into_iter())
        .filter(|(p1, p2)| p1.pokemon != p2.pokemon).collect();

    return day_only;

}

fn print_example_json(){


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

fn get_pokemon_list () -> Vec<Pokemonname>{
    
    let mut json_file = std::fs::File::open("all_pokemon.json").unwrap();
    let mut buff = String::new();
    json_file.read_to_string(&mut buff).unwrap();
    let json: Pokemonlist = serde_json::from_str(&buff).unwrap();
    let pokemons = json.results;

    return pokemons;
}


fn calamine_grass_extractor() {

    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").unwrap();
    let sheets = excel.sheet_names().to_owned();
    
    if let Some(Ok(document)) = excel.worksheet_range(&sheets[0]) {

        let mut column_number: u32 = 1;
        let mut cell_not_empty = true;

        while  cell_not_empty{
            //let pokemon_encounters: PokemonEncounters;
            let mut day_pokemon_vector = vec![];
            let mut night_pokemon_vector = vec![];

            let route_name = &(document.range((1,column_number),(1,column_number))[0]);
            let route_name = route_name[0]     .get_string()   .unwrap();

            for (pokemon_column,rate_column) in (document.range((2,column_number),(13,column_number)).cells()).zip(document.range((2,0),(13,0)).used_cells()) {

                let pokemon_name = pokemon_column.2
                    . get_string()
                    .expect("Failed to read Pokemon name")  
                    .to_string();

                let pokemon_encounterrate = rate_column.2
                    .get_float()
                    .expect("Failed to read Pokemon encounter rate");

                //println!("Pokemon name: {}, encounter rate:{}",pokemon_name, pokemon_encounterrate);
                day_pokemon_vector.push(Pokemon{pokemon: pokemon_name, encounterrate:pokemon_encounterrate});
            }

            for (pokemon_column,rate_column) in (document.range((16,column_number),(27,column_number)).cells()).zip(document.range((2,0),(13,0)).used_cells()) {

                let pokemon_name = pokemon_column.2
                    . get_string()
                    .expect("Failed to read Pokemon name")
                    .to_string();

                let pokemon_encounterrate = rate_column.2
                    .get_float()
                    .expect("Failed to read Pokemon encounter rate");
                //println!("Pokemon name: {}, encounter rate:{}",pokemon_name, pokemon_encounterrate);
                
                night_pokemon_vector.push(Pokemon{pokemon: pokemon_name, encounterrate:pokemon_encounterrate});
            }

            println!("{:?}", day_pokemon_vector[10].pokemon);
            column_number += 1;
            cell_not_empty = false;
        }
    }
                    
}
    
   



fn main() {

    let list = get_pokemon_list();
    calamine_grass_extractor();
}