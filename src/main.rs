use calamine::{open_workbook, Xlsx, Reader};
use serde_derive::{Serialize, Deserialize};
use std::fmt::Debug;
use std::fs::File;
use std::io::{Read,BufWriter};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::{fmt, vec};

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

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{} - {}%", self.pokemon, self.encounterrate*100.0)
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct PokemonTrade {
    pokemon: String,
    wantedPokemon: String
}
impl fmt::Debug for PokemonTrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pokemon: {}", self.pokemon);
        write!(f, "\n   Pokemon to trade: {}\n",self.wantedPokemon)
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct EggVendor {
    pokemon: String,
    price: String
}

impl fmt::Debug for EggVendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nPokemon: {}", self.pokemon);
        write!(f, "\n   Price: {}\n",self.price)
    }
}



#[derive(Serialize, Deserialize, Clone)]
struct PokemonGift {
    pokemon: String,
    gift_requirement: String
}

impl fmt::Debug for PokemonGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pokemon: {}", self.pokemon);
        write!(f, "\n   Gift Requirements: {}\n",self.gift_requirement)
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Fossil {
    pokemon: String,
    price: String,
    obs: String
}

impl fmt::Debug for Fossil {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pokemon: {}", self.pokemon);
        write!(f, "\n   Price: {}\n",self.price);
        if self.obs.len() > 0 {
            write!(f, "\n   Observation: {}\n",self.obs);
        }
        write!(f, "\n")
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct UnobtainablePokemon {
    pokemon:String,
}


#[derive(Serialize, Deserialize, Clone)]
struct EncounterTime {
    always:Vec<Pokemon>,
    day:Vec<Pokemon>,
    night:Vec<Pokemon>
    
}
impl fmt::Debug for EncounterTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "\nAlways:\n");
        for pokemon1 in &self.always {
            write!(f, "     {}\n", pokemon1);
        }
        write!(f, "\nDay:\n");
        for pokemon2 in &self.day {
            write!(f, "     {}\n", pokemon2);
        }
        write!(f, "\nNight:\n");
        for pokemon3 in &self.night {
            write!(f, "     {}\n", pokemon3);
        }

        write!(f,"\n")
    }

}

#[derive(Serialize, Deserialize, Clone)]
struct EncounterTimeWater {
    old_rod:Vec<Pokemon>,
    good_rod:Vec<Pokemon>,
    super_rod:Vec<Pokemon>,
    surfing:Vec<Pokemon>
    
}
impl fmt::Debug for EncounterTimeWater {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "\n   Old Rod:\n");
        for pokemon1 in &self.old_rod {
            write!(f, "     {}\n", pokemon1);
        }
        write!(f, "\n   Good Rod:\n");
        for pokemon2 in &self.good_rod {
            write!(f, "     {}\n", pokemon2);
        }
        write!(f, "\n   Super Rod:\n");
        for pokemon3 in &self.super_rod {
            write!(f, "     {}\n", pokemon3);
        }
        write!(f, "\n   Surfing:\n");
        for pokemon3 in &self.surfing {
            write!(f, "     {}\n", pokemon3);
        }

        write!(f,"\n")
    }

}
#[derive(Serialize, Deserialize, Clone)]
struct EggEncounter {
    mansion:Vec<EggVendor>,
    gamecorner:Vec<EggVendor>,
}
impl fmt::Debug for EggEncounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "\nCeladon Mansion: {:?}\n", self.mansion);
        write!(f, "\nCeladon Game Corner: {:?}\n",self.gamecorner)
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct FossilEncounter {
    trade:Vec<Fossil>,
    found:Vec<Fossil>,
}
impl fmt::Debug for FossilEncounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "\nTraded Fossil: {:?}\n", self.trade);
        write!(f, "\nFound Fossil: {:?}\n",self.found)
    }
}



#[derive(Serialize, Deserialize, Clone)]
struct RouteInfo<T>{
    route: String,
    encounter_info: T,
}
impl fmt::Debug for RouteInfo<EncounterTimeWater> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "\nRoute:{}   {:?}", self.route, self.encounter_info)
    }
}

