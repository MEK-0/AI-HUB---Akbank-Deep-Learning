// Control flow Constructs 2 , functions and method syntax
fn main() {
   // aralıklarla for döngüsü

   for i in 1..5{ // 1 den 4 e kadar döner
      println!("değer {}",i);
   }
     println!("");
    // Ters Döngü
    
    for i in (1..5).rev(){
        println!("değer {}",i);
    }
      println!("");
    // Koleksiyon üzerinde yenileme 
    
    let meyveler = ["elma","armut","kiraz"];

    for meyve in meyveler.iter(){
         println!("Meyve {}",meyve);
    }

    // for ile index ve değer kullanımı

    let meyveler1 =["elma","armut","kiraz"];

    for (index,meyve) in meyveler.iter().enumerate(){
         println!("{}. Meyve: {}",index+1,meyve);
    }
    println!("");

    // Nested For Loop
    
    for i in 1..=5{
        for j in 1..=5{
            print!("{} ",i*j);
        } 
        println!();
    }

    // Etiketli break 

    'dis_dongu: for i in 0..5{
        for j in 0..5{
            if i ==2 && j ==2 {
                 break 'dis_dongu;  // dış döngüden çıkılır
            }
            println!("i: {}, j:{}",i,j);
        }
    }
    println!();
     
}
