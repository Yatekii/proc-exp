use proc_exp::frame;

frame! {
    Frame {
        // byte 0 through 8 as byte-array
        field1 [0..8]: [u8; 8],
        // byte 0 through 8 as u64
        field2 [0..8]: u64,
        // byte 2 as u8
        field3 [2]: u8,
        // byte 3 bits 1 through 5 as u8
        field4 [3][1..5]: u8,
        // byte 4 bits 0 through 1 as enum KekHint; KekHint needs to impl Serializable
        field5 [4][0..1]: KekHint,
        // // dependant enum with it's body ranging from byte 5 to 42
        // // type is Kek and the hinter is field4
        // // parsed variant depends on the variant field4 holds
        // // Keks variant needs to hold a struct with the same name as the variant
        // field6 [5..42]: Kek: field5,
        // // A bool, characterized by byte 7, bit 0
        // field7 [7][0]: bool,
        // this field will automatically be Option<char> where char is byte 4
        // and the option state is represented by field6
        field8 [4]: char -> [7][0]: bool,
    }
}

fn main() {
    println!("Hello, world!");
}
