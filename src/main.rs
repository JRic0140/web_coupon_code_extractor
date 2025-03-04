mod lib;

use lib::web_code_extractor::{web_code_extractor,web_code_extractor_trait};

fn main() {
    let my_coupon_codes: Vec<String> = vec!["CODE1".to_string(), "CODE2".to_string()];
    let web_code_extracted:web_code_extractor = web_code_extractor::new(
        "https://dealspotr.com/promo-codes/vegega.com/74014427".to_owned()
        , my_coupon_codes);    
    
    print!("{}",web_code_extracted.get_code_Info());
    
    
    



    // let response= getPage();
    // let html_content = response.unwrap();
    // print!("{}",html_content) 
    
}

