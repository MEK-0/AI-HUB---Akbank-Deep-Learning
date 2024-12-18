//Structs and Enums , Pattern matching with enums

/*1. Structs (Yapılar):

Rust'ta bir struct (yapı), birden fazla değeri bir araya getiren bir veri tipidir.
Alanlar (fields) ile tanımlanır ve her alan bir tür (type) ve bir ada sahiptir.
Üç çeşit struct vardır:
Normal Struct: Alan isimleri ve türleriyle tanımlanır.
Tuple Struct: Sadece türleri belirtir; alanların isimleri yoktur.
Unit-like Struct: Boş bir struct türüdür; genelde marker olarak kullanılır.
Kullanım Alanı: Bir nesneye veya kavrama ait birden fazla özelliği bir arada tutmak için kullanılır. Örneğin, bir "Kullanıcı" bilgilerini tutmak.

2. Enums (Numaralandırmalar):

enum, Rust'ta birden fazla veri türünü bir arada ifade edebilen, bir tür "seçenek" mekanizmasıdır.
Farklı çeşitlerdeki değerleri tanımlayıp, her birini adlandırarak birleştirmenizi sağlar.
Kullanım Alanı: Bir verinin birden fazla durumu olabileceğinde tercih edilir. Örneğin, bir dosya işlem durumu "Başarılı", "Hata", veya "Beklemede" olabilir. */


struct Kullanici{
    ad: String,
    yas : u32,
    email:String,
}

struct Nokta(i32,i32,i32);// Tuple Struct kullanımı

// Unit-like Struct 
/*Unit-like struct, Rust'ta boş bir yapıdır ve genellikle veri tutmaz. Hiçbir alanı veya özelliği bulunmaz. Bu yapılar, çoğunlukla marker (işaretleyici) veya durum göstergesi olarak kullanılır. */

struct Marker;

// Struct içinde Method kullanımı 

struct Dikdortgen{
    genislik: u32,
    yukseklik: u32,
}

impl Dikdortgen{ //impl (implement) bloğu, Dikdortgen yapısı için metotları (yöntemleri) tanımlar.
    fn alan(&self)->u32{ // self nesneye erişim sağlar 
        self.genislik*self.yukseklik
    }
    fn cevre(&self)->u32{
        2*(self.genislik+self.yukseklik)
    }
}


fn main() {
   let kullanici1=Kullanici{
    ad:String::from("Esat"),
    yas:20,
    email:String::from("esat@example.com"),
   };

   println!("Ad: {}, Yaş {}, Email: {}",kullanici1.ad,kullanici1.yas,kullanici1.email);
    
    let nokta1=Nokta(10,20,30);

    println!("X: {}, Y: {}, Z: {}",nokta1.0,nokta1.1,nokta1.2);

    let marker=Marker;
    println!("Bir marker struct ı oluşturuldu!");

    let dikdortgen =Dikdortgen{
        genislik: 10,
        yukseklik: 5,
    };
    println!("Alan: {}",dikdortgen.alan());
    println!("Çevre: {}",dikdortgen.cevre()); 
    

}

