use std::net::UdpSocket;
use std::io;

enum IpAddr {
    V4(String),
    V6(String),
}        

fn main() {
    // enum'dan bir örnek oluşturuyoruz
    let v = IpAddr::V4(String::from("127.0.0.1"));

    // eşleştirme sonucu elde s olarak tutulur ve ip_str'ye atanır
    let ip_str = match v {
            IpAddr::V4(s) => s,
            IpAddr::V6(s) => s,
    };

    println!("{ip_str}"); // 127.0.0.1

    /* Result
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    fn main() {
        let x: Result<i32, &str> = Ok(-3);
        assert_eq!(x.is_ok(), true);

        let x: Result<i32, &str> = Err("Hata Oldu");
        assert_eq!(x.is_err(), true);

        match x {
            Ok(i) => assert_eq!(i, -3),
            Err(s) => assert_eq(s),
        }
    } */

    // Result yapısına bir örnek

    let socket: Result<UdpSocket, io::Error> = UdpSocket::bind("127.0.0.1:1453");

    let socket:UdpSocket = match socket {
        Ok(sock) => sock,
        Err(err) => panic!("Bind etme hatası: {err}"),
    };

    let socket2: Result<UdpSocket, io::Error> = UdpSocket::bind("127.0.0.1:1453");

    // Ok() ise socketi döndürür, Err() ise panic!
    let socket: UdpSocket = socket2.unwrap();

    let socket3: Result<UdpSocket, io::Error> = UdpSocket::bind("127.0.0.1:1453");

    // Ok() ise socketi döndürür, Err() ise hata mesajı ve çalışma durur
    let socket: UdpSocket = socket3.expect("blind etme hatası");

    
    /* Option 
    pub enum Option<T> {
        None,
        Some(T),
    }

    // Null yani None olabilir 
    let var: Option<i32> = Some(-3);
    assert_eq!(var.unwrap(), -3);
    let variable: Option<i32> = None;
    assert_eq!(var.unwrap(), -3); // panic
    */

    let x: Option<i32> = Some(-3);
    match x {
        Some(i) => assert_eq!(i, -3),
        None => println!("Değer Yok.")
    };

    let y: Option<i32> = Some(-3);
    if let Some(i) = y {
        assert_eq!(i, -3);
    }

    // struct

    let p = Point {x:1, y:2};
    let px = p.x;   //p.0
    let py = p.y;  //p.1 


    let person = Person {name: String::from("Tunahan"), age: 23};

    let mut p = Person::new(String::from("Tuna"), 24);

    p.yas_ayarla(27);

    println!("{:?}", p);
    
    // Trait

    let s = Person2 {name: String::from("Tuns"), age:23};

    say_your_name(&s);

    // macro kullanımı, makrolar compile-time'da çalışır

    #[cfg(target_os = "linux")]
    fn sadece_linuxta_calis() {}

    #[test]
    fn test_fonksiyonudur() {}

    macro_rules! merhaba {
        () => {
            println!("Merhaba")
        };
    }

    merhaba!();

}



// Result kolay syntax, değer bir Err değilse çalışmaya devam et, Err ise durdur

fn easy_result() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:1453")?;

    Ok(())
}

// struct

struct Point {
    x: i64,
    y: i64,
}

struct TupleOrnek(i32, i32);

struct VeriYok;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn yas_kac(&self) -> u8 { //bu method nesneye erişir fakat değişiklik yapmaz
        self.age
    }

    fn yas_ayarla(&mut self, yas: u8) { // burada hem erişiriz hem de değişiklik yaparız
        self.age = yas;
    }

    fn new(name: String, age: u8) -> Self { // yeni bir nesne oluşturmak için kullanırız
        Self { name, age}
    }
}

/* Trait: Diğer dillerdeki interfacelerdir, bir davranış biçimi taslağı sunar
    örnek: "bir kouşmacı olabilmek için iki methodu implement etmeli." */

    trait Speaker {
        fn say_name(&self) -> String;
        fn say_age(&self) -> String;
    }

    struct Person2 {
        name: String,
        age: u8,
    }

    impl Speaker for Person2 {
        fn say_name(&self) -> String {
            format!("My name is {}", self.name) // string formatlama
        }
        fn say_age(&self) -> String{
            format!("My age is {}", self.age)
        }
        
    }

    fn say_your_name(speaker: &impl Speaker) {
        println!("{}", speaker.say_name());
    }



