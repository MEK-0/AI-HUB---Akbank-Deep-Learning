/*2. Enums (Numaralandırmalar):

enum, Rust'ta birden fazla veri türünü bir arada ifade edebilen, bir tür "seçenek" mekanizmasıdır.
Farklı çeşitlerdeki değerleri tanımlayıp, her birini adlandırarak birleştirmenizi sağlar.
Kullanım Alanı: Bir verinin birden fazla durumu olabileceğinde tercih edilir. Örneğin, bir dosya işlem durumu "Başarılı", "Hata", veya "Beklemede" olabilir. */

enum Renk{
    Kirmizi,
    Yesil,
    Mavi,
    
}
// Enum ile veri taşıma
enum Mesaj{
    Selamla(String),
    Sayisal(u32),
    Bitir,
}

enum Hareket{
    Ileri {mesafe: u32},
    Sag{derece: u32},
    Dur,
}

//Option Enum Kullanımı

fn bul_metin(metin: Option<&str>){
    match metin {
        Some(deger)=> println!("Bulunan metin: {}",deger),
        None => println!("metin Bulunamadı"),
    }
}
struct Kullanici {
     ad: String,
     yas: u32,
}
enum Durum {
    GirisYapildi(Kullanici),
    GirisBekleniyor,
    Hata(String),
}

fn main(){

    let renk = Renk::Yesil;

    match renk{
        Renk::Kirmizi => println!("Renk kırmızı!"),
        Renk::Yesil => println!("Renk yeşil!"),
        Renk::Mavi => println!("Renk Mavi!"),
    }
    
    let mesaj =Mesaj::Selamla(String::from("Merhaba !"));
    let sayilar =Mesaj::Sayisal(40);
    match sayilar {
        Mesaj::Selamla(icerik) => println!("Mesaj: {}",icerik),
        Mesaj::Sayisal(sayi) => println!("Bir sayı gönderildi: {} ",sayi),
        Mesaj::Bitir =>println!("Program sonlandı."),
    }
    
    let hareket = Hareket::Ileri{mesafe: 10};

    match hareket {
         Hareket::Ileri{mesafe} => println!("{} birim ileri",mesafe),
         Hareket::Sag{derece} => println!("{} derece sağa dön",derece),
         Hareket::Dur => println!("Hareket Durduruldu."),
    }
    
    let metin1 =Some("Merhaba");
    let metin2: Option<&str>=None;

    bul_metin(metin1);
    bul_metin(metin2);

    let Kullanici = Kullanici{
        ad: String::from("Ali"),
        yas: 30,
    };

    let durum = Durum::GirisYapildi(Kullanici);

    match durum {
        Durum::GirisYapildi(Kullanici) => println!("kullanıcı girişi yapıldı"),
        Durum::GirisBekleniyor =>println!("Kullıanıcı girişi bekleniyor"),
        Durum::Hata(mesaj) => println!("Hata: {}",mesaj),
    }


}