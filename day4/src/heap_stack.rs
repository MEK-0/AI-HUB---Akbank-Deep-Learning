/*Stack (Yığın) Belleği
Özellikleri:

Sabit boyutlu veriler: Derleme zamanında boyutu bilinen veri türleri burada saklanır (örneğin, i32, f64, vb.).
Hızlıdır: Veriler, "son giren, ilk çıkar" (LIFO) prensibine göre saklanır ve kaldırılır.
Yerel kapsam: Stack'te saklanan değişkenler, fonksiyonun kapsamı bittiğinde otomatik olarak bellekten temizlenir.
Küçük boyut: Stack'in boyutu sınırlıdır. */

fn main (){
   let x =10; //X değeri stack'te saklanır
   println!("Stack'teki x:{}",x);

   let y =[1,2,3]; // Sabit boyutlu bir dizi, stack'te saklanır
   println!("Stack'teki dizi: {:?}",y);

   /*Heap (Küme) Belleği
Özellikleri:

Dinamik boyutlu veriler: Boyutu çalışma zamanında belirlenen veya değişen veri türleri burada saklanır (örneğin, String, Vec<T>).
Yavaş erişim: Verilere erişmek daha yavaştır çünkü heap üzerindeki veri için bir referans (pointer) kullanılır.
Kontrollü yaşam süresi: Heap'teki veriler manuel olarak tahsis edilir ve serbest bırakılır (Rust'ta bu işlem sahiplik sistemiyle otomatik yapılır).
Büyük veri: Büyük miktarda veri depolamak için uygundur. */

 let heap_string =String::from("Heap bellekte saklıyorum");
 println!("{}",heap_string);

 let heap_dizi = vec![1,2,3,4]; //vec heap'te saklanır 
 println!("Heap'teki dizi: {:?}",heap_dizi);

 let heap_sayi =Box::new(42); //Box ile veri heap'e taşınır
 println!("Heap'teki sayi: {}",heap_sayi);
}
/*
Stack hızlıdır: Çünkü veri doğrudan erişilebilir. Ancak yalnızca sabit boyutlu veriler için uygundur.
Heap esnektir: Veri boyutu büyüyüp küçülebilir. Ancak erişim için bir referansa ihtiyaç vardır.
Rust’ta sahiplik sistemi: Heap'teki verilerin yaşam döngüsünü otomatik olarak yönetir, bu sayede malloc veya free gibi işlemlere gerek kalmaz.
 */