impl fmt::Debug for RouteInfo<EncounterTime> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "\n Route:{} {:?}", self.route, self.encounter_info)
    }
}

impl fmt::Debug for RouteInfo<PokemonTrade> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "\nRoute:{} \n\n{:?}", self.route, self.encounter_info)
    }
}

impl fmt::Debug for RouteInfo<PokemonGift> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "\nRoute:{} \n\n{:?}", self.route, self.encounter_info)
    }
}

impl fmt::Debug for RouteInfo<FossilEncounter> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "\nRoute:{} \n\n{:?}", self.route, self.encounter_info)
    }
}

impl fmt::Debug for RouteInfo<EggEncounter> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "\nRoute:{} \n{:?}", self.route, self.encounter_info)
        
    }
}


 
#[derive(Serialize, Deserialize, Clone)]
struct Encounters<T> {

    encountermethod: String,
    routes: Vec<T>
}

impl fmt::Debug for Encounters<RouteInfo<EncounterTime>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        for route in &self.routes {
            write!(f, "{:?}", route);
            write!(f,"------------------------------");
        }
        write!(f,"\n")
    }
}

impl fmt::Debug for Encounters<RouteInfo<EncounterTimeWater>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        for route in &self.routes {
            write!(f, "{:?}", route);
            write!(f,"------------------------------");
        }
        write!(f,"\n")
    }
}

impl fmt::Debug for Encounters<RouteInfo<PokemonTrade>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        for all_routes in &self.routes {
            write!(f, "{:?}", all_routes);
            write!(f,"------------------------------");
        }
        write!(f,"\n")
    }
}
impl fmt::Debug for Encounters<RouteInfo<PokemonGift>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        for all_routes in &self.routes {
            write!(f, "{:?}", all_routes);
            write!(f,"------------------------------");
        }
        write!(f,"\n")
    }
}

impl fmt::Debug for Encounters<RouteInfo<FossilEncounter>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        for all_routes in &self.routes {
            write!(f, "{:?}", all_routes);
            write!(f,"------------------------------");
        }
        write!(f,"\n")
    }
}


impl fmt::Debug for Encounters<RouteInfo<EggEncounter>> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        for all_routes in &self.routes {
            write!(f, "{:?}", all_routes);
            write!(f,"------------------------------");
        }
        write!(f,"\n")
    }
}


#[derive(Serialize, Deserialize, Clone)]
struct AllEncounters<T> {

    encounters:Vec<T>
}


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


fn create_json (json: AllEncounters<Encounters<RouteInfo<EncounterTime>>>){

    let f = File::create("grass.json").expect("Unable to create file");
    let bw = BufWriter::new(f);
    serde_json::to_writer(bw, &json).expect("Failed writing :(");
}

fn get_pokemon_list () -> Vec<Pokemonname>{
    
    let mut json_file = std::fs::File::open("all_pokemon.json")
        .expect("Can't open json");
    let mut buff = String::new();
    json_file.read_to_string(&mut buff).unwrap();
    let json: Pokemonlist = serde_json::from_str(&buff)
        .expect("Couldn't Serialize Json");

    json.results
}
fn replace_galarian_alola(name:&str) -> String{

    if name.contains("-G"){
        let result = name.replace("-G", "-galar");
        result.to_string()
    } else if name.contains("-A"){
        let result = name.replace("-A", "-alola");
        result.to_string()
    } else {
        name.to_string()
    }
}


