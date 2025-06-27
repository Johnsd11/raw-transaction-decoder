use std::io::Read;


#[allow(unused_variables)]
fn read_version(transaction_bytes: &mut &[u8]) -> u32 {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer) 
}

#[allow(unused_variables)]
fn main() {
    let transaction_hex = "0200000000010133e0fb4b86aa7711b9cc164d23873c1eaf62e0149c6489d72f96a178df2315b20000000000fdffffff018e68000000000000160014d7e4b7c8c3c4ea01c33cfd9862bcfa9aa3ea41d00247304402206e8a1f16ab004aa8c7ad4bf5774359008176b2ca446f35e58d63095ceec49c2b02202b86535f8881eb8ddd301101ed43f2c7ad19e4f7172e5793cb983973751456bc01210337ca21787105ed454080e4414cbbe2904be57f1643d23cb0172ed9368da43bbd43c60d00";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_version(&mut bytes_slice);
    println!("bytes slice first element: {:?}", bytes_slice[0]);
    println!("version:{}",version);
}
