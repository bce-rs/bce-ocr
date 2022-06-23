mod token;
use token::access_token;

use bce_ocr::v1::accurate_basic;

use rstest::*;

const IMAGE: &str = include_str!("res/text.png.b64");

#[rstest]
#[ignore]
fn accurate_basic(access_token: String) {
    use accurate_basic::Image;
    use accurate_basic::ImageType;
    let image = Image {
        image_data: IMAGE.to_owned(),
        image_type: ImageType::ImageFile,
    };

    let mut req = accurate_basic::Request::new(image);

    req.detect_direction = Some(true);
    req.paragraph = Some(true);
    req.probability = Some(true);

    let response = req.post(&access_token).unwrap();
    let words = &response.words_result[0].words;

    assert_eq!(words, "bug是常事")
}
