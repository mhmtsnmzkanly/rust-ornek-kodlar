fn main() {
    println!("Hello, world!");
    let mut carpilacak: i32 = 5;
    let kacla: i32 = 100;
    println!("x = {}, y = {}", carpilacak, kacla);
    carp(&mut carpilacak, &kacla);
    println!("carp(x, y) = {}", carpilacak);


}

fn carp(carpilacak: &mut i32, kacla: &i32)
{
    *carpilacak = *carpilacak * kacla;
}