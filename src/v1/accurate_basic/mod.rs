/// [API document](https://cloud.baidu.com/doc/OCR/s/1k3h7y3db)

enum Image {
    /// must be base64 + url_encode, not including data URI header
    RawImage(String),
    Url(String),
    /// must be base64 + url_encode, not including data URI header
    Pdf(String),
}

pub enum Lang {
    AutoDetect, // auto_detect
    ChineseEnglish, // CHN_ENG
    English, // ENG
    Japanese, // JAP
    Korean, // KOR
    French, // FRE
    Spanish, // SPA
    Portuguese, // POR
    German, // GER
    Italian, // ITA
    Russian, // RUS
    Danish, // DAN
    Nederlands, // DUT
    Malaysian, // MAL
    Svenska, // SWE
    Indonesia, // IND
    Polska, // POL
    Romana, // ROM
    Turkish, // TUR
    Ellinika, // GRE
    Magyar, // HUN
    Thai, // THA
    Viet, // VIE
    Arabi, // ARA
    Hindi, // HIN
}