fn calamine_grass_extractor() -> Option<Encounters<RouteInfo<EncounterTime>>>{

    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").expect("Couldn't open pokemon xlsx");
    let sheets = excel.sheet_names().to_owned();
    
    if let Some(Ok(document)) = excel.worksheet_range(&sheets[0]) {

        let mut column_number: u32 = 1;
        let mut route_vector: Vec<RouteInfo<EncounterTime>> = vec![];
        

        loop {

            let mut day_pokemon_vector = vec![];
            let mut night_pokemon_vector = vec![];
            
            let route_name = &(document
                .range((1,column_number),(1,column_number))[0]);

            if route_name[0].is_empty() {
                break 
            } 
                
            let route_name = route_name[0]
                .get_string()
                .unwrap();

            for (pokemon_column,rate_column) in (document.range((2,column_number),(13,column_number)).cells()).zip(document.range((2,0),(13,0)).used_cells()) {

                let pokemon_name = pokemon_column.2
                    . get_string()
                    .expect("Failed to read Pokemon name");

                let pokemon_encounterrate = rate_column.2
                    .get_float()
                    .expect("Failed to read Pokemon encounter rate");

                let current_pokemon: Pokemon = Pokemon{pokemon: replace_galarian_alola(pokemon_name), encounterrate:pokemon_encounterrate};

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
                    .expect("Failed to read Pokemon name");

                let pokemon_encounterrate = rate_column.2
                    .get_float()
                    .expect("Failed to read Pokemon encounter rate");
                
                let current_pokemon: Pokemon = Pokemon{pokemon: replace_galarian_alola(pokemon_name), encounterrate:pokemon_encounterrate};


                if night_pokemon_vector.contains(&current_pokemon){
                    let index = night_pokemon_vector.iter().position(|r| r == &current_pokemon)
                        .expect("Can't find pokemon index.");
                    night_pokemon_vector[index].encounterrate = (night_pokemon_vector[index].encounterrate)*2.0;
                } else {
                    night_pokemon_vector.push(current_pokemon);
                }
            }


            let (only_day, only_night, always) = get_separate_encounters(day_pokemon_vector, night_pokemon_vector);
            let encounter_time:EncounterTime = EncounterTime{always:always, day: only_day, night: only_night};
            let route_info:RouteInfo<EncounterTime> = RouteInfo{route:route_name.to_string(), encounter_info:encounter_time};
            route_vector.push(route_info);
            

            column_number += 1;
        }

        let encounters:Encounters<RouteInfo<EncounterTime>> = Encounters{encountermethod: "Grass".to_string(), routes: route_vector};
        //let all_encounters:AllEncounters<Encounters<RouteInfo<EncounterTime>>> = AllEncounters{encounters: vec!(encounters)};
        Some(encounters)
    } else {
        None
    }
                    
}
    
fn get_route_name(column_number: u32, document: &calamine::Range<calamine::DataType>) -> Option<String>{

    let document_range = document.range((0, column_number), (0, column_number));
    let route_str = document_range[0][0].get_string();
    match route_str {
        Some(&ref route_str) => Some(route_str.to_string()),
        _ => None,
    }
}

fn get_cell_text(row_number:u32, column_number: u32, document: &calamine::Range<calamine::DataType>) -> Option<String>{

    let document_range = document.range((row_number, column_number), (row_number, column_number));
    let route_str = document_range[0][0].get_string();
    match route_str {
        Some(&ref route_str) => Some(route_str.to_string()),
        _ => None,
    }
}


fn calamine_water_extractor() -> Option<Encounters<RouteInfo<EncounterTimeWater>>>{
    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").expect("Couldn't open pokemon xlsx");
    let sheets = excel.sheet_names().to_owned();
    let mut water_encounters:Vec<RouteInfo<EncounterTimeWater>> = vec![];

    
    if let Some(Ok(document)) = excel.worksheet_range(&sheets[1]) {
        let mut column_number = 1;
        loop {
            match get_route_name(column_number, &document){
                Some(n) => water_encounters.push(get_rods_surf_list(n, column_number, &document)),
                None => break
            }
            column_number += 1;
        }
        let encounters:Encounters<RouteInfo<EncounterTimeWater>> = Encounters { encountermethod: "Fishing and Surfing".to_string(), routes: water_encounters };
        Some(encounters)

    } else {
        None
    }
}



fn check_pokemon_in_vec(mut vec:Vec<Pokemon>,pokemon:Pokemon) -> Vec<Pokemon>{

    if vec.contains(&pokemon){
        let index = vec.iter().position(|r| r == &pokemon)
            .expect("Can't find pokemon index.");
        vec[index].encounterrate += pokemon.encounterrate;
        vec
    } else {
        vec.push(pokemon);
        vec
    }
}

