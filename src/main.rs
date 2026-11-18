mod kitab;

use std::{fs, io};
use kitab::{Kitab, fayildan_oxu, fayla_yaz, setirden_kitab, oxu, tap};
enum Secim{
    Siyahi,
    Elave,
    Axtar,
    Sil,
    Saxla,
    Cixis,
}

fn secimi_oxu(metn: &str) ->Option<Secim>{

    match metn{
        "1" =>Some(Secim::Siyahi),
        "2" =>Some(Secim::Elave),
        "3" =>Some(Secim::Axtar),
        "4" =>Some(Secim::Sil),
        "5" =>Some(Secim::Saxla),
        "6" =>Some(Secim::Cixis),
        _ => None,
    }
}

fn main() {

    let mut kitablar: Vec<Kitab> = fayildan_oxu();

    

    loop{
        println!("\n1 - Kitablarin siyahisi.");
        println!("2 - Kitab elave et.");
        println!("3 - Kitab axtar");
        println!("4 - Kitab sil");
        println!("5 - Kitab saxla");
        println!("6 - Cixis");

        let giris = oxu("Secim edin:");

        match secimi_oxu(&giris){
            Some(Secim::Siyahi) =>{
                for k in &kitablar{
                    println!("{}, {}, {}",k.ad, k.muellif, k.il);
                }
            }
            Some(Secim::Elave)=>{
                let ad = oxu("Ad:");
                let muellif = oxu("Muellif");
                let il_metni = oxu("Il");

                match il_metni.parse::<i32>(){
                    Ok(il) =>{
                        kitablar.push(Kitab{ad, muellif, il});
                        println!("Elave edildi");
                    }
                    Err(_) =>println!("Il ededi duz deyil"),
                } 
            }
            Some(Secim::Axtar) =>{
                let ad = oxu("Axtarilan ad:");

                match tap(&kitablar, &ad) {
                    Some(k) =>println!("Tapildi: {}, {}, {}",k.ad, k.muellif, k.il),
                    None =>println!("Bele kitab yoxdur"),
                }
            }
            Some(Secim::Sil) =>{

                let ad = oxu("Silinecek kitab");

                match kitablar.iter().position(|k| k.ad == ad){
                    Some(yer) =>{
                        kitablar.remove(yer);
                        println!("Silindi");
                    }
                    None =>println!("Bele kitab yoxdur")
                }
            }
            Some(Secim::Saxla) =>{
                match fayla_yaz(&kitablar){
                    Ok(_) =>println!("Saxlanlidi"),
                    Err(e) =>println!("Xeta: {e}"),
                }
            }
            Some(Secim::Cixis) =>{
                println!("Gorusenedek");
                break;
            }
            None =>println!("Bele secim yoxdur."),
        }
    }

    println!("Hazirda oxudugum kitab sayi: {}.",kitablar.len());
}