use std::io;

// Operation adında bir enum tanımlanıyor.
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// calculate fonksiyonu, Operation enumunu alıp uygun aritmetik işlemi gerçekleştiren bir f64 sonuç döndürüyor.
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    // Kullanıcıdan ilk sayıyı girmesini istiyoruz.
    println!("Enter the first number:");
    let mut input = String::new();
    // Kullanıcının girişini "input" değişkenine alıyoruz ve hataları handle ediyoruz.
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // Girişi alıp bir sayıya dönüştürüp "first_number" değişkenine atıyoruz.
    let first_number: f64 = input.trim().parse().expect("Invalid input");

    // Kullanıcıdan gerçekleştirilecek işlemi girmesini istiyoruz.
    println!("Enter the operation (+, -, *, /):");
    let mut operation_input = String::new();
    // Kullanıcının girişini "operation_input" değişkenine alıyoruz ve hataları handle ediyoruz.
    io::stdin().read_line(&mut operation_input).expect("Failed to read line");
    // Girişi alıp işlem sembolünü belirleyerek "operation_symbol" değişkenine atıyoruz.
    let operation_symbol = operation_input.trim();

    // Kullanıcıdan ikinci sayıyı girmesini istiyoruz.
    println!("Enter the second number:");
    let mut second_input = String::new();
    // Kullanıcının girişini "second_input" değişkenine alıyoruz ve hataları handle ediyoruz.
    io::stdin().read_line(&mut second_input).expect("Failed to read line");
    // Girişi alıp bir sayıya dönüştürüp "second_number" değişkenine atıyoruz.
    let second_number: f64 = second_input.trim().parse().expect("Invalid input");

    // Belirlenen sembole göre uygun Operation enumunu oluşturuyoruz.
    let operation = match operation_symbol {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => panic!("Invalid operation"),
    };

    // Oluşturulan Operation enumu ile calculate fonksiyonunu çağırıyoruz ve sonucu "result" değişkenine atıyoruz.
    let result = calculate(operation);

    // Sonucu ekrana yazdırıyoruz.
    println!("Result: {}", result);
}
