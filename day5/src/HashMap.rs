/*HashMap, Rust'ta anahtar-değer (key-value) çiftlerini depolamak için kullanılan bir veri yapısıdır. Örneğin, bir öğrencinin adıyla notlarını eşleştirmek gibi senaryolar için idealdir.

Özellikler:

Dinamik boyutlu: Veri miktarına göre büyüyebilir.
Anahtarlar benzersiz olmalıdır: Aynı anahtar tekrar eklenemez.
Performans: Veri arama işlemleri oldukça hızlıdır. */

use std::{collections::HashMap, ops::Not};
fn main() {
    
    let mut ogrenci_notlari= HashMap::new();

    //veri ekleme kısmı
   ogrenci_notlari.insert("ali", 85);
   ogrenci_notlari.insert("ayse",90);
   ogrenci_notlari.insert("esat ",100);


   println!("Öğrenci Notları: {:?}",ogrenci_notlari);


   //Bir anahtarın değerine erişme

   if let Some(not) = ogrenci_notlari.get("ali"){
    println!("Ali'nin notu: {}",not);
   } else {
     println!("Ali'nin notu bulunamadı.");
   }

   // Döngü ile tüm verileri dolaşma
   for (isim,not)in &ogrenci_notlari{
     println!("{}: {}",isim,not);
   }
    
    //Aynı anahatarı güncelleme 
    ogrenci_notlari.insert("esat",99);

    if let Some(not) = ogrenci_notlari.get("esat"){
        println!("Esat'ın notu: {}",not);
    }  else {
        println!("Esat'ın notu bulunamadı");
    } 
    
    // metinde ne kadar kelime geçtiğini hesaplayan kod bloğu

    let mut kelime_sayisi = HashMap::new();
    let metin ="Mahmut Esat Kolay Selçuk Üniversitesi Bilgisayar Mühendisliği";

    for kelime in metin.split_ascii_whitespace(){
        *kelime_sayisi.entry(kelime).or_insert(0)+=1;
    }
    println!("Kelime Frekansı: {:?}",kelime_sayisi);


}
