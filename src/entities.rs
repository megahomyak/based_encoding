// This file was generated automatically from entities.json. DO NOT EDIT THIS FILE MANUALLY!
use crate::operations::{read, write};
use num_bigint::BigUint;
use num_traits::ops::checked::CheckedSub;

#[derive(Debug, PartialEq, Eq)]
pub struct Page {
    pub items: Vec<Tag>,
}

impl Page {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        Tag::base() + BigUint::from(1usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        write(number, &Self::base(), &BigUint::from(0usize));
        for item in self.items.iter().rev() {
            item.encode(number);
            let item = read(number, &Tag::base());
            write(number, &Self::base(), &(item + &BigUint::from(1usize)));
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let mut items = Vec::new();
        loop {
            let num = read(number, &Self::base());
            let Some(decremented) = num.checked_sub(&BigUint::from(1usize)) else {
                break;
            };
            write(number, &Tag::base(), &decremented);
            items.push(Tag::decode(number));
        }
        Self { items }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TagSequence {
    pub items: Vec<Tag>,
}

impl TagSequence {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        Tag::base() + BigUint::from(1usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        write(number, &Self::base(), &BigUint::from(0usize));
        for item in self.items.iter().rev() {
            item.encode(number);
            let item = read(number, &Tag::base());
            write(number, &Self::base(), &(item + &BigUint::from(1usize)));
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let mut items = Vec::new();
        loop {
            let num = read(number, &Self::base());
            let Some(decremented) = num.checked_sub(&BigUint::from(1usize)) else {
                break;
            };
            write(number, &Tag::base(), &decremented);
            items.push(Tag::decode(number));
        }
        Self { items }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DaletString {
    pub contents: std::string::String,
}

impl DaletString {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(256usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        write(number, &Self::base(), &BigUint::from(0usize));
        for byte in self.contents.as_bytes().iter().rev() {
            write(number, &Self::base(), &BigUint::from(*byte as usize));
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let mut bytes = Vec::new();
        loop {
            let byte = read(number, &Self::base());
            if byte == BigUint::from(0usize) {
                break;
            }
            bytes.push(u8::try_from(&byte).unwrap());
        }
        Self {
            contents: std::string::String::from_utf8(bytes).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HContents {
    pub tbody: TBody,
    pub hl: Hl,
}

impl HContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        TBody::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.hl.encode(number);
        self.tbody.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let tbody = TBody::decode(number);
        let hl = Hl::decode(number);
        Self { tbody, hl }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RowContents {
    pub tags: TagSequence,
    pub alignment: AlignArg,
}

impl RowContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        TagSequence::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.alignment.encode(number);
        self.tags.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let tags = TagSequence::decode(number);
        let alignment = AlignArg::decode(number);
        Self { tags, alignment }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinkContents {
    pub body: Body,
    pub targ: TArg,
}

impl LinkContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        Body::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.targ.encode(number);
        self.body.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let body = Body::decode(number);
        let targ = TArg::decode(number);
        Self { body, targ }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NavlinkContents {
    pub body: Body,
    pub targ: TArg,
}

impl NavlinkContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        Body::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.targ.encode(number);
        self.body.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let body = Body::decode(number);
        let targ = TArg::decode(number);
        Self { body, targ }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BtnContents {
    pub body: Body,
    pub targ: TArg,
}

impl BtnContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        Body::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.targ.encode(number);
        self.body.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let body = Body::decode(number);
        let targ = TArg::decode(number);
        Self { body, targ }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FootnContents {
    pub tbody: TBody,
    pub nnarg: NNArg,
}

impl FootnContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        TBody::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.nnarg.encode(number);
        self.tbody.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let tbody = TBody::decode(number);
        let nnarg = NNArg::decode(number);
        Self { tbody, nnarg }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NavbtnContents {
    pub body: Body,
    pub targ: TArg,
}

impl NavbtnContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        Body::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.targ.encode(number);
        self.body.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let body = Body::decode(number);
        let targ = TArg::decode(number);
        Self { body, targ }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockContents {
    pub nnbody: NNBody,
    pub alignment: AlignArg,
}

impl BlockContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        NNBody::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.alignment.encode(number);
        self.nnbody.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let nnbody = NNBody::decode(number);
        let alignment = AlignArg::decode(number);
        Self { nnbody, alignment }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeContents {
    pub tbody: TBody,
    pub tnullarg: TNullArg,
}

impl CodeContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        TBody::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.tnullarg.encode(number);
        self.tbody.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let tbody = TBody::decode(number);
        let tnullarg = TNullArg::decode(number);
        Self { tbody, tnullarg }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MetaContents {
    pub tbody: TBody,
    pub targ: TArg,
}

impl MetaContents {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        TBody::base()
    }
    pub fn encode(&self, number: &mut BigUint) {
        self.targ.encode(number);
        self.tbody.encode(number);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let tbody = TBody::decode(number);
        let targ = TArg::decode(number);
        Self { tbody, targ }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tag {
    El(NNBody),
    H(HContents),
    P(NNBody),
    Br(),
    Ul(TagSequence),
    Ol(TagSequence),
    Row(RowContents),
    Link(LinkContents),
    Navlink(NavlinkContents),
    Btn(BtnContents),
    Navbtn(NavbtnContents),
    Img(TArg),
    Table(TagSequence),
    Trow(TagSequence),
    Tprow(TagSequence),
    Hr(),
    B(TBody),
    I(TBody),
    Bq(NNBody),
    Footlnk(NNArg),
    Footn(FootnContents),
    A(NNArg),
    S(TBody),
    Sup(TBody),
    Sub(TBody),
    Disc(NNBody),
    Block(BlockContents),
    Carousel(TagSequence),
    Code(CodeContents),
    Pre(TBody),
    Meta(MetaContents),
}

impl Tag {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(31usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        match self {
            Self::El(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(0usize));
            }
            Self::H(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(1usize));
            }
            Self::P(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(2usize));
            }
            Self::Br() => {
                write(number, &Self::base(), &BigUint::from(3usize));
            }
            Self::Ul(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(4usize));
            }
            Self::Ol(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(5usize));
            }
            Self::Row(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(6usize));
            }
            Self::Link(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(7usize));
            }
            Self::Navlink(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(8usize));
            }
            Self::Btn(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(9usize));
            }
            Self::Navbtn(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(10usize));
            }
            Self::Img(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(11usize));
            }
            Self::Table(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(12usize));
            }
            Self::Trow(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(13usize));
            }
            Self::Tprow(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(14usize));
            }
            Self::Hr() => {
                write(number, &Self::base(), &BigUint::from(15usize));
            }
            Self::B(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(16usize));
            }
            Self::I(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(17usize));
            }
            Self::Bq(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(18usize));
            }
            Self::Footlnk(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(19usize));
            }
            Self::Footn(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(20usize));
            }
            Self::A(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(21usize));
            }
            Self::S(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(22usize));
            }
            Self::Sup(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(23usize));
            }
            Self::Sub(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(24usize));
            }
            Self::Disc(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(25usize));
            }
            Self::Block(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(26usize));
            }
            Self::Carousel(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(27usize));
            }
            Self::Code(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(28usize));
            }
            Self::Pre(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(29usize));
            }
            Self::Meta(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(30usize));
            }
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == BigUint::from(0usize) {
            let associated_value = NNBody::decode(number);
            return Self::El(associated_value);
        }
        if num == BigUint::from(1usize) {
            let associated_value = HContents::decode(number);
            return Self::H(associated_value);
        }
        if num == BigUint::from(2usize) {
            let associated_value = NNBody::decode(number);
            return Self::P(associated_value);
        }
        if num == BigUint::from(3usize) {
            return Self::Br();
        }
        if num == BigUint::from(4usize) {
            let associated_value = TagSequence::decode(number);
            return Self::Ul(associated_value);
        }
        if num == BigUint::from(5usize) {
            let associated_value = TagSequence::decode(number);
            return Self::Ol(associated_value);
        }
        if num == BigUint::from(6usize) {
            let associated_value = RowContents::decode(number);
            return Self::Row(associated_value);
        }
        if num == BigUint::from(7usize) {
            let associated_value = LinkContents::decode(number);
            return Self::Link(associated_value);
        }
        if num == BigUint::from(8usize) {
            let associated_value = NavlinkContents::decode(number);
            return Self::Navlink(associated_value);
        }
        if num == BigUint::from(9usize) {
            let associated_value = BtnContents::decode(number);
            return Self::Btn(associated_value);
        }
        if num == BigUint::from(10usize) {
            let associated_value = NavbtnContents::decode(number);
            return Self::Navbtn(associated_value);
        }
        if num == BigUint::from(11usize) {
            let associated_value = TArg::decode(number);
            return Self::Img(associated_value);
        }
        if num == BigUint::from(12usize) {
            let associated_value = TagSequence::decode(number);
            return Self::Table(associated_value);
        }
        if num == BigUint::from(13usize) {
            let associated_value = TagSequence::decode(number);
            return Self::Trow(associated_value);
        }
        if num == BigUint::from(14usize) {
            let associated_value = TagSequence::decode(number);
            return Self::Tprow(associated_value);
        }
        if num == BigUint::from(15usize) {
            return Self::Hr();
        }
        if num == BigUint::from(16usize) {
            let associated_value = TBody::decode(number);
            return Self::B(associated_value);
        }
        if num == BigUint::from(17usize) {
            let associated_value = TBody::decode(number);
            return Self::I(associated_value);
        }
        if num == BigUint::from(18usize) {
            let associated_value = NNBody::decode(number);
            return Self::Bq(associated_value);
        }
        if num == BigUint::from(19usize) {
            let associated_value = NNArg::decode(number);
            return Self::Footlnk(associated_value);
        }
        if num == BigUint::from(20usize) {
            let associated_value = FootnContents::decode(number);
            return Self::Footn(associated_value);
        }
        if num == BigUint::from(21usize) {
            let associated_value = NNArg::decode(number);
            return Self::A(associated_value);
        }
        if num == BigUint::from(22usize) {
            let associated_value = TBody::decode(number);
            return Self::S(associated_value);
        }
        if num == BigUint::from(23usize) {
            let associated_value = TBody::decode(number);
            return Self::Sup(associated_value);
        }
        if num == BigUint::from(24usize) {
            let associated_value = TBody::decode(number);
            return Self::Sub(associated_value);
        }
        if num == BigUint::from(25usize) {
            let associated_value = NNBody::decode(number);
            return Self::Disc(associated_value);
        }
        if num == BigUint::from(26usize) {
            let associated_value = BlockContents::decode(number);
            return Self::Block(associated_value);
        }
        if num == BigUint::from(27usize) {
            let associated_value = TagSequence::decode(number);
            return Self::Carousel(associated_value);
        }
        if num == BigUint::from(28usize) {
            let associated_value = CodeContents::decode(number);
            return Self::Code(associated_value);
        }
        if num == BigUint::from(29usize) {
            let associated_value = TBody::decode(number);
            return Self::Pre(associated_value);
        }
        if num == BigUint::from(30usize) {
            let associated_value = MetaContents::decode(number);
            return Self::Meta(associated_value);
        }
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TBody {
    pub contents: std::string::String,
}

