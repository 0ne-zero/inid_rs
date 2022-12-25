//! INID (Iran National Identifier)
//! This crate can generate INID and also even can generate INID with prefix.
//! Another feature of this crate is that it can check a INID is valid or not.

use std::{vec};
use error::INIDError;
use rand::Rng;

mod error;

const DECIMAL_RADIX:u8 = 10;
type INID = Vec<u8>;


/// Every INID has a checksum number at the end
/// This function Generates that checksum for an INID
fn generate_checksum(nid:&INID) -> u8{
    let mut n_sum: u16 = 0;
        for p in (2..=10).rev(){
            n_sum += (nid[10 - p] as u16) * (p as u16);
        }
    let r = n_sum % 11 ;
    let checksum:u8 = if r < 2 {r as u8} else {11 - r as u8};
    checksum
}

/// Generates INID, also can generate with prefix
/// 
/// # Examples
/// ```
/// let inid =rs_inid::generate_id(None).unwrap();
/// // Or we can pass a prefix to it
/// let inid = rs_inid::generate_id(Some(&"123".to_string())).unwrap();
/// ```
pub fn generate_id(prefix:Option<&String>) -> Result<String,INIDError>{

    match prefix {
       Some(prefix) =>{

        let mut inid = convert_str_to_inid(&prefix)?;

        for _ in 0..9-inid.len(){
            let random_number = rand::thread_rng().gen_range(0..=9);
            inid.push(random_number);
        }

        let checksum = generate_checksum(&inid);
        inid.push(checksum);

        return Ok(convert_inid_to_str(&inid));
       },
       None =>{
        let mut inid :Vec<u8> = vec![];

        for _ in 1..=9{
            let random_number = rand::thread_rng().gen_range(0..=9);
            inid.push(random_number);
        }

        let checksum = generate_checksum(&inid);
        inid.push(checksum);

        Ok(convert_inid_to_str(&inid))
       }
   }
}

/// Chekcs a INID is valid or not
/// 
/// # Examples
/// ```
/// let inid = String::from("1234567890");
/// let res = rs_inid::check_inid(&inid).unwrap();
/// assert_eq!(res,false);
/// ```
pub fn check_inid(s_inid:&String) -> Result<bool,INIDError>{
    let inid = convert_str_to_inid(s_inid)?;
    Ok(generate_checksum(&inid) == inid[inid.len()-1])   
}

/// Converts an INID to String
/// INID is a alias for Vec<u8>
fn convert_inid_to_str(inid:&INID) -> String{
    let mut s = String::from("");
    for item in inid{
        s.push_str(&item.to_string());
    }
    s
}
/// Converts a String to an INID
fn convert_str_to_inid(input:&String) -> Result<INID,INIDError>{
    
    let mut nid: Vec<u8> = vec![];
    for s in input.chars(){
        if s == ' '{
            continue;
        }
        match s.to_digit(DECIMAL_RADIX as u32){
            Some(char_digit) => nid.push(char_digit as u8),
            None => {return Err(error::INIDError::InvalidFormat);},
        }
    }
    Ok(nid)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generate_id_test() {
        // Test without prefix
        let result = generate_id(None);
        match result{
            Ok(v) => {
                
                let inid = match convert_str_to_inid(&v){
                    Ok(v) =>v,
                    Err(e) => panic!("{}",e)
                };

                let res_len = inid.len();

                // Check length
                assert_eq!(res_len,10);
                
                // Get last char of String
                assert_eq!(inid[res_len -1],generate_checksum(&inid))
            },
            Err(e) => {panic!("{}",e)},
        }
        // Test with prefix
        let result = generate_id(Some(&"123".to_string()));
        match result{
            Ok(v) => {
                
                let inid = match convert_str_to_inid(&v){
                    Ok(v) =>v,
                    Err(e) => panic!("{}",e)
                };

                let res_len = inid.len();

                // Check length
                assert_eq!(res_len,10);
                
                // Get last char of String
                assert_eq!(inid[res_len -1],generate_checksum(&inid))
            },
            Err(e) => {panic!("{}",e)},
        }
    }

    #[test]
    fn check_id_and_generate_checksum_test(){
        let ids:Vec<INID> = vec![vec![2,3,5,7,2,1,4,8,1,3],vec![8,7,4,7,6,2,3,6,6,7],vec![4,5,7,5,9,0,0,7,6,1]];
        for item in ids{
            let checksum = generate_checksum(&item);
            assert_eq!(checksum,item[item.len()-1]);
        }
    }
    
}
