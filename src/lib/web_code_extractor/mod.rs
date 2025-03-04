use reqwest;
use scraper::Html;
/// # web_code_extractor
/// Extracts *coupon codes* from a web page and
/// returns information of the coupon codes from the page to analityze
pub struct WebCodeExtractor {
    /// **URL of the page**
    url: String,
    /// **The coupon codes that you have**
    pub my_coupon_codes: Vec<String>,
    /// **The coupon code extracted from the page**
    pub coupon_code_from_the_page: String,
    /// **The HTML document**
    document: Html,
    /// item code tag writed as a normal querySelector
    item_code_tag:String,
}

impl WebCodeExtractor {
    // Extract the coupon code from the page
    pub fn new(url: String, item_code_tag:String, my_coupon_codes: Vec<String>) -> WebCodeExtractor {
        let response = reqwest::blocking::get(&url);
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);

        WebCodeExtractor {
            url,
            my_coupon_codes,
            coupon_code_from_the_page: String::new(),
            document,
            item_code_tag
        }
    }

    pub fn extract_total_codes(&mut self) -> Vec<String> {

        // Extract the Coupon list Elements as a selector
        let html_product_selector = scraper::Selector::parse(&self.item_code_tag).unwrap();
        let html_products = self.document.select(&html_product_selector);
        for coupon_tag in html_products {

            // Extract the Coupon code from the element
            let coupon_code = coupon_tag.attr("value").unwrap();
            self.my_coupon_codes.push(coupon_code.to_owned());
        }

        return self.my_coupon_codes.to_owned();
        // Implementación para extraer el código de la página web
        // self.coupon_code_from_the_page = format!("Extracted code from {}", self.url);
    }

     fn get_code_Info(&self) -> &str {
        todo!("Display Coupons Info of the store")
    }

}   