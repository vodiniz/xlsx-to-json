use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};
use office::{Excel, Range, DataType};

fn office_test() {



    // opens a new workbook
    let mut workbook = Excel::open("test.xlsx").unwrap();
    
    // Read whole worksheet data and provide some statistics
    if let Ok(range) = workbook.worksheet_range("Sheet1") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.rows().map(|r| {
            r.iter().filter(|cell| cell != &&DataType::Empty).count()
        }).sum();
        println!("Found {} cells in 'Sheet1', including {} non empty cells",
                 total_cells, non_empty_cells);
    }
    
    // Check if the workbook has a vba project
    if workbook.has_vba() {
        let mut vba = workbook.vba_project().expect("Cannot find VbaProject");
        let vba = vba.to_mut();
        let module1 = vba.get_module("Module 1").unwrap();
        println!("Module 1 code:");
        println!("{}", module1);
        for r in vba.get_references() {
            if r.is_missing() {
                println!("Reference {} is broken or not accessible", r.name);
            }
        }
    }
}


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
    // }
    if let Some(Ok(r)) = excel.worksheet_range(&sheets[0]) {
        for row in r.rows() {

        println!("row={:?}, row[0]={:?}", row, row[0]);
        println!("\n --------------------------------------")

        }
    }


}

fn main() {


    //office_test();
    calamine();

}