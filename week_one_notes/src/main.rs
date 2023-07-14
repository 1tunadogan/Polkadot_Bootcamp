/*  -----------------CARGO------------------------------
    yeni bir proje pluşturmak için "cargo new, cargo init" 
    derlemek için  "cargo init 
    bağımlılık eklemek için "cargo add crateisimi"
    calıştırmak için "cargo run"
    test koşturma içi "cargo test"
    bağımlı paket lisansları korol etmek için "cargo deny"
    kod formatlama ve linting "cargo ftm, cargo clippy" */

use std::{vec, collections::HashMap};

fn main() {

    // tam sayı değişkenleri, i8...i128, u8...u128, işlemci biti isize, usize
    let a = 1; // default i32
    let b = 1i32;
    let c: i32 = 125;

    // float değişkenler, f32, f64
    let pi = 3.14159; // default f64
    let pi: f64 = 3.14159;

    // boolean değişkenler, 8-bit
    let a: bool = true;
    let b = false;

    // char 32-bit
    let karakter: char = 't';
    let sigma = 'σ';
    let emoji = '😄';

    // string değişkenleri, String, &str
    let s: &str = "Sabit bir metin"; // değiştirilemez sabit bir UTF-8 string slice
    let s: String = String::from("Dinamik bir metin");  // heap'te oluşturulmuş dinamik bir String struct

    // değişken atamaları
    let z = 5;
    z = 4; // ! hata verir çünkü z immutable
    
    let mut x = 4;
    x = 45; // artık atayabilriz "mut" ile değiştirlebilir bir yapı oluşturduk

    let g = 5;
    let g = 15; // eski değer yok oldu g artık 15

    // fonksiyonlar

    fn do_someting () -> i32 {
        5 // return değeri
    }

    fn karesi ( sayi: i32) -> i32 {
        sayi * sayi // return sayi * sayi ile aynı şey
    }

    karesi(12); // 144

    // tuple
    let new_tuple = (500, 6.4, 1);
    let xyz = (1.0, 2.0, 3.0 );

    // ayrıştırma
    let (x, y, z) = xyz;

    // veya
    let x = xyz.0;
    let y = xyz.1;
    let z = xyz.2;

    // struct
    struct User {
        active: bool,
        usernama: String,
        email: String,
        sign_in_count: u64,
    }

    let user = User {
        active: true,
        usernama: String::from("user"),
        email: String::from("user@mail.com"),
        sign_in_count: 12,

    };

    // bu yapıda format! makrosu ile yeni bir string oluşturmak hedeflenmiştir
    struct User2 {name: String}

    impl User2 {
        fn greet(&self) -> String { // &self ifadesi bir referans değer tutar ve asıl değer kaybolmaz
            format!("Merhaba ben {}", self.name)
        }
    }

    let user2 = User2 {
        name: String::from("Tuna"),
    };

    println!("{}", user2.greet());

    // enum
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        CahgeColor(i32, i32, i32),
    }

    fn print_enum(msg: &Message) {  // referans değer ataması hem sahiipiliği hem de performansı korur
        match msg {
            Message::Quit => println!("I quit"),
            Message::Move { x, y }  => println!("{}, {}", x, y),
            Message::Write(str) => println!("{}", str),
            Message::CahgeColor(r, g, b) => println!("{r}, {g}, {b}"), 
        }  
    }

    let msg_quit: Message = Message::Quit;
    let msg_move: Message = Message::Move {x: 1, y: 2};
    let msg_write: Message = Message::Write(String::from("Writting"));
    let msg_color = Message::CahgeColor(255, 0, 0);

    print_enum(&msg_quit);
    print_enum(&msg_move);
    print_enum(&msg_write);
    print_enum(&msg_color);

    // array, statik yapıdadırlar ve derleme zamanında boyutları belirlenir değiştirilemezler
    let array: [i32; 3] = [1, 2, 3];

    // array ve slices
    let array2: [i32; 6] = [10, 20, 30, 40, 50, 60];    // 6 elemanlı bir dizi
    println!("array: {array:?}");   // degub formatı ile çıktı oluşturur

    let slice: &[i32] = &array2[2..4];  // belli bir aralıkta seçilmiş dizinin parçaları
    println!("slice: {slice:?}");

    /* slicer bellek kullanımı açısından çok büyük avantaj sağlar yeni bir depolama alanını işgal etmeden 
    sadece referans ile bir array2in belli bir ksımını tutar, tutuğu arrayde bu sayede bir değişiklik olmaz */

    // sliceler da default olarak immutable'dır
    let array3 = [1, 2, 3];

    array3[3] = 4;  // bu atama hata verir çünkü 3 elemanlı bir array
    array3[0] = 10; // bu atamada da hata verir çünkü array immutable'dır

    let mut array3 = [1, 2, 3];

    array3[3] = 4;  // bu kod hala hata verir çümkü arrayler statik bir yapıdadır
    array3[0] = 10; // bu işlemi artık yapabiliriz

    // vector, dinamik diziler
    let mut vector = Vec::from([1, 2, 3]);
    let mut vector = vec![1, 2, 3]; // bir macro yardımı ile tanımlayabilriz
    
    let mut vector = vec![0; 3]; // 0, 0, 0

    // program çalışırken ekleme ve çıkarma yapılabilir
    vector.push(4); // 0, 0, 0, 4
    vector.pop();   // 0, 0, 0

    // HashMap, dictionary
    let mut hm: HashMap<i32, &str> = HashMap::new();

    hm.insert(42, "masa");
    hm.insert(16, "sandalye");

    println!("{:?}", hm);

    // tip belirtimi yapmak zorunda değiliz aslında, derleyici ileride kullanılan veri tipini algılayabilir

    // ownership, 
    let mut hm = HashMap::new();

    hm.insert(42, "masa");
    hm.insert(16, "sandalye");

    use_hashmap(hm); // bu kullanımda scope dışına çıktığı ve gerçek değer kullanıldığı için alt kısımda erişemeyiz

    println!("{:?}", hm);

    // bu durumu düzeltmek için use_hashmap fonksiyonunu düzeltebiriz

    // borrowing
    let mut hm2 = HashMap::new();

    hm2.insert(42, "masa");
    hm2.insert(16, "sandalye");

    use_hashmap2(&hm); // sahipliği vermedik ödünç verdik sadece

        println!("{:?}", hm2);

    // control flow, if, if let
    let a = 1;

    if a == 1{
        println!("a == 1");
    } else {
        println!("a != 1");
    }

    let a: Option<i32> = Some(1); // option: dolu/boş olabilen bir enum

    if let Some(deger) = a {
        println!("Bir değer varmış {}", deger)
    }else {
        println!("a'nın içinde bir değişken yok.")
    }

    // match
    let a = 1;

    match a {
        1 => println!("a == 1"),
        _ => println!("a != 1"), // default durum
    }

    let a = 5;

    match a {
        0..=4 => println!("a 0 ve 4 arasında"),
        5..=10 => println!("a 5 ve 10 arasında"),
        _ => (),
    }

    let a:Option<i32> = None;

    match a {
        Some(deger) => println!("değer var"),
        None => println!("değer yok"),
    }

    let a = Message::Move { x: 1, y: 2 };

match a {
    Message::Quit => {
        println!("Çıkış yapıyorum");
    },
    Message::Move { x, y } => {
        println!("Yürüyorum {} {}", x, y);
    },
    _ => {
        // Diğer durumlarla ilgili işlemler
    }
}


    // for
    for i in 1..=41 {
        println!("maşallah")
    }   // output: 41 kere maşallah :)

    // while
    let mut i = 1;

    while i <=41 {
        println!("maşallah");

        i += 1;
    }   // output: 41 kere maşallah :)

    // loop
    let mut i = 1;

    loop {
        println!("maşallah");

        i += 1;
        if i > 41 {
            break;
        }
    }

    // while let
    let mut x = vec![1, 2, 3];

    while let Some(deger) = x.pop()  {  // vektörde bir değer var ise çıkarmaya devam eder
        println!("deger = {deger}");
    }


}

fn use_hashmap(hm: HashMap<i32, &str>)-> HashMap<i32, &str> {
    hm
}

fn use_hashmap2(hm: &HashMap<i32, &str>) { // bir struct referansı aldık

}

enum Message {
    Quit,
    Move {x: i32, y: i32},
}