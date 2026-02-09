
//Enum code..
#[derive(Debug)]
enum SpreadsheetCell<'a> {
    Int(i32),
    Float(f64),
    Text(&'a str)
}

impl SpreadsheetCell<'_> {
    fn value(&self) -> &dyn std::fmt::Debug {
        match self {
            SpreadsheetCell::Int(i) => i,
            SpreadsheetCell::Float(f) => f,
            SpreadsheetCell::Text(s) => s
        }
    }
}

/* // FIND Largest number in an array slice
fn find_largest(stream: &[i32]) -> &i32 {
    let mut largest: &i32 = &stream[0];
    for item in stream {
        if item > largest {
            largest = item;
        }
    }
    largest
}
*/

fn main() {

    //CONTINUED..Enum code..
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.2),
        SpreadsheetCell::Text("hello")
    ];

    let first = &row[0].value();
    println!("{:?}", first);
    /// END..Enum code..

    /* CONTINUED..FIND Largest number in an array slice
    let master: [i32; 7] = [1,2,3,12,5,6,7];
    let stream: &[i32; 7] = &master;

    let result = find_largest(stream);

    println!("{result}");
    */

    /* Updating a Box (heap) via a mutable reference (stack) several mutable refs deep..
    let mut a: Box<i32> = Box::new(1);

    let mut b: &mut Box<i32> = &mut a;
    let mut c: &mut &mut Box<i32> = &mut b;
    let mut d: &mut &mut &mut Box<i32> = &mut c;
    let mut e: &mut &mut &mut &mut Box<i32> = &mut d;
    let mut f: &mut &mut &mut &mut &mut Box<i32> = &mut e;
    let mut g: &mut &mut &mut &mut &mut &mut Box<i32> = &mut f;
    let mut h: &mut &mut &mut &mut &mut &mut &mut Box<i32> = &mut g;
    let i: &mut &mut &mut &mut &mut &mut &mut &mut Box<i32> = &mut h;

    ********i = Box::new(2);   //OR *********i = 2;

    println!("{i}");
    */


    /* Updating a Box (heap) via a mutable reference (stack)
    let mut b: Box<i32> = Box::new(1);
    let c: &mut Box<i32> = &mut b;

    *c = Box::new(2); // OR **c = 2;

    println!("{c}");
     */


    /* Updating an i32 (stack) via a mutable reference (stack)
    let mut a: i32 = 1;
    let b: &mut i32 = &mut a;

    *b = 2; // OR a = 2;  OR a += 1;
    println!("{b}");
    */


    /* Updating a mutable variable (stack) via a nested mutable reference (stack) n-layers deep.
    let mut a: i32 = 1;
    let mut b: &mut i32 = &mut a;
    let mut c: &mut &mut i32 = &mut b;
    let mut d: &mut &mut &mut i32 = &mut c;
    let mut e: &mut &mut &mut &mut i32 = &mut d;
    let mut f: &mut &mut &mut &mut &mut i32 = &mut e;
    let mut g: &mut &mut &mut &mut &mut &mut i32 = &mut f;
    let mut h: &mut &mut &mut &mut &mut &mut &mut i32 = &mut g;
    let i: &mut &mut &mut &mut &mut &mut &mut &mut i32 = &mut h;

    ********i += 1;

    println!("{i}");
    */

}
