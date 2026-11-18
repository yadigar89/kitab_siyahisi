use std::{io, fs};

pub struct Kitab{
   pub ad: String,
   pub muellif: String,
   pub il: i32,
}
pub fn setirden_kitab(setir: &str) ->Result<Kitab, String>{

    let hisseler: Vec<&str> = setir.split(',').collect();

    if hisseler.len() != 3{

        return Err(String::from("Setirde 3 hisse olmalidir"));
    }
    let il = hisseler[2]
       .trim()
       .parse::<i32>() 
       .map_err(|_| String::from("Il ededi duz deyil"))?;

    Ok(Kitab{
        ad: String::from(hisseler[0].trim()),
        muellif: String::from(hisseler[1].trim()),
        il,
    })
}
pub fn fayildan_oxu() ->Vec<Kitab>{

    let mut kitablar: Vec<Kitab> = Vec::new();

    match fs::read_to_string("kitablar.txt"){
        Ok(metn) =>{
            for setir in metn.lines(){
                match setirden_kitab(setir){
                    Ok(k) => kitablar.push(k),
                    Err(e) => println!("Xeta: {e} -> {setir}"),
                }
            }
        }
        Err(_) =>println!("Fayl tapimadi, bos siyahi ile baslayiriq"),
    }
    kitablar
}
pub fn fayla_yaz(kitablar: &[Kitab]) ->Result<(), std::io::Error>{

    let mut metn = String::new();

    for k in kitablar{
        metn.push_str(&format!("{}, {}, {}\n", k.ad, k.muellif, k.il));
    }
    fs::write("kitablar.txt", metn)
}
pub fn oxu(izah: &str) ->String{

    println!("{izah}");

    let mut giris = String::new();

    match io::stdin().read_line(&mut giris){
        Ok(_) =>giris.trim().to_string(),
        Err(_) =>String::new(),
    }
}
pub fn tap<'a>(kitablar: &'a [Kitab], ad: &str) ->Option<&'a Kitab>{

    kitablar.iter().find(|k| k.ad == ad)
}