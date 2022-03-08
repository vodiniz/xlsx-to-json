use calamine::{open_workbook, Xlsx, Reader,Range, DataType};


fn calamine() {

    let mut excel: Xlsx<_> = open_workbook("pokemon_locations.xlsx").unwrap();
    let sheets = excel.sheet_names().to_owned();
    
    // for s in sheets {
    //     println! ("Current Sheet : {}", s);
    //     if let Some(Ok(r)) = excel.worksheet_range(&s) {
    //         for row in r.rows() {
    //             println!("row={:?}, row[0]={:?}", row, row[0]);
    //         }
    //     }
    // }1

    if let Some(Ok(document)) = excel.worksheet_range(&sheets[0]) {


        for column in document.range((0,2),(28,2)).used_cells() {
            println!("Valor: {:?}", column);
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

        }
            // println!("{:?}", cell);

    }

            // println!("row={:?}", row[0]);
            // println!("\n --------------------------------------")



}
        


fn main() {

    calamine();

}