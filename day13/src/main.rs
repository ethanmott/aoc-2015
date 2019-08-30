const TABLE_SIZE: usize = 4;

enum Person {
    ALICE,
    BOB,
    CAROL,
    DAVID
}



struct Table {

}

fn main() {
    let table = vec![1; TABLE_SIZE];

    println!("{:?}", table);
}
