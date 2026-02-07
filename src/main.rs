fn main() {

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
}
