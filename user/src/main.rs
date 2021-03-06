use proc_exp::frame;

frame! {
    Frame {
        // Bytes 0 to 7 represent a byte array of size 8 and will be parsed into an owned array.
        field1 [0..8]: [u8; 8],
        // Bytes 0 to 7 represent a 8 byte unsigned int and will be represented as an u64.
        field2 [0..8]: u64,
        // Byte 2 represents a 1 byte unsigned int and will be represented as an u8.
        field3 [2]: u8,
        // The bits 1 to 4 of byte 3 are seen as a 4 bit unsigned int and will be represented by an u8.
        field4 [3][1..5]: u8,
        // The bits 0 and 1 of byte 4 are seen as a 2 bit enum and will be represented by the Kek type which is an enum.
        // The valid values need to be defined on the enum defined via frame!() too.
        field5 [4][0..1]: Kek,
        // Byte 4 might hold an optional char if bit 0 of byte 7 is set.
        field6 [4]: Option<char> -> [7][0]: bool,
        // Bytes 4 to 41 represent a dynamically sized list with the lenght indicated by byte 3, represented as an u8.
        field7 [4..42]: &'static [u8] -> [3]: u8,
        // Bytes 5 to 1336 represent an enum Payload (to be defined otherplace as a frame! type too). The enum can hold different typed payloads. Which variant is to be used, is indicated by bits 0 and 1 of byte 4, which are interpreted as a PayloadMarker. The enum Variant names need to match.
        field8 [5..1337]: Payload -> [4][0..1]: PayloadMarker
    }
}

fn main() {
    println!("Hello, world!");
}
