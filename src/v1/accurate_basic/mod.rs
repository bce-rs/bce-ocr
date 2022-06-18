//! [API document](https://cloud.baidu.com/doc/OCR/s/1k3h7y3db)
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

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

    pub fn post(
        self,
        access_token: impl Into<String>,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let access_token: String = access_token.into();
        let data = serde_url_params::to_string(&self)?;

        let response = ureq::post("https://aip.baidubce.com/rest/2.0/ocr/v1/accurate_basic")
            .set("Content-Type", "application/x-www-form-urlencoded")
            .query("access_token", &access_token)
            .send_string(&data)?
            .into_string()?;

        let response = serde_json::from_str(&response)?;

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

#[derive(Serialize, Default, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Response {
    pub log_id: u64,
    pub direction: Option<Direction>,
    pub words_result: Vec<Sentence>,
    pub pdf_file_size: Option<String>,
    pub paragraphs_result: Option<Vec<SentenceIdx>>
}

#[derive(Deserialize, Debug)]
pub struct SentenceIdx {
    words_result_idx: Vec<u32>,
} 

#[derive(Deserialize, Debug)]
pub struct Sentence {
    words: String,
    probability: Option<Probability>,
}

#[derive(Deserialize, Debug)]
pub struct Probability {
    average: f64,
    min: f64,
    variance: f64,
}

#[derive(Deserialize_repr, Debug)]
#[repr(i32)]
pub enum Direction {
    Unknown = -1,
    Normal,
    Counterclockwise90,
    Counterclockwise180,
    Counterclockwise270,
}

#[derive(Serialize, Debug)]
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
