fn main(){
    let hello_world = String::from("hello world");

    let hello = &hello_world[0..5]; // "..5"'te aynı sonucu verir
    let world = &hello_world[6..11];

    println!("hello_world: {0}\nhello: {1}\nworld: {2}\n", hello_world, hello, world);

    let hello = String::from("hello");
    
    let slice_hello = &hello[0..2];
    let slice_hello = &hello[..2];
    println!("hello: {0}\nslice_hello: {1}", hello, slice_hello);
}