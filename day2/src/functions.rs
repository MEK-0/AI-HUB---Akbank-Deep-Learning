fn selamla(){
     println!("Merhaba Rust !");
}
fn selamla2(isim:&str){
   println!("Merhaba, {}!",isim);
}
fn topla(a: i32,b: i32){
   println!("{} + {} = {}",a,b,a+b);
}
fn kare(x: i32)-> i32 {
    return x*x;
}
fn kup(x: i32)-> i32{
    x*x*x  // Return olmadan 
}
fn topla2(a: i32, b: Option<i32>) -> i32 {
    match b {
        Some(deger) => a + deger,
        None => a, // Varsayılan değer 0 gibi düşünülebilir
    }
}
fn ana_fonksiyon() {
    fn ic_fonksiyon() {
        println!("İç fonksiyon çalıştı!");
    }

    println!("Ana fonksiyon çalıştı!");
    ic_fonksiyon();
}
fn faktoriyel(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * faktoriyel(n - 1)
    }
}
fn islemi_yap(f: impl Fn(i32, i32) -> i32, x: i32, y: i32) {
    println!("İşlem sonucu: {}", f(x, y));
}
fn topla_fonksiyonu() -> fn(i32, i32) -> i32 {
    fn topla(a: i32, b: i32) -> i32 {
        a + b
    }
    topla
}



fn main(){
  selamla(); // Basit fonksiyon Parametresiz 
  selamla2("Esat"); // Parametreli fonksiyon (String)
  topla(5,7);  // Parametreli fonksiyon (integer)
    //Değer döndüren fonksiyonlar
  let sonuc = kare(3);  // Return ile değer dönderen fonksiyonlar
  println!("Sonuç : {}",sonuc);
  let sonuc1 = kup(3);
   println!("Sonuç : {}",sonuc1);
     // Opsiyonel Parametre ile çalışma
   println!("Toplam: {}", topla(5, Some(3))); // 5 + 3
   println!("Toplam: {}", topla(5, None));    // 5

    // Nested Fonksiyon

    ana_fonksiyon();
 
    // Rekursif fonksiyonlar
     
     let sonuc2 = faktoriyel(5);
     println!("5! = {}",sonuc2);

    // Lambda Fonksiyonlar(Closure)
    // -> Rust'ta anonim fonksiyonlar (closure) |parametreler| { ... } sözdizimi ile tanımlanır.
    
    let toplama = |a: i32, b: i32| a + b;

    println!("Sonuç: {}", toplama(4, 5));
     
     //Bir Closure Fonksiyona Parametre Olarak Geçme
     
     let carpma = |a, b| a * b;

    islemi_yap(carpma, 3, 4); // Closure parametre olarak gönderilir
     

     // Fonksiyon işaretçileri 

     islemi_yap(topla,3,5);

     //Fonksiyonların Geri Dönüş Tipi Olarak Kullanılması
     
     let f = topla_fonksiyonu();
    println!("Sonuç: {}", f(4, 6));

} 
