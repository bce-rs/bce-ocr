use bce_ocr::{token, v1::accurate_basic};
use rstest::*;

const SECRET: &str = include_str!("res/key.csv");
const IMAGE: &str = include_str!("res/text.png.b64");

#[fixture]
fn access_token() -> String {
    let secret = SECRET.to_owned();
    let keys: Vec<&str> = secret.trim().split(",").collect();
    let api_key = keys[0];
    let secret_key = keys[1];

    let (access_token, _) = bce_ocr::token::acquire_access_token(api_key, secret_key).unwrap();
    access_token
}

#[rstest]
#[ignore]
fn test_accurate_basic(access_token: String) {
    use accurate_basic::Image;
    use accurate_basic::ImageType;
    let image = Image {
        image_data: IMAGE.to_owned(),
        image_type: ImageType::ImageFile,
    };

    let req = accurate_basic::Request::new(image);
    let response = req.post(access_token).unwrap();

    println!("{response}");
}
