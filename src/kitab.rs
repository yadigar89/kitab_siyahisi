use std::{io, fs};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]

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
pub fn fayldan_oxu() ->Option<Vec<Kitab>> {
    let metn = std::fs::read_to_string("kitablar.json").ok()?;

    serde_json::from_str(&metn).ok()
}
pub fn fayla_yaz(kitablar: &Vec<Kitab>) ->Result<(), String> {
    let metn = serde_json::to_string_pretty(kitablar).map_err(|e| e.to_string())?;

    std::fs::write("kitablar.json", metn).map_err(|e| e.to_string())?;

    Ok(())
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