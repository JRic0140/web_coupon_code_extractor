use reqwest;
/// web_code_extractor module extracts *coupon codes* from a web page
/// ## returns coupon codes from the page
pub struct web_code_extractor {
    pub url: String,
    pub my_coupon_codes: Vec<String>,
    pub coupon_code_from_the_page: String,
}

pub trait web_code_extractor_trait {
    // Extract the coupon code from the page
     fn new(url: String, my_coupon_codes: Vec<String>) -> web_code_extractor;

     fn extract_total_codes(&mut self) -> Vec<String>;

     fn get_code_Info(&self) -> &str;

}

impl web_code_extractor_trait for web_code_extractor {
    // Extract the coupon code from the page
     fn new(url: String, my_coupon_codes: Vec<String>) -> web_code_extractor {
        let response = reqwest::blocking::get("https://www.scrapingcourse.com/ecommerce/");
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);
        
        // Extract the Coupon list Elements as a selector
        let html_product_selector = scraper::Selector::parse("li.product").unwrap();
        let html_products = document.select(&html_product_selector);



        web_code_extractor {
            url,
            my_coupon_codes,
            coupon_code_from_the_page: String::new(),
        }
    }

     fn extract_total_codes(&mut self) -> Vec<String> {
        

        todo!();
        // Implementación para extraer el código de la página web
        // self.coupon_code_from_the_page = format!("Extracted code from {}", self.url);
    }

     fn get_code_Info(&self) -> &str {
        todo!()
    }

}