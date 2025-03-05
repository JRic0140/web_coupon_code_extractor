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
    pub coupon_code_from_the_page: Vec<CouponCode>,
    /// **The HTML document**
    document: Html,
    /// item code tag writed as a normal querySelector
    item_code_tag:String,
}

pub trait WebCodeExtractorTrait {
    // Extract the coupon code from the page
     fn new(url: String, item_code_tag:String, my_coupon_codes: Vec<String>) -> WebCodeExtractor;

     fn extract_codes(&mut self) -> &Vec<CouponCode>;

     fn get_code_Info(&self,code_name:String) ->CouponCode;

}

impl WebCodeExtractorTrait for WebCodeExtractor {
    // Extract the coupon code from the page
     fn new(url: String, item_code_tag:String, my_coupon_codes: Vec<String>) -> WebCodeExtractor {
        let response: Result<reqwest::blocking::Response, reqwest::Error> = reqwest::blocking::get(&url);
        let html_content: String = response.unwrap().text().unwrap();
        let document: Html = scraper::Html::parse_document(&html_content);

        WebCodeExtractor {
            url,
            my_coupon_codes,
            coupon_code_from_the_page: Vec::new(),
            document,
            item_code_tag
        }
    }

    fn extract_codes(&mut self) -> &Vec<CouponCode> {

        // Extract the Coupon list Elements as a selector
        let html_product_selector: scraper::Selector = scraper::Selector::parse(&self.item_code_tag).unwrap();
        let html_products: scraper::html::Select<'_, '_> = self.document.select(&html_product_selector);
        for coupon_tag in html_products {

            // Extract the Coupon code from the element
            self.coupon_code_from_the_page.push(
                CouponCode{
                    code_name:coupon_tag.select(&scraper::Selector::parse("input.dnone").unwrap())
                             .next().unwrap().value().attr("value").unwrap().to_owned(),
                    code_uses:coupon_tag.select(&scraper::Selector::parse("span.vam").unwrap())
                             .next().unwrap().text().collect::<Vec<_>>().join(" "),
                    description: "This is a coupon code".to_owned(),
                }
            );
        }
        return &self.coupon_code_from_the_page;
        // return self.coupon_code_from_the_page.to_owned();
        // Implementación para extraer el código de la página web
        // self.coupon_code_from_the_page = format!("Extracted code from {}", self.url);
    }

    fn get_code_Info(&self, code_name: String) -> CouponCode {
            self.coupon_code_from_the_page.iter().find(|coupon| coupon.code_name == code_name)
                .cloned()
                .unwrap_or_else(|| CouponCode {
                    code_name: "Not Found".to_string(),
                    code_uses: "0".to_string(),
                    description: "No description available".to_string(),
                })
        }
}   

#[derive(Debug, Clone)]
pub struct CouponCode{

    code_name: String,
    code_uses: String,
    description: String,

}

impl CouponCode{

}