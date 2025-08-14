use std::fs::File;
use std::io::{self, Read};

fn main() {
    let division1 = divide1(10.0 , 0.0);
    match division1 {
        Some(x) => println!("Resultado {}",x),
        None => println!("No se puede dividir por 0")
    };   
    let division2 = divide2(10.0 , 2.0);
    match division2 {
        Ok (x) => println!("Resultado: {}", x),
        Err (x) => println!("{}", x)
    }
    
    let v = vec![1,2,3,4,5];
    v.iter().for_each(|x| println!("{}", x));

    let s = "Hola mundo".to_string();
    s.chars().map(|n| {
        println! ("{}", n);
    }).collect::<Vec<_>>();    
}

fn leer_archivo () -> Result <i32, io::Error> {
    let f = File::open("numero.txt");
    let mut f = match f{
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => {
            match s.trim().parse::<i32>(){
                Ok(num) => Ok(num),
                Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "No se pudo convertir en i32")),
            }
        }
        Err(e) => Err(e),
    }
}

fn leer_archivo_consiso () -> Result<i32, io::Error> {
    let mut f = File::open("numero.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    s.trim().parse::<i32>().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "No se pudo convertir en i32"))
}

fn divide1 (num:f64, den: f64) -> Option<f64> {
    if den == 0.0 {
        None
    } else {
        Some( num / den )
    }
}

fn divide2 (num:f64, den:f64) -> Result<f64, String> {
    if den == 0.0 {
        Err ("No se puede dividir por 0".to_string())
    } else {
        Ok(num / den)
    }
}

