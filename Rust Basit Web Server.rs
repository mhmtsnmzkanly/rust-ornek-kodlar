use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream,SocketAddr};

fn tcp_dinleyici_gelen_eleal(mut bağlantı: TcpStream){
    let mut bağlantı_arabelleği = [0; 1024];
    bağlantı.read(&mut bağlantı_arabelleği).unwrap();
    let bağlantı_isteği = String::from_utf8_lossy(&bağlantı_arabelleği[..]);
    let cevap = format!("İstek: {:?}", bağlantı_isteği);
    let cevap_başlık = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", cevap.len(), cevap);
    bağlantı.write(cevap_başlık.as_bytes()).unwrap();
    bağlantı.flush().unwrap();
}

fn tcp_dinleyici_gelenler(tcp_dinleyici: &TcpListener) {
    println!("tcp_dinleyici_gelenler(): Fonksiyon devreye girdi.");
    for bağlantı in tcp_dinleyici.incoming() {
        let bağlantı = bağlantı.unwrap();
        println!("Gelen Bağlantı:\n {:?}", bağlantı);
        tcp_dinleyici_gelen_eleal(bağlantı)
    }
    println!("tcp_dinleyici_gelenler(): Fonksiyon işlemi bitti.");
}

fn main(){
    let tcp_müsait_soketler = [
        SocketAddr::from(([127, 0, 0, 1], 8090)),
    ];
    let tcp_dinleyici = TcpListener::bind(&tcp_müsait_soketler[..]).unwrap();
    
    tcp_dinleyici_gelenler(&tcp_dinleyici);
}