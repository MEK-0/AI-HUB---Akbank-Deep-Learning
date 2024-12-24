//Rust'ta hata yönetimi, kodun güvenliğini artırmak için güçlü mekanizmalar sunar: Option ve Result.

/*Option Enum
Option veri türü, bir değerin olup olmadığını ifade eder. İki farklı durumu vardır:

Some(value): Değer mevcutsa.
None: Değer mevcut değilse. */

//Option Kullanımı 

fn bolme(x:f64, y:f64)-> Option<f64>{
    if y ==0.0 {
        None // Bölme sıfıra yapılmaz
    } else {
         Some(x/y)
    }
}





fn main(){

    match bolme(10.0,2.0){
        Some(sonuc) => println!("Sonuç: {}",sonuc),
        None => println!("Bölme sıfıra yapılamaz!"),
    }
    
}