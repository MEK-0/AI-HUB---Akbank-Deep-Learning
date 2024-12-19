/*Stringler

Rust dilinde String türü iki şekilde kullanılır:

String (sahiplikli): Heap bellekte saklanır, büyüyüp küçülebilir ve değiştirilebilir.
&str (string dilimi): Sabit, bir String'in veya ham bir metnin referansıdır.

 */
fn main (){
     //Sahiplik String
     let mut sahiplik_string =String::from("Merhaba");
     sahiplik_string.push_str(", Dünya!"); // String ekler
     println!("{}",sahiplik_string);
    
    // String uzunluğu
    println!("Uzunluk: {}", sahiplik_string.len());

    // Karakterlere erişim
     for c in sahiplik_string.chars() {
        println!("Karakter: {}", c);
     }
    // String’i dilimleme
    let ilk_kelime = &sahiplik_string[0..7]; // İlk 7 karakter
    println!("İlk kelime: {}", ilk_kelime);

     //String dilimi (&str)
     let dilim: &str = "Bu bir dilim";
     println!("{}",dilim);

     //String birleştirme
     let string1 =String::from("Rust ");
     let string2 =String::from("dili");
     let birlesim = string1 + &string2;
     println!("{}",birlesim);


}