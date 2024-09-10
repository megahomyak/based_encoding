mod entities;
mod operations;
use num_bigint::BigUint;

use entities::*;
use operations::represent;

fn main() {
    let original_page = Page {
        items: vec![
            Tag::H(HContents {
                tbody: TBody {
                    contents: "Heading 1".into(),
                },
                hl: Hl::One(),
            }),
            Tag::H(HContents {
                tbody: TBody {
                    contents: "Heading 2".into(),
                },
                hl: Hl::Two(),
            }),
            Tag::P(NNBody::Tags(TagSequence {
                items: vec![
                    Tag::El(NNBody::Text(DaletString {
                        contents: "Some ".into(),
                    })),
                    Tag::B(TBody {
                        contents: "bold".into(),
                    }),
                    Tag::I(TBody {
                        contents: "italic".into(),
                    }),
                    Tag::S(TBody {
                        contents: "strike".into(),
                    }),
                ],
            })),
            Tag::Br(),
            Tag::Code(CodeContents {
                tbody: TBody {
                    contents: "Hello world".into(),
                },
                tnullarg: TNullArg::Null(),
            }),
            Tag::Br(),
            Tag::Ul(TagSequence {
                items: vec![
                    Tag::El(NNBody::Text(DaletString {
                        contents: "abc".into(),
                    })),
                    Tag::El(NNBody::Tags(TagSequence {
                        items: vec![
                            Tag::El(NNBody::Text(DaletString {
                                contents: "def".into(),
                            })),
                            Tag::Ul(TagSequence {
                                items: vec![
                                    Tag::El(NNBody::Text(DaletString {
                                        contents: "defabc".into(),
                                    })),
                                    Tag::El(NNBody::Text(DaletString {
                                        contents: "defdef".into(),
                                    })),
                                ],
                            }),
                        ],
                    })),
                    Tag::El(NNBody::Text(DaletString {
                        contents: "xyz".into(),
                    })),
                ],
            }),
            Tag::Br(),
            Tag::P(NNBody::Tags(TagSequence {
                items: vec![
                    Tag::El(NNBody::Text(DaletString {
                        contents: "Lorem ipsum ".into(),
                    })),
                    Tag::Link(LinkContents {
                        body: Body::Tags(TagSequence {
                            items: vec![Tag::Img(TArg {
                                contents: "https://my-picture".into(),
                            })],
                        }),
                        targ: TArg {
                            contents: "https://some-link".into(),
                        },
                    }),
                    Tag::El(NNBody::Text(DaletString {
                        contents: " dolor sit amet consequetur adipiscing elit".into(),
                    })),
                ],
            })),
            Tag::Table(TagSequence {
                items: vec![
                    Tag::Tprow(TagSequence {
                        items: vec![
                            Tag::El(NNBody::Text(DaletString {
                                contents: "Col 1".into(),
                            })),
                            Tag::El(NNBody::Text(DaletString {
                                contents: "Col 2".into(),
                            })),
                            Tag::El(NNBody::Text(DaletString {
                                contents: "Col 3".into(),
                            })),
                        ],
                    }),
                    Tag::Trow(TagSequence {
                        items: vec![
                            Tag::El(NNBody::Text(DaletString {
                                contents: "Never gonna".into(),
                            })),
                            Tag::El(NNBody::Text(DaletString {
                                contents: "give you".into(),
                            })),
                            Tag::El(NNBody::Text(DaletString {
                                contents: "up".into(),
                            })),
                        ],
                    }),
                ],
            }),
        ],
    };
    let mut number = BigUint::ZERO;
    original_page.encode(&mut number);
    println!(
        "The encoded version takes {} bits or {} bytes",
        represent(number.clone(), &BigUint::from(2usize)).len(),
        represent(number.clone(), &BigUint::from(256usize)).len(),
    );
    println!("The compressed version takes {} bytes", {
        use std::io::Write;
        let bytes: Vec<u8> = represent(number.clone(), &BigUint::from(256usize))
            .into_iter()
            .map(|num| num.try_into().unwrap())
            .collect();
        let mut encoder =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&bytes).unwrap();
        encoder.finish().unwrap().len()
    });
    let decoded_page = Page::decode(&mut number);
    assert_eq!(original_page, decoded_page);
}
