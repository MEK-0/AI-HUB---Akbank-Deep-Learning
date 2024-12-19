//Vector and Strings,Heap vs Stack Memory
/*Vektörler (Vectors)

Vektörler (Vec<T>), Rust dilinde dinamik olarak büyüyüp küçülebilen dizilerdir. Heap (yığın) bellekte saklanır ve elemanların yönetimi için birçok güçlü metod sağlar.

Temel Özellikler:

Dinamik olarak boyut değiştirebilir.
Genel bir türdür (Vec<T> her türden eleman içerebilir).
Elemanlar bellek üzerinde ardışık olarak saklanır. */

fn main() {
     let mut sayilar = Vec::new(); // yeni bir vektör oluşturma

     // vektöre eleman ekleme
     sayilar.push(1);
     sayilar.push(2);
     sayilar.push(3);

     println!("Sayilar: {:?}",sayilar);

     //elemanlara erişim
     println!("İlk eleman: {}",sayilar[0]);
      
      //Vektör üzerinde gezinme 
     for sayi in &sayilar{
        println!("Sayi : {}",sayi);
     }

     // Eleman üzerinde döngü
     for(index,eleman) in sayilar.iter().enumerate(){
      println!("Index {}: {}",index,eleman);
     }

     // Son elemanı Kaldırma
     sayilar.pop();
     println!("Son hali: {:?}",sayilar);
 
     // Kapasite Yönetimi

     let mut v: Vec<i32> = Vec::with_capacity(5);
     println!("başlangıç kapasitesi: {}",v.capacity());

     //Eleman Ekleme
     for i in 1..=6 {
         v.push(i);
         println!("Eleman eklendi: {}, kapasite: {}",i,v.capacity());
     }
     println!("Son vektör: {:?} ",v);
}
