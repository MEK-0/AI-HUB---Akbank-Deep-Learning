// day1 topics -> variables , data type , Control flow constructs ,functions and method syntax , patter matching and destructuring 

fn main() {
    // variables 
    let x = 5; // Immutable (değiştirilemez)
    println!("x: {}", x);

    let mut y = 10; // Mutable (değiştirilebilir)
    println!("y (ilk hali): {}", y);

    y = 20; // Yeni değer atayabiliriz
    println!("y (güncellenmiş): {}", y);

   println!(" ");
    // Data types 

    let a: i32 = 42; // 32 bit işaretli tamsayı
    let b: f64 = 3.14; // 64 bit ondalık
    let c: bool = true; // Boolean
    let d: char = 'R'; // Karakter

    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);

    let array = [1, 2, 3, 4, 5]; // Dizi
    println!("İlk eleman: {}", array[0]);

    let tuple = (10, 20.5, 'X'); // Tuple
    println!("Tuple Data: {} ve {}", tuple.0, tuple.2);
    
    // Control flow Constructs :D
            // Basics if   

    let yas = 20;

    if yas >= 18 {
        println!("Reşitsiniz!");
    } else {
        println!("Reşit değilsiniz.");
    }

        // rust dilinde if değer döndürebilir 

        let yas1 = 17;

        let durum = if yas >= 18 { "Reşit" } else {"Reşit Değil"};
       
         println!("Durum {}",durum);
     
     // Birden fazla şart 
     
      let not =85;

      if not >=90{
        println!("Pekiyi");
      } else if not >=70{
         println!("iyi");
      } else if not >=50 {
         println!("geçer");
      } else {
         println!("Başarısız");
      }

      //  loop(Sonsuz döngü) 

       let mut sayac =0;

       loop{
         println!("sayac: {}",sayac);
         sayac+=1;

         if sayac == 5{
            break; // döngüden çıkar 
         }
       }
         
         // While Döngüsü 

         let mut sayac1 = 0;

         while sayac1 < 10{
            println!("Sayac: {}",sayac1);
            sayac1+=1;
         }

}
