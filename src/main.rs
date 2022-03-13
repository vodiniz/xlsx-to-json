use calamine::{open_workbook, Xlsx, Reader, Range};
//use serde::{Serialize, Deserialize};
use serde_derive::{Serialize, Deserialize};
//use serde_json::json;
//use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};


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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Pokemon {
    pokemon:String,
    encounterrate:f64
}

impl PartialEq for Pokemon {
    fn eq(&self, other: &Pokemon) -> bool{
        other.pokemon == self.pokemon
    }
}

impl Eq for Pokemon {

}

impl Hash for Pokemon {
    fn hash<H>(&self, state: &mut H)
    where 
        H: Hasher
    {
        self.pokemon.hash(state);
    }
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

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

// fn get_input() -> String {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     return input;
// }

fn get_separate_encounters(day_vec: Vec<Pokemon>, night_vec: Vec<Pokemon>) -> (Vec<Pokemon>, Vec<Pokemon>, Vec<Pokemon>){

    let day_set:HashSet<Pokemon> = HashSet::from_iter(day_vec);
    let night_set = HashSet::from_iter(night_vec);

    let day_only_set = day_set.difference(&night_set)
        .cloned()
        .collect::<Vec<Pokemon>>();

    let night_only_set = night_set.difference(&day_set)
        .cloned()
        .collect::<Vec<Pokemon>>();

    let always_set = night_set.intersection(&day_set)
        .cloned()
        .collect::<Vec<Pokemon>>();

    (day_only_set, night_only_set, always_set)
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
    
    let mut json_file = std::fs::File::open("all_pokemon.json")
        .expect("Can't open json");
    let mut buff = String::new();
    json_file.read_to_string(&mut buff).unwrap();
    let json: Pokemonlist = serde_json::from_str(&buff)
        .expect("Couldn't Serialize Json");
    let pokemons = json.results;

    return pokemons;
}


fn calamine_grass_extractor() {

    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").expect("Couldn't open pokemon xlsx");
    let sheets = excel.sheet_names().to_owned();
    
    if let Some(Ok(document)) = excel.worksheet_range(&sheets[0]) {

        let mut column_number: u32 = 1;
        let mut cell_not_empty = true;

        'column_loop: while  cell_not_empty{

            let mut day_pokemon_vector = vec![];
            let mut night_pokemon_vector = vec![];

            let route_name = &(document
                .range((1,column_number),(1,column_number))[0]);

            if route_name[0].is_empty() {
                break 'column_loop;
            } 
                
            let route_name = route_name[0]
                .get_string()
                .unwrap();

            println!("{}",route_name);

            for (pokemon_column,rate_column) in (document.range((2,column_number),(13,column_number)).cells()).zip(document.range((2,0),(13,0)).used_cells()) {

                let pokemon_name = pokemon_column.2
                    . get_string()
                    .expect("Failed to read Pokemon name")  
                    .to_string();

                let pokemon_encounterrate = rate_column.2
                    .get_float()
                    .expect("Failed to read Pokemon encounter rate");

                let current_pokemon: Pokemon = Pokemon{pokemon: pokemon_name, encounterrate:pokemon_encounterrate};

                if day_pokemon_vector.contains(&current_pokemon){
                    let index = day_pokemon_vector.iter().position(|r| r == &current_pokemon)
                        .expect("Can't find pokemon index.");
                    day_pokemon_vector[index].encounterrate = (day_pokemon_vector[index].encounterrate)*2.0;
                } else {
                    day_pokemon_vector.push(current_pokemon);
                }
                
            }

            for (pokemon_column,rate_column) in (document.range((16,column_number),(27,column_number)).cells()).zip(document.range((2,0),(13,0)).used_cells()) {

                let pokemon_name = pokemon_column.2
                    . get_string()
                    .expect("Failed to read Pokemon name")
                    .to_string();

                let pokemon_encounterrate = rate_column.2
                    .get_float()
                    .expect("Failed to read Pokemon encounter rate");
                
                let current_pokemon: Pokemon = Pokemon{pokemon: pokemon_name, encounterrate:pokemon_encounterrate};

                if night_pokemon_vector.contains(&current_pokemon){
                    let index = night_pokemon_vector.iter().position(|r| r == &current_pokemon)
                        .expect("Can't find pokemon index.");
                    night_pokemon_vector[index].encounterrate = (night_pokemon_vector[index].encounterrate)*2.0;
                } else {
                    night_pokemon_vector.push(current_pokemon);
                }
            }

            let (only_day, only_night, always) = get_separate_encounters(day_pokemon_vector, night_pokemon_vector);
            
            column_number += 1;

            

        }
    }
                    
}
    
   

fn main() {

    let list = get_pokemon_list();
    calamine_grass_extractor();
}