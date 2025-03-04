mod lib;

use lib::Web_Code_Extractor::WebCodeExtractor;

fn main() {
    let my_coupon_codes: Vec<String> = vec!["CODE1".to_string(), "CODE2".to_string()];
    let mut web_code_extracted:WebCodeExtractor = WebCodeExtractor::new(
        "https://dealspotr.com/promo-codes/vegega.com/74014427".to_owned(),
        "input.dnone".to_owned(),
        my_coupon_codes);    
    let extracted_codes = web_code_extracted.extract_total_codes();

    
        
    print!("{:?}",extracted_codes)



    // let response= getPage();
    // let html_content = response.unwrap();
    // print!("{}",html_content) 
    
}

