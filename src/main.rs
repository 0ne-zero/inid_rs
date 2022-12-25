use std::io;

fn main(){
    println!("Select Mode:\n1. Check an INID\n2. Generate an INID");

    let mut buf = String::new();
    io::stdin().read_line(& mut buf).unwrap();
    buf = buf.trim().to_string();
    match buf.as_str(){
        "1" => {
            buf.clear();
            println!("Enter the INID that you want to check its validity:");
            io::stdin().read_line(& mut buf).unwrap();
            buf = buf.trim().to_string();
            match inid_rs::check_inid(&buf){
                Ok(v) => {
                    if v {
                        println!("\nThe \"{}\" INID is VALID !",buf);
                    }else{
                        println!("\nThe \"{}\" INID is NOT VALID !",buf);
                    }
                },
                Err(e) => {
                    match e {
                        inid_rs::error::INIDError::InvalidLength => {
                            println!("\nThe length of the INID is INVALID, Its length must be ten number, Now its {} number",buf.chars().count());
                        },
                        inid_rs::error::INIDError::NotNumerical => {
                            println!("\nAn INID must be all in numberical, Now it's \"{}\"",buf);
                        },
                    }
                }
            }
        },
        "2" => {
            buf.clear();
            println!("\nIf you want to set a prefix, Please enter it, If you don't want, just press the enter:");
            io::stdin().read_line(& mut buf).unwrap();
            buf = buf.trim().to_string();
            let res = {
                if buf.chars().count() > 0{
                    inid_rs::generate_id(Some(&buf))
                }else{
                    inid_rs::generate_id(None)
                }
            };
            
            match res{
                Ok(v) => {
                    println!("\nThe generated INID is: {}",v)
                },
                Err(e) => {
                    match e{
                        inid_rs::error::INIDError::InvalidLength => {
                            println!("\nThe length of the INID is INVALID, Its length must be ten number, Now its {} number",buf.chars().count());
                        },
                        inid_rs::error::INIDError::NotNumerical => {
                            println!("\nAn INID must be all in numberical, Now it's \"{}\"",buf);
                        },
                    }
                }
            }
        },
        _ => {
            println!("\nYou Selected Invalid mode !")
        },
    }

}