impl TBody {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(256usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        write(number, &Self::base(), &BigUint::from(0usize));
        for byte in self.contents.as_bytes().iter().rev() {
            write(number, &Self::base(), &BigUint::from(*byte as usize));
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let mut bytes = Vec::new();
        loop {
            let byte = read(number, &Self::base());
            if byte == BigUint::from(0usize) {
                break;
            }
            bytes.push(u8::try_from(&byte).unwrap());
        }
        Self {
            contents: std::string::String::from_utf8(bytes).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hl {
    One(),
    Two(),
    Three(),
    Four(),
    Five(),
    Six(),
}

impl Hl {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(6usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        match self {
            Self::One() => {
                write(number, &Self::base(), &BigUint::from(0usize));
            }
            Self::Two() => {
                write(number, &Self::base(), &BigUint::from(1usize));
            }
            Self::Three() => {
                write(number, &Self::base(), &BigUint::from(2usize));
            }
            Self::Four() => {
                write(number, &Self::base(), &BigUint::from(3usize));
            }
            Self::Five() => {
                write(number, &Self::base(), &BigUint::from(4usize));
            }
            Self::Six() => {
                write(number, &Self::base(), &BigUint::from(5usize));
            }
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == BigUint::from(0usize) {
            return Self::One();
        }
        if num == BigUint::from(1usize) {
            return Self::Two();
        }
        if num == BigUint::from(2usize) {
            return Self::Three();
        }
        if num == BigUint::from(3usize) {
            return Self::Four();
        }
        if num == BigUint::from(4usize) {
            return Self::Five();
        }
        if num == BigUint::from(5usize) {
            return Self::Six();
        }
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AlignArg {
    Start(),
    Center(),
    End(),
}

impl AlignArg {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(3usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        match self {
            Self::Start() => {
                write(number, &Self::base(), &BigUint::from(0usize));
            }
            Self::Center() => {
                write(number, &Self::base(), &BigUint::from(1usize));
            }
            Self::End() => {
                write(number, &Self::base(), &BigUint::from(2usize));
            }
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == BigUint::from(0usize) {
            return Self::Start();
        }
        if num == BigUint::from(1usize) {
            return Self::Center();
        }
        if num == BigUint::from(2usize) {
            return Self::End();
        }
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Body {
    Text(DaletString),
    Tags(TagSequence),
    Null(),
}

impl Body {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(3usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        match self {
            Self::Text(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(0usize));
            }
            Self::Tags(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(1usize));
            }
            Self::Null() => {
                write(number, &Self::base(), &BigUint::from(2usize));
            }
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == BigUint::from(0usize) {
            let associated_value = DaletString::decode(number);
            return Self::Text(associated_value);
        }
        if num == BigUint::from(1usize) {
            let associated_value = TagSequence::decode(number);
            return Self::Tags(associated_value);
        }
        if num == BigUint::from(2usize) {
            return Self::Null();
        }
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NNBody {
    Text(DaletString),
    Tags(TagSequence),
}

impl NNBody {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(2usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        match self {
            Self::Text(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(0usize));
            }
            Self::Tags(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(1usize));
            }
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == BigUint::from(0usize) {
            let associated_value = DaletString::decode(number);
            return Self::Text(associated_value);
        }
        if num == BigUint::from(1usize) {
            let associated_value = TagSequence::decode(number);
            return Self::Tags(associated_value);
        }
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TNullArg {
    Text(DaletString),
    Null(),
}

impl TNullArg {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(2usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        match self {
            Self::Text(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(0usize));
            }
            Self::Null() => {
                write(number, &Self::base(), &BigUint::from(1usize));
            }
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == BigUint::from(0usize) {
            let associated_value = DaletString::decode(number);
            return Self::Text(associated_value);
        }
        if num == BigUint::from(1usize) {
            return Self::Null();
        }
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TArg {
    pub contents: std::string::String,
}

impl TArg {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(256usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        write(number, &Self::base(), &BigUint::from(0usize));
        for byte in self.contents.as_bytes().iter().rev() {
            write(number, &Self::base(), &BigUint::from(*byte as usize));
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let mut bytes = Vec::new();
        loop {
            let byte = read(number, &Self::base());
            if byte == BigUint::from(0usize) {
                break;
            }
            bytes.push(u8::try_from(&byte).unwrap());
        }
        Self {
            contents: std::string::String::from_utf8(bytes).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum NNArg {
    Text(DaletString),
    Number(ArgNumber),
}

impl NNArg {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(2usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        match self {
            Self::Text(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(0usize));
            }
            Self::Number(associated_value) => {
                associated_value.encode(number);
                write(number, &Self::base(), &BigUint::from(1usize));
            }
        }
    }
    pub fn decode(number: &mut BigUint) -> Self {
        let num = read(number, &Self::base());
        if num == BigUint::from(0usize) {
            let associated_value = DaletString::decode(number);
            return Self::Text(associated_value);
        }
        if num == BigUint::from(1usize) {
            let associated_value = ArgNumber::decode(number);
            return Self::Number(associated_value);
        }
        unreachable!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArgNumber {
    pub value: BigUint,
}

impl ArgNumber {
    #[allow(dead_code)]
    pub fn base() -> BigUint {
        BigUint::from(256usize)
    }
    pub fn encode(&self, number: &mut BigUint) {
        write(number, &Self::base(), &self.value);
    }
    pub fn decode(number: &mut BigUint) -> Self {
        Self {
            value: read(number, &Self::base()),
        }
    }
}
