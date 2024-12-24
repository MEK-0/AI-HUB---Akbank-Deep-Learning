/*? Operatörü ile Hata Yönetimi ? operatörü, hata yönetimini basitleştirir ve bir hatayla karşılaşılırsa otomatik olarak dışarı döner. */

use std::fs::File;
use std::io::{self,Read};


fn dosya_oku(dosya_adi: &str)-> Result<String, io::Error>{
    let mut dosya =File::open(dosya_adi)?; // Eğer hata olursa burada döner
    let mut icerik = String::new();
    dosya.read_to_string(&mut icerik)?;
    Ok(icerik)
}

fn main(){
    match dosya_oku("mevcut_dosya.txt"){
        Ok(icerik) => println!("Dosya içeriği: \n{}",icerik),
        Err(hata) => println!("Hata:{}",hata),
    }
}