fn get_rods_surf_list(route_name: String, column_number: u32, document: &calamine::Range<calamine::DataType>) -> RouteInfo<EncounterTimeWater>{

    let old_rod = get_old_rod(column_number, &document);
    let good_rod = get_good_rod(column_number, &document);
    let super_rod = get_super_rod(column_number, &document);
    let surfing = get_surfing(column_number, &document);  

    let route_info = make_route_info(route_name, old_rod, good_rod, super_rod, surfing);
    route_info
    
}

fn get_old_rod(column_number: u32, document: &calamine::Range<calamine::DataType>) -> Vec<Pokemon>{

    let mut pokemon_vec = vec![];
    for (pokemon_column,rate_column) in (document.range((2,column_number),(3,column_number)).cells()).zip(document.range((2,0),(3,0)).used_cells()){
        let tuple = (pokemon_column.2.get_string(), rate_column.2.get_float());
        match tuple{
            (Some(string), Some(float)) => pokemon_vec = check_pokemon_in_vec(pokemon_vec, Pokemon{pokemon: replace_galarian_alola(string), encounterrate:float as f64}),
            _ => (),
        }
    }
    pokemon_vec
}

fn get_good_rod(column_number: u32, document: &calamine::Range<calamine::DataType>) -> Vec<Pokemon>{
    let mut pokemon_vec = vec![];
    for (pokemon_column,rate_column) in (document.range((5,column_number),(7,column_number)).cells()).zip(document.range((5,0),(7,0)).used_cells()){
        let tuple = (pokemon_column.2.get_string(), rate_column.2.get_float());
        match tuple{
            (Some(string), Some(float)) => pokemon_vec = check_pokemon_in_vec(pokemon_vec, Pokemon{pokemon: replace_galarian_alola(string), encounterrate:float as f64}),
            _ => (),
        }
    }
    pokemon_vec
}

fn get_super_rod(column_number: u32, document: &calamine::Range<calamine::DataType>) -> Vec<Pokemon>{
    let mut pokemon_vec = vec![];
    for (pokemon_column,rate_column) in (document.range((9,column_number),(13,column_number)).cells()).zip(document.range((9,0),(13,0)).used_cells()){
        let tuple = (pokemon_column.2.get_string(), rate_column.2.get_float());
        match tuple{
            (Some(string), Some(float)) => pokemon_vec = check_pokemon_in_vec(pokemon_vec, Pokemon{pokemon: replace_galarian_alola(string), encounterrate:float as f64}),
            _ => (),
        }
    }
    pokemon_vec
}

fn get_surfing(column_number: u32, document: &calamine::Range<calamine::DataType>) -> Vec<Pokemon>{
    let mut pokemon_vec = vec![];
    for (pokemon_column,rate_column) in (document.range((15,column_number),(19,column_number)).cells()).zip(document.range((15,0),(19,0)).used_cells()){
        let tuple = (pokemon_column.2.get_string(), rate_column.2.get_float());
        match tuple{
            (Some(string), Some(float)) => pokemon_vec = check_pokemon_in_vec(pokemon_vec, Pokemon{pokemon: replace_galarian_alola(string), encounterrate:float as f64}),
            _ => (),
        }
    }
    pokemon_vec
}

fn make_route_info(route_name: String, old_rod: Vec<Pokemon>, good_rod: Vec<Pokemon>, super_rod: Vec<Pokemon>, surfing: Vec<Pokemon>) -> RouteInfo<EncounterTimeWater>{
    
    let encounter_water: EncounterTimeWater = EncounterTimeWater{
        old_rod:old_rod, good_rod:good_rod, super_rod:super_rod, surfing:surfing
    };
    let route_info:RouteInfo<EncounterTimeWater> = RouteInfo { route: route_name, encounter_info: encounter_water };
    route_info
}

fn get_trade_pokemon(row_number: u32, document: &calamine::Range<calamine::DataType>) -> Option<(String, String, String)>{

    let range = document.range((row_number + 2, 0), (row_number + 2, 3));

    let tuple = (range[0][0].get_string(), range[0][1].get_string(), range[0][2].get_string());

    match tuple {
        (Some(a), Some(b), Some(c))
            => Some((a.to_string(),b.to_string(), c.to_string())),
            _ => None,
    }
}

