mod lib;
use lib::Web_Code_Extractor::{WebCodeExtractor, WebCodeExtractorTrait};
fn main() {
    let my_coupon_codes: Vec<String> = vec!["CODE1".to_string(), "CODE2".to_string()];
    let mut web_code_extracted:WebCodeExtractor = WebCodeExtractor::new(
        "https://dealspotr.com/promo-codes/vegega.com/".to_owned(),
        "div.copy-code".to_owned(),
        my_coupon_codes);
        web_code_extracted.extract_codes();    

    // print!("{:?}",extracted_codes)
    // let response= getPage();
    // let html_content = response.unwrap();
    // print!("{}",html_content) 

}