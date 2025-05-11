use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct MyStruct {
    id: u64,
    data: String,
    v: Vec<u32>
}

pub fn borsh() {
    let original = MyStruct{
        id: 42,
        data: "hello, borsh!".into(),
        v: vec![1,2,3,4]
    };

    let mut buffer: Vec<u8> = Vec::new();
    original.serialize(&mut buffer).unwrap();

    // deserialize
    let deserialized = MyStruct::try_from_slice(&mut buffer).unwrap();
    
    assert_eq!(original,deserialized);
    println!("done: {:?}", deserialized);

}