fn calamine_trade_pokemon_extractor() -> Encounters<RouteInfo<PokemonTrade>>{
    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").expect("Couldn't open pokemon xlsx");
    let sheets = excel.sheet_names().to_owned(); 
    let mut route_vec: Vec<RouteInfo<PokemonTrade>> = vec![];

    if let Some(Ok(document)) = excel.worksheet_range(&sheets[2]){
        let mut row_number = 1;
        loop {

            match get_trade_pokemon(row_number, &document){
                Some(tuple) => {
                    let trade_pokemon: PokemonTrade = PokemonTrade{pokemon: replace_galarian_alola(&tuple.1) , wantedPokemon: replace_galarian_alola(&tuple.2)};
                    let route_info: RouteInfo<PokemonTrade> = RouteInfo { route: tuple.0, encounter_info: trade_pokemon};
                    route_vec.push(route_info);
                }
                None => break
            }
            row_number += 1;
        }

    }
    let encounters:Encounters<RouteInfo<PokemonTrade>> = Encounters { encountermethod: "Trade".to_string(), routes: route_vec };
    encounters
}

fn calamine_gift_pokemon_extractor() -> Encounters<RouteInfo<PokemonGift>>{
    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").expect("Couldn't open pokemon xlsx");
    let sheets = excel.sheet_names().to_owned(); 
    let mut route_vec: Vec<RouteInfo<PokemonGift>> = vec![];

    if let Some(Ok(document)) = excel.worksheet_range(&sheets[3]){
        let mut row_number = 1;
        loop {
            match get_trade_pokemon(row_number, &document){
                Some(tuple) => {
                    let trade_pokemon: PokemonGift = PokemonGift{pokemon: replace_galarian_alola(&tuple.0) , gift_requirement: replace_galarian_alola(&tuple.2)};
                    let route_info: RouteInfo<PokemonGift> = RouteInfo { route: tuple.1, encounter_info: trade_pokemon};
                    route_vec.push(route_info);
                }
                None => break
            }
            row_number += 1;
        }

    }
    let encounters:Encounters<RouteInfo<PokemonGift>> = Encounters { encountermethod: "Trade".to_string(), routes: route_vec };
    encounters
}

fn get_egg_vendor_pokemon( document: &calamine::Range<calamine::DataType>) -> Option<(Vec<EggVendor>, Vec<EggVendor>)>{

    let mut egg_vendor_mansion:Vec<EggVendor> = vec![];
    let mut egg_vendor_gamecorner:Vec<EggVendor> = vec![];
    let mut column_number = 0;

    while column_number < 3{
        let shard_type = get_cell_text(2, column_number, document);
        match shard_type{
            Some(shard_str) => {
                for pokemon_cell in document.range((3,column_number),(8,column_number)).used_cells(){
                    if pokemon_cell.0 > 3 {
                        break;
                    }
                    match pokemon_cell.2.get_string(){
                        Some(pokemon) => {
                            let pokemon_egg: EggVendor = EggVendor { pokemon: replace_galarian_alola(pokemon), price: format!("1 {}", shard_str)};
                            egg_vendor_mansion.push(pokemon_egg);

                        },
                        None => break
                    }
                }
            },
            None => ()
        }
    column_number += 1;
    }

    for pokemon_cell in document.range((2,5),(15,5)).used_cells(){
        match pokemon_cell.2.get_string(){
            Some(pokemon) => {
                let pokemon_egg: EggVendor = EggVendor { pokemon: replace_galarian_alola(pokemon), price: "$100 000 each, additional $100 000 for Shiny form.".to_string()};
                egg_vendor_gamecorner.push(pokemon_egg);
            },
            None => break
        }
    

    }
    Some((egg_vendor_mansion, egg_vendor_gamecorner))
}


