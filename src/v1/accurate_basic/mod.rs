//! [API document](https://cloud.baidu.com/doc/OCR/s/1k3h7y3db)
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct Request {
    image: Option<String>,
    url: Option<String>,
    pdf_file: Option<String>,
    pub language_type: Option<Lang>,
    pub pdf_file_num: Option<String>,
    pub detect_direction: Option<bool>,
    pub paragraph: Option<bool>,
    pub probability: Option<bool>,
}

impl Request {
    pub fn new(image: Image) -> Self {
        match image.image_type {
            ImageType::ImageFile => Request {
                image: Some(image.image_data),
                ..Default::default()
            },
            ImageType::URL => Request {
                url: Some(image.image_data),
                ..Default::default()
            },
            ImageType::PDF => Request {
                pdf_file: Some(image.image_data),
                ..Default::default()
            },
        }
    }

    pub fn probability(mut self, probility: bool) -> Self {
        self.probability = Some(probility);
        self
    }

    pub fn post(self, access_token: impl Into<String>) -> Result<String, Box<dyn std::error::Error>> {
        let access_token: String = access_token.into();
        let data = serde_url_params::to_string(&self)?;

        let response = ureq::post("https://aip.baidubce.com/rest/2.0/ocr/v1/accurate_basic")
            .set("Content-Type", "application/x-www-form-urlencoded")
            .query("access_token", &access_token)
            .send_string(&data)?
            .into_string()?;

        Ok(response)
    }
}

pub enum ImageType {
    /// base64, can be jpeg/png/bmp
    ImageFile,
    /// URI to the image file. remember do url encode
    URL,
    /// base64
    PDF,
}

pub struct Image {
    pub image_data: String,
    pub image_type: ImageType,
}

#[derive(Serialize)]
pub enum Lang {
    #[serde(rename = "auto_detect")]
    AutoDetect,
    #[serde(rename = "CHN_ENG")]
    ChineseEnglish,
    #[serde(rename = "ENG")]
    English,
    #[serde(rename = "JAP")]
    Japanese,
    #[serde(rename = "KOR")]
    Korean,
    #[serde(rename = "FRE")]
    French,
    #[serde(rename = "SPA")]
    Spanish,
    #[serde(rename = "POR")]
    Portuguese,
    #[serde(rename = "GER")]
    German,
    #[serde(rename = "ITA")]
    Italian,
    #[serde(rename = "RUS")]
    Russian,
    #[serde(rename = "DAN")]
    Danish,
    #[serde(rename = "DUT")]
    Nederlands,
    #[serde(rename = "MAL")]
    Malaysian,
    #[serde(rename = "SWE")]
    Svenska,
    #[serde(rename = "IND")]
    Indonesia,
    #[serde(rename = "POL")]
    Polska,
    #[serde(rename = "ROM")]
    Romana,
    #[serde(rename = "TUR")]
    Turkish,
    #[serde(rename = "GRE")]
    Ellinika,
    #[serde(rename = "HUN")]
    Magyar,
    #[serde(rename = "THA")]
    Thai,
    #[serde(rename = "VIE")]
    Viet,
    #[serde(rename = "ARA")]
    Arabi,
    #[serde(rename = "HIN")]
    Hindi,
}
