mod based_number;
use based_number::{BasedDigit, BasedNumber, ValidBase};

fn base(n: usize) -> ValidBase {
    ValidBase::new(n).unwrap()
}

fn based_digit(base: ValidBase, value: usize) -> BasedDigit {
    BasedDigit::new(base, value).unwrap()
}

trait Entity {
    fn encode(&self, based_number: &mut BasedNumber);
    fn decode(based_number: &mut BasedNumber) -> Self;
}

struct Page {
    data: Vec<Tag>,
}

enum Tag {
    El(NNBody),
    H(TBody, Hl),
    P(NNBody),
    Br,
    Ul(Vec<Tag>),
    Ol(Vec<Tag>),
    Row(Vec<Tag>, AlignArg),
    Link(Body, TArg),
    Navlink(Body, TArg),
    Btn(Body, TArg),
    Navbtn(Body, TArg),
    Img(TArg),
    Table(Vec<Tag>),
    Trow(Vec<Tag>),
    Tprow(Vec<Tag>),
    Hr,
    B(TBody),
    I(TBody),
    Bq(NNBody),
    Footlnk(NNArg),
    Footn(TBody, NNArg),
    A(NNArg),
    S(TBody),
    Sup(TBody),
    Sub(TBody),
    Disc(NNBody),
    Block(NNBody, AlignArg),
    Carousel(Vec<Tag>),
    Code(TBody, TNullArg),
    Pre(TBody),
    Meta(TBody, TArg),
}

enum Body {
    Text(String),
    Tags(Vec<Tag>),
    Null,
}

enum NNBody {
    Text(String),
    Tags(Vec<Tag>),
}

type TBody = String;

enum TNullArg {
    Text(String),
    Null,
}

type TArg = String;

enum NNArg {
    Text(String),
    Number(u8),
}

enum AlignArg {
    Start,
    Center,
    End,
}

enum Hl {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
}

fn main() {
    let mut based_number = based_number::BasedNumber::new();
    based_number.write(based_digit(base(10), 4));
    based_number.write(based_digit(base(20), 6));
    println!("{}", based_number.read(base(20)));
    println!("{}", based_number.read(base(10)));
    based_number.convert(base(2));
}