fn calamine_egg_vendor_extractor() -> Encounters<RouteInfo<EggEncounter>>{
    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").expect("Couldn't open pokemon xlsx");
    let sheets = excel.sheet_names().to_owned(); 
    let mut route_vec: Vec<RouteInfo<EggEncounter>> = vec![];

    if let Some(Ok(document)) = excel.worksheet_range(&sheets[4]){
        match get_egg_vendor_pokemon(&document){
            Some(tuple) => {
                let egg_encounter:EggEncounter = EggEncounter { mansion: tuple.0, gamecorner: tuple.1 };
                let route_info_mansion: RouteInfo<EggEncounter> = RouteInfo{ route: "Celadon".to_string(), encounter_info:egg_encounter };
                route_vec.push(route_info_mansion);
                //let route_info_gamecorner: RouteInfo<EggVendor> = RouteInfo{ route: "Celadon's Mansion".to_string(), encounter_info:tuple.1 };
            }
            None => (),
        
        }
    }
    let encounters:Encounters<RouteInfo<EggEncounter>> = Encounters { encountermethod: "Egg Vendor".to_string(), routes: route_vec };
    encounters
}

fn get_fossil_pokemon( document: &calamine::Range<calamine::DataType>) -> FossilEncounter{

    let mut trade_fossil_list:Vec<Fossil> = vec![];
    let mut found_fossil_list:Vec<Fossil> = vec![];
    let mut column_number = 0;

    while column_number < 4{
        let shard_type = get_cell_text(2, column_number, document);
        
        match shard_type{
            Some(shard_str) => {
                for pokemon_cell in document.range((3,column_number),(5,column_number)).used_cells(){
                    match pokemon_cell.2.get_string(){
                        Some(pokemon) => {
                            let pokemon_split: Vec<_> = pokemon.split([' ', ','].as_ref()).collect();

                            let pokemon_fossil: Fossil = Fossil {
                                pokemon: replace_galarian_alola(pokemon_split[0]),
                                price: format!("{} {}s", pokemon_split[1].replace("(", ""), shard_str.replace(":", "")), obs:"".to_string()};

                            println!("{:?}", pokemon_fossil);
                            trade_fossil_list.push(pokemon_fossil);

                        },
                        None => break
                    }
                }
            },
            None => ()
        }
    
    column_number += 1;
    }

    found_fossil_list.push(Fossil{
        pokemon: "Aerodactyl".to_string(),
        price:"-".to_string(),
        obs: "Aerodactyl's fossil is in Pewter's Museum back entrance, same as in vanilla Fire Red. A guy in the Mansion can restore it.".to_string()
    });
    found_fossil_list.push(Fossil{
        pokemon: "Omanyte/Kabuto".to_string(),
        price:"-".to_string(),
        obs: "Omanyte and Kabuto's fossils are in Mt. Moon, same as in vanilla Fire Red. A guy in the Mansion can restore them\n
        The guy in Celadon's Mansion gives you the other choice that you didn't pick in Mt. Moon..".to_string()
    });


    let fossil_encounter: FossilEncounter = FossilEncounter { trade: trade_fossil_list, found: found_fossil_list };
    fossil_encounter
}

fn calamine_fossil_extractor() -> Encounters<RouteInfo<FossilEncounter>>{
    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").expect("Couldn't open pokemon xlsx");
    let sheets = excel.sheet_names().to_owned(); 
    let mut route_vec: Vec<RouteInfo<FossilEncounter>> = vec![];
    if let Some(Ok(document)) = excel.worksheet_range(&sheets[5]){
        let fossil_encounter = get_fossil_pokemon(&document);
        let route_info:RouteInfo<FossilEncounter> = RouteInfo { route: "Mansion".to_string(), encounter_info: fossil_encounter };
        route_vec.push(route_info);
        
    }
    let encounters:Encounters<RouteInfo<FossilEncounter>> = Encounters { encountermethod: "Egg Vendor".to_string(), routes: route_vec };
    encounters
}

fn get_unobtainable_pokemon(){

}

fn calamine_unobtainable_pokemon() {
    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").expect("Couldn't open pokemon xlsx");
    let sheets = excel.sheet_names().to_owned(); 
    if let Some(Ok(document)) = excel.worksheet_range(&sheets[5]){
    
    }

}   

fn main() {

    let list = get_pokemon_list();
    let grass_encounter = calamine_grass_extractor();
    let water_encounter = calamine_water_extractor();
    let trade_encounters =calamine_trade_pokemon_extractor();
    let gift_encounters = calamine_gift_pokemon_extractor();
    let egg_counters = calamine_egg_vendor_extractor();
    let fossil_encounters = calamine_fossil_extractor();

    dbg!(water_encounter);

    
}