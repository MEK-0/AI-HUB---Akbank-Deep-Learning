/*Result Enum
Result türü, işlemin başarılı mı başarısız mı olduğunu ifade eder. İki farklı durumu vardır:

Ok(value): İşlem başarılı ve değer döndürür.
Err(error): İşlem başarısız ve hata döndürür. */

fn dosya_ac(dosya_adi: &str) -> Result<String, String> {
    if dosya_adi == "mevcut_dosya.txt" {
        Ok("Dosya başarıyla açıldı!".to_string())
    } else {
        Err("Dosya bulunamadı!".to_string())
    }
}

fn main() {
    match dosya_ac("mevcut_dosya.txt") {
        Ok(mesaj) => println!("{}", mesaj),
        Err(hata) => println!("Hata: {}", hata),
    }

    match dosya_ac("olmayan_dosya.txt") {
        Ok(mesaj) => println!("{}", mesaj),
        Err(hata) => println!("Hata: {}", hata),
    }
}
