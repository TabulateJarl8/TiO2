use lazy_static::lazy_static;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    RHSFunction(&'static str),
    LHSFunction(&'static str),
    BothSidesFunction(&'static str),
    NoArgsFunction(&'static str),
    Token(&'static str),
    Conditional(&'static str),
}

impl PartialEq<&str> for TokenType {
    fn eq(&self, other: &&str) -> bool {
        match (self, other) {
            (Self::RHSFunction(l0), r0) => l0 == r0,
            (Self::LHSFunction(l0), r0) => l0 == r0,
            (Self::BothSidesFunction(l0), r0) => l0 == r0,
            (Self::NoArgsFunction(l0), r0) => l0 == r0,
            (Self::Token(l0), r0) => l0 == r0,
            (Self::Conditional(l0), r0) => l0 == r0,
        }
    }
}

impl AsRef<str> for TokenType {
    fn as_ref(&self) -> &str {
        match self {
            TokenType::RHSFunction(v) => v,
            TokenType::LHSFunction(v) => v,
            TokenType::BothSidesFunction(v) => v,
            TokenType::NoArgsFunction(v) => v,
            TokenType::Token(v) => v,
            TokenType::Conditional(v) => v,
        }
    }
}


/// A utility function that returns the inverse mapping of byte tokens.
/// Used when compiling instead of decompiling.
///
/// This function constructs a `HashMap` where the key is a `&'static str` and the value is a [`Byte`].
///
/// # Examples
///
/// ```
/// use tio2::translation::tokens::{get_inverse_tokens, BYTE_TOKENS, Byte};
///
/// let inverse_tokens = get_inverse_tokens();
/// let token = ">DMS";
/// if let Some(byte) = inverse_tokens.get(token) {
///     assert_eq!(&token, BYTE_TOKENS.get(byte).unwrap());
/// }
/// ```
pub fn get_inverse_tokens_as_str() -> HashMap<&'static str, Byte> {
    let mut flipped: HashMap<&'static str, Byte> = Default::default();

    // BYTE_TOKENS.clone().into_iter().for_each(|(key, value)| {
    //     flipped.insert(value.as_ref(), key);
    // });
    for (byte, token) in BYTE_TOKENS.iter() {
        flipped.insert(token.as_ref(), *byte);
    }

    flipped
}

/// The `Byte` enum represents bytes, which can be either single bytes (`Single`) or
/// two bytes (`Double`).
///
/// # Examples
///
/// ```
/// use tio2::translation::tokens::{BYTE_TOKENS, Byte};
///
/// // Access the token for a double byte (e.g., [0x60, 0x00])
/// if let Some(token) = BYTE_TOKENS.get(&Byte::Double([0x60, 0x00])) {
///     assert_eq!(token, &"Pic1");
/// }
/// ```
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Byte {
    Single(u8),
    Double([u8; 2]),
}

lazy_static! {
    /// Provides a `HashMap` of byte tokens where the key is a [`Byte`] and the value is a `&'static str`.
    ///
    /// This hash map contains mappings for byte tokens used in TI-8XP files.
    ///
    /// # Example
    ///
    /// ```
    /// use tio2::translation::tokens::{BYTE_TOKENS, Byte};
    ///
    /// if let Some(token) = BYTE_TOKENS.get(&Byte::Single(0x01)) {
    ///     assert_eq!(token, &">DMS");
    /// }
    /// ```
    pub static ref BYTE_TOKENS: HashMap<Byte, TokenType> = [
        (Byte::Single(0x01), TokenType::LHSFunction(">DMS")),
        (Byte::Single(0x02), TokenType::LHSFunction(">Dec")),
        (Byte::Single(0x03), TokenType::LHSFunction(">Frac")),
        (Byte::Single(0x04), TokenType::BothSidesFunction("->")),
        (Byte::Single(0x05), TokenType::NoArgsFunction("BoxPlot")),
        (Byte::Single(0x06), TokenType::Token("[")),
        (Byte::Single(0x07), TokenType::Token("]")),
        (Byte::Single(0x08), TokenType::Token("{")),
        (Byte::Single(0x09), TokenType::Token("}")),
        (Byte::Single(0x0B), TokenType::Token("°")),
        (Byte::Single(0x0C), TokenType::Token("^-1")),
        (Byte::Single(0x0D), TokenType::Token("^2")),
        (Byte::Single(0x0E), TokenType::Token("^T")),
        (Byte::Single(0x0F), TokenType::Token("^3")),
        (Byte::Single(0x10), TokenType::Token("(")),
        (Byte::Single(0x11), TokenType::Token(")")),
        (Byte::Single(0x12), TokenType::RHSFunction("round(")),
        (Byte::Single(0x13), TokenType::RHSFunction("pxl-Test(")),
        (Byte::Single(0x14), TokenType::RHSFunction("augment(")),
        (Byte::Single(0x15), TokenType::RHSFunction("rowSwap(")),
        (Byte::Single(0x16), TokenType::RHSFunction("row+(")),
        (Byte::Single(0x17), TokenType::RHSFunction("*row(")),
        (Byte::Single(0x18), TokenType::RHSFunction("*row+(")),
        (Byte::Single(0x19), TokenType::RHSFunction("max(")),
        (Byte::Single(0x1A), TokenType::RHSFunction("min(")),
        (Byte::Single(0x1B), TokenType::RHSFunction("R>Pr(")),
        (Byte::Single(0x1C), TokenType::RHSFunction("R>Pθ(")),
        (Byte::Single(0x1D), TokenType::RHSFunction("P>Rx(")),
        (Byte::Single(0x1E), TokenType::RHSFunction("P>Ry(")),
        (Byte::Single(0x1F), TokenType::RHSFunction("median(")),
        (Byte::Single(0x20), TokenType::RHSFunction("randM(")),
        (Byte::Single(0x21), TokenType::RHSFunction("mean(")),
        (Byte::Single(0x22), TokenType::RHSFunction("solve(")),
        (Byte::Single(0x23), TokenType::RHSFunction("seq(")),
        (Byte::Single(0x24), TokenType::RHSFunction("fnInt(")),
        (Byte::Single(0x25), TokenType::RHSFunction("nDeriv(")),
        (Byte::Single(0x27), TokenType::RHSFunction("fMin(")),
        (Byte::Single(0x28), TokenType::RHSFunction("fMax(")),
        (Byte::Single(0x29), TokenType::Token(" ")),
        (Byte::Single(0x2A), TokenType::Token("\"")),
        (Byte::Single(0x2B), TokenType::Token(",")),
        (Byte::Single(0x2C), TokenType::Token("imaginary")),
        (Byte::Single(0x2D), TokenType::Token("!")),
        (Byte::Single(0x2E), TokenType::RHSFunction("CubicReg ")),
        (Byte::Single(0x2F), TokenType::RHSFunction("QuartReg ")),
        (Byte::Single(0x3A), TokenType::Token(".")),
        // Alternate notation of E
        (Byte::Single(0x3B), TokenType::Token("10^")),
        (Byte::Single(0x3C), TokenType::BothSidesFunction(" or ")),
        (Byte::Single(0x3D), TokenType::BothSidesFunction(" xor ")),
        (Byte::Single(0x3E), TokenType::Token(":")),
        (Byte::Single(0x3F), TokenType::Token("\n")),
        (Byte::Single(0x40), TokenType::BothSidesFunction(" and ")),
        (Byte::Single(0x41), TokenType::Token("A")),
        (Byte::Single(0x42), TokenType::Token("B")),
        (Byte::Single(0x43), TokenType::Token("C")),
        (Byte::Single(0x44), TokenType::Token("D")),
        (Byte::Single(0x45), TokenType::Token("E")),
        (Byte::Single(0x46), TokenType::Token("F")),
        (Byte::Single(0x47), TokenType::Token("G")),
        (Byte::Single(0x48), TokenType::Token("H")),
        (Byte::Single(0x49), TokenType::Token("I")),
        (Byte::Single(0x4A), TokenType::Token("J")),
        (Byte::Single(0x4B), TokenType::Token("K")),
        (Byte::Single(0x4C), TokenType::Token("L")),
        (Byte::Single(0x4D), TokenType::Token("M")),
        (Byte::Single(0x4E), TokenType::Token("N")),
        (Byte::Single(0x4F), TokenType::Token("O")),
        (Byte::Single(0x50), TokenType::Token("P")),
        (Byte::Single(0x51), TokenType::Token("Q")),
        (Byte::Single(0x52), TokenType::Token("R")),
        (Byte::Single(0x53), TokenType::Token("S")),
        (Byte::Single(0x54), TokenType::Token("T")),
        (Byte::Single(0x55), TokenType::Token("U")),
        (Byte::Single(0x56), TokenType::Token("V")),
        (Byte::Single(0x57), TokenType::Token("W")),
        (Byte::Single(0x58), TokenType::Token("X")),
        (Byte::Single(0x59), TokenType::Token("Y")),
        (Byte::Single(0x5A), TokenType::Token("Z")),
        (Byte::Single(0x30), TokenType::Token("0")),
        (Byte::Single(0x31), TokenType::Token("1")),
        (Byte::Single(0x32), TokenType::Token("2")),
        (Byte::Single(0x33), TokenType::Token("3")),
        (Byte::Single(0x34), TokenType::Token("4")),
        (Byte::Single(0x35), TokenType::Token("5")),
        (Byte::Single(0x36), TokenType::Token("6")),
        (Byte::Single(0x37), TokenType::Token("7")),
        (Byte::Single(0x38), TokenType::Token("8")),
        (Byte::Single(0x39), TokenType::Token("9")),
        (Byte::Single(0x5B), TokenType::Token("θ")),
        (Byte::Single(0x5F), TokenType::RHSFunction("prgm")),
        (Byte::Single(0x64), TokenType::NoArgsFunction("Radian")),
        (Byte::Single(0x65), TokenType::NoArgsFunction("Degree")),
        (Byte::Single(0x66), TokenType::NoArgsFunction("Normal")),
        (Byte::Single(0x67), TokenType::NoArgsFunction("Sci")),
        (Byte::Single(0x68), TokenType::NoArgsFunction("Eng")),
        (Byte::Single(0x69), TokenType::NoArgsFunction("Float")),
        (Byte::Single(0x6A), TokenType::BothSidesFunction("=")),
        (Byte::Single(0x6B), TokenType::BothSidesFunction("<")),
        (Byte::Single(0x6C), TokenType::BothSidesFunction(">")),
        (Byte::Single(0x6D), TokenType::BothSidesFunction("<=")),
        (Byte::Single(0x6E), TokenType::BothSidesFunction(">=")),
        (Byte::Single(0x6F), TokenType::BothSidesFunction("!=")),
        (Byte::Single(0x70), TokenType::RHSFunction("+")),
        (Byte::Single(0x71), TokenType::RHSFunction("–")),
        (Byte::Single(0x72), TokenType::Token("Ans")),
        (Byte::Single(0x73), TokenType::RHSFunction("Fix ")),
        (Byte::Single(0x74), TokenType::NoArgsFunction("Horiz")),
        (Byte::Single(0x75), TokenType::NoArgsFunction("Full")),
        (Byte::Single(0x76), TokenType::NoArgsFunction("Func")),
        (Byte::Single(0x77), TokenType::NoArgsFunction("Param")),
        (Byte::Single(0x78), TokenType::NoArgsFunction("Polar")),
        (Byte::Single(0x79), TokenType::NoArgsFunction("Seq")),
        (Byte::Single(0x7A), TokenType::NoArgsFunction("IndpntAuto")),
        (Byte::Single(0x7B), TokenType::NoArgsFunction("IndpntAsk")),
        (Byte::Single(0x7C), TokenType::NoArgsFunction("DependAuto")),
        (Byte::Single(0x7D), TokenType::NoArgsFunction("DependAsk")),
        (Byte::Single(0x7F), TokenType::Token("▫︎")),
        (Byte::Single(0x80), TokenType::Token("˖")),
        (Byte::Single(0x81), TokenType::Token("·")),
        (Byte::Single(0x82), TokenType::Token("*")),
        (Byte::Single(0x83), TokenType::Token("/")),
        (Byte::Single(0x84), TokenType::NoArgsFunction("Trace")),
        (Byte::Single(0x85), TokenType::NoArgsFunction("ClrDraw")),
        (Byte::Single(0x86), TokenType::NoArgsFunction("ZStandard")),
        (Byte::Single(0x87), TokenType::NoArgsFunction("ZTrig")),
        (Byte::Single(0x88), TokenType::NoArgsFunction("ZBox")),
        (Byte::Single(0x89), TokenType::NoArgsFunction("Zoom_In")),
        (Byte::Single(0x8A), TokenType::NoArgsFunction("Zoom_Out")),
        (Byte::Single(0x8B), TokenType::NoArgsFunction("ZSquare")),
        (Byte::Single(0x8C), TokenType::NoArgsFunction("ZInteger")),
        (Byte::Single(0x8D), TokenType::NoArgsFunction("ZPrevious")),
        (Byte::Single(0x8E), TokenType::NoArgsFunction("ZDecimal")),
        (Byte::Single(0x8F), TokenType::NoArgsFunction("ZoomStat")),
        (Byte::Single(0x90), TokenType::NoArgsFunction("ZoomRcl")),
        (Byte::Single(0x91), TokenType::NoArgsFunction("PrintScreen")),
        (Byte::Single(0x92), TokenType::NoArgsFunction("ZoomSto")),
        (Byte::Single(0x93), TokenType::RHSFunction("Text(")),
        (Byte::Single(0x94), TokenType::BothSidesFunction(" nPr ")),
        (Byte::Single(0x95), TokenType::BothSidesFunction(" nCr ")),
        (Byte::Single(0x96), TokenType::RHSFunction("FnOn ")),
        (Byte::Single(0x97), TokenType::RHSFunction("FnOff ")),
        (Byte::Single(0x98), TokenType::RHSFunction("StorePic ")),
        (Byte::Single(0x99), TokenType::RHSFunction("RecallPic ")),
        (Byte::Single(0x9A), TokenType::RHSFunction("StoreGDB ")),
        (Byte::Single(0x9B), TokenType::RHSFunction("RecallGDB ")),
        (Byte::Single(0x9C), TokenType::RHSFunction("Line(")),
        (Byte::Single(0x9D), TokenType::RHSFunction("Vertical ")),
        (Byte::Single(0x9E), TokenType::RHSFunction("Pt-On(")),
        (Byte::Single(0x9F), TokenType::RHSFunction("Pt-Off(")),
        (Byte::Single(0xA0), TokenType::RHSFunction("Pt-Change( ")),
        (Byte::Single(0xA1), TokenType::RHSFunction("Pxl-On( ")),
        (Byte::Single(0xA2), TokenType::RHSFunction("Pxl-Off( ")),
        (Byte::Single(0xA3), TokenType::RHSFunction("Pxl-Change( ")),
        (Byte::Single(0xA4), TokenType::RHSFunction("Shade(")),
        (Byte::Single(0xA5), TokenType::RHSFunction("Circle(")),
        (Byte::Single(0xA6), TokenType::RHSFunction("Horizontal ")),
        (Byte::Single(0xA7), TokenType::RHSFunction("Tangent(")),
        (Byte::Single(0xA8), TokenType::RHSFunction("DrawInv ")),
        (Byte::Single(0xA9), TokenType::RHSFunction("DrawF ")),
        (Byte::Single(0xAB), TokenType::Token("rand")),
        (Byte::Single(0xAC), TokenType::Token("π")),
        (Byte::Single(0xAD), TokenType::NoArgsFunction("getKey")),
        (Byte::Single(0xAE), TokenType::Token("\'")),
        (Byte::Single(0xAF), TokenType::Token("?")),
        (Byte::Single(0xB0), TokenType::Token("-")),
        (Byte::Single(0xB1), TokenType::RHSFunction("int(")),
        (Byte::Single(0xB2), TokenType::RHSFunction("abs(")),
        (Byte::Single(0xB3), TokenType::RHSFunction("det(")),
        (Byte::Single(0xB4), TokenType::RHSFunction("identity(")),
        (Byte::Single(0xB5), TokenType::RHSFunction("dim(")),
        (Byte::Single(0xB6), TokenType::RHSFunction("sum(")),
        (Byte::Single(0xB7), TokenType::RHSFunction("prod(")),
        (Byte::Single(0xB8), TokenType::RHSFunction("not(")),
        (Byte::Single(0xB9), TokenType::RHSFunction("iPart(")),
        (Byte::Single(0xBA), TokenType::RHSFunction("fPart(")),
        (Byte::Single(0xBC), TokenType::RHSFunction("sqrt(")),
        (Byte::Single(0xBD), TokenType::RHSFunction("cubrt(")),
        (Byte::Single(0xBE), TokenType::RHSFunction("ln (")),
        (Byte::Single(0xBF), TokenType::RHSFunction("e^(")),
        (Byte::Single(0xC0), TokenType::RHSFunction("log(")),
        (Byte::Single(0xC1), TokenType::RHSFunction("10^(")),
        (Byte::Single(0xC2), TokenType::RHSFunction("sin(")),
        (Byte::Single(0xC3), TokenType::RHSFunction("sin^-1(")),
        (Byte::Single(0xC4), TokenType::RHSFunction("cos(")),
        (Byte::Single(0xC5), TokenType::RHSFunction("cos^-1(")),
        (Byte::Single(0xC6), TokenType::RHSFunction("tan(")),
        (Byte::Single(0xC7), TokenType::RHSFunction("tan^-1(")),
        (Byte::Single(0xC8), TokenType::RHSFunction("sinh(")),
        (Byte::Single(0xC9), TokenType::RHSFunction("sinh^-1(")),
        (Byte::Single(0xCA), TokenType::RHSFunction("cosh(")),
        (Byte::Single(0xCB), TokenType::RHSFunction("cosh^-1(")),
        (Byte::Single(0xCC), TokenType::RHSFunction("tanh(")),
        (Byte::Single(0xCD), TokenType::RHSFunction("tanh^-1(")),
        (Byte::Single(0xCE), TokenType::Conditional("If ")),
        (Byte::Single(0xCF), TokenType::Conditional("Then")),
        (Byte::Single(0xD0), TokenType::Conditional("Else")),
        (Byte::Single(0xD1), TokenType::Conditional("While ")),
        (Byte::Single(0xD2), TokenType::Conditional("Repeat ")),
        (Byte::Single(0xD3), TokenType::Conditional("For ")),
        (Byte::Single(0xD4), TokenType::Conditional("End")),
        (Byte::Single(0xD5), TokenType::Conditional("Return")),
        (Byte::Single(0xD6), TokenType::RHSFunction("Lbl ")),
        (Byte::Single(0xD7), TokenType::RHSFunction("Goto ")),
        (Byte::Single(0xD8), TokenType::RHSFunction("Pause ")),
        (Byte::Single(0xD9), TokenType::NoArgsFunction("Stop")),
        (Byte::Single(0xDA), TokenType::RHSFunction("IS>(")),
        (Byte::Single(0xDB), TokenType::RHSFunction("DS>(")),
        (Byte::Single(0xDC), TokenType::RHSFunction("Input ")),
        (Byte::Single(0xDD), TokenType::RHSFunction("Prompt ")),
        (Byte::Single(0xDE), TokenType::RHSFunction("Disp ")),
        (Byte::Single(0xDF), TokenType::NoArgsFunction("DispGraph")),
        (Byte::Single(0xE0), TokenType::RHSFunction("Output(")),
        (Byte::Single(0xE1), TokenType::NoArgsFunction("ClrHome")),
        (Byte::Single(0xE2), TokenType::RHSFunction("Fill(")),
        (Byte::Single(0xE3), TokenType::RHSFunction("SortA(")),
        (Byte::Single(0xE4), TokenType::RHSFunction("SortD(")),
        (Byte::Single(0xE5), TokenType::NoArgsFunction("DispTable")),
        (Byte::Single(0xE6), TokenType::RHSFunction("Menu(")),
        (Byte::Single(0xE7), TokenType::RHSFunction("Send(")),
        (Byte::Single(0xE8), TokenType::RHSFunction("Get(")),
        (Byte::Single(0xE9), TokenType::RHSFunction("PlotsOn ")),
        (Byte::Single(0xEA), TokenType::RHSFunction("PlotsOff ")),
        (Byte::Single(0xEC), TokenType::RHSFunction("Plot1(")),
        (Byte::Single(0xED), TokenType::RHSFunction("Plot2(")),
        (Byte::Single(0xEE), TokenType::RHSFunction("Plot3(")),
        (Byte::Single(0xF0), TokenType::Token("^")),
        (Byte::Single(0xF1), TokenType::Token("[xth root]")),
        (Byte::Single(0xF2), TokenType::RHSFunction("1-Var Stats ")),
        (Byte::Single(0xF3), TokenType::RHSFunction("2-Var Stats ")),
        (Byte::Single(0xF4), TokenType::RHSFunction("LinReg(a+bx) ")),
        (Byte::Single(0xF5), TokenType::RHSFunction("ExpReg ")),
        (Byte::Single(0xF6), TokenType::RHSFunction("LnReg ")),
        (Byte::Single(0xF7), TokenType::RHSFunction("PwrReg ")),
        (Byte::Single(0xF8), TokenType::RHSFunction("Med-Med ")),
        (Byte::Single(0xF9), TokenType::RHSFunction("QuadReg ")),
        (Byte::Single(0xFA), TokenType::RHSFunction("ClrList ")),
        (Byte::Single(0xFB), TokenType::NoArgsFunction("ClrTable")),
        (Byte::Single(0xFC), TokenType::NoArgsFunction("Histogram")),
        (Byte::Single(0xFD), TokenType::NoArgsFunction("xyLine")),
        (Byte::Single(0xFE), TokenType::NoArgsFunction("Scatter")),
        (Byte::Single(0xFF), TokenType::RHSFunction("LinReg(ax+b) ")),
        (Byte::Single(0xEB), TokenType::Token("l")),
        // System variables (incomplete: TODO)
        (Byte::Double([0x5C, 0x00]), TokenType::Token("[A]")),
        (Byte::Double([0x5C, 0x01]), TokenType::Token("[B]")),
        (Byte::Double([0x5C, 0x02]), TokenType::Token("[C]")),
        (Byte::Double([0x5C, 0x03]), TokenType::Token("[D]")),
        (Byte::Double([0x5C, 0x04]), TokenType::Token("[E]")),
        (Byte::Double([0x5C, 0x05]), TokenType::Token("[F]")),
        (Byte::Double([0x5C, 0x06]), TokenType::Token("[G]")),
        (Byte::Double([0x5C, 0x07]), TokenType::Token("[H]")),
        (Byte::Double([0x5C, 0x08]), TokenType::Token("[I]")),
        (Byte::Double([0x5C, 0x09]), TokenType::Token("[J]")),
        (Byte::Double([0x5D, 0x00]), TokenType::Token("l1")),
        (Byte::Double([0x5D, 0x01]), TokenType::Token("l2")),
        (Byte::Double([0x5D, 0x02]), TokenType::Token("l3")),
        (Byte::Double([0x5D, 0x03]), TokenType::Token("l4")),
        (Byte::Double([0x5D, 0x04]), TokenType::Token("l5")),
        (Byte::Double([0x5D, 0x05]), TokenType::Token("l6")),
        (Byte::Double([0x5D, 0x06]), TokenType::Token("l7")),
        (Byte::Double([0x5D, 0x07]), TokenType::Token("l8")),
        (Byte::Double([0x5D, 0x08]), TokenType::Token("l0")),
        (Byte::Double([0x5E, 0x10]), TokenType::Token("y1")),
        (Byte::Double([0x5E, 0x11]), TokenType::Token("y2")),
        (Byte::Double([0x5E, 0x12]), TokenType::Token("y3")),
        (Byte::Double([0x5E, 0x13]), TokenType::Token("y4")),
        (Byte::Double([0x5E, 0x14]), TokenType::Token("y5")),
        (Byte::Double([0x5E, 0x15]), TokenType::Token("y6")),
        (Byte::Double([0x5E, 0x16]), TokenType::Token("y7")),
        (Byte::Double([0x5E, 0x17]), TokenType::Token("y8")),
        (Byte::Double([0x5E, 0x18]), TokenType::Token("y9")),
        (Byte::Double([0x5E, 0x19]), TokenType::Token("y0")),
        (Byte::Double([0x5E, 0x40]), TokenType::Token("r_1")),
        (Byte::Double([0x5E, 0x41]), TokenType::Token("r_2")),
        (Byte::Double([0x5E, 0x42]), TokenType::Token("r_3")),
        (Byte::Double([0x5E, 0x43]), TokenType::Token("r_4")),
        (Byte::Double([0x5E, 0x44]), TokenType::Token("r_5")),
        (Byte::Double([0x5E, 0x45]), TokenType::Token("r_6")),
        (Byte::Double([0x5E, 0x20]), TokenType::Token("x1t")),
        (Byte::Double([0x5E, 0x21]), TokenType::Token("y1t")),
        (Byte::Double([0x5E, 0x22]), TokenType::Token("x2t")),
        (Byte::Double([0x5E, 0x23]), TokenType::Token("y2t")),
        (Byte::Double([0x5E, 0x24]), TokenType::Token("x3t")),
        (Byte::Double([0x5E, 0x25]), TokenType::Token("y3t")),
        (Byte::Double([0x5E, 0x26]), TokenType::Token("x4t")),
        (Byte::Double([0x5E, 0x27]), TokenType::Token("y4t")),
        (Byte::Double([0x5E, 0x28]), TokenType::Token("x5t")),
        (Byte::Double([0x5E, 0x29]), TokenType::Token("y5t")),
        (Byte::Double([0x5E, 0x2A]), TokenType::Token("x6t")),
        (Byte::Double([0x5E, 0x2B]), TokenType::Token("y6t")),
        (Byte::Double([0x60, 0x00]), TokenType::Token("Pic1")),
        (Byte::Double([0x60, 0x01]), TokenType::Token("Pic2")),
        (Byte::Double([0x60, 0x02]), TokenType::Token("Pic3")),
        (Byte::Double([0x60, 0x03]), TokenType::Token("Pic4")),
        (Byte::Double([0x60, 0x04]), TokenType::Token("Pic5")),
        (Byte::Double([0x60, 0x05]), TokenType::Token("Pic6")),
        (Byte::Double([0x60, 0x06]), TokenType::Token("Pic7")),
        (Byte::Double([0x60, 0x07]), TokenType::Token("Pic8")),
        (Byte::Double([0x60, 0x08]), TokenType::Token("Pic9")),
        (Byte::Double([0x60, 0x09]), TokenType::Token("Pic0")),
        (Byte::Double([0x61, 0x00]), TokenType::Token("GDB1")),
        (Byte::Double([0x61, 0x01]), TokenType::Token("GDB2")),
        (Byte::Double([0x61, 0x02]), TokenType::Token("GDB3")),
        (Byte::Double([0x61, 0x03]), TokenType::Token("GDB4")),
        (Byte::Double([0x61, 0x04]), TokenType::Token("GDB5")),
        (Byte::Double([0x61, 0x05]), TokenType::Token("GDB6")),
        (Byte::Double([0x61, 0x06]), TokenType::Token("GDB7")),
        (Byte::Double([0x61, 0x07]), TokenType::Token("GDB8")),
        (Byte::Double([0x61, 0x08]), TokenType::Token("GDB9")),
        (Byte::Double([0x61, 0x09]), TokenType::Token("GDB0")),
        (Byte::Double([0x62, 0x01]), TokenType::NoArgsFunction("RegEq")),
        (Byte::Double([0x62, 0x03]), TokenType::Token("[x-bar]")),
        (Byte::Double([0x62, 0x04]), TokenType::Token("[Summ x]")),
        (Byte::Double([0x62, 0x05]), TokenType::Token("[Summ x^2]")),
        (Byte::Double([0x62, 0x06]), TokenType::Token("Sx")),
        (Byte::Double([0x62, 0x07]), TokenType::Token("[sigma]x")),
        (Byte::Double([0x62, 0x08]), TokenType::Token("minX")),
        (Byte::Double([0x62, 0x09]), TokenType::Token("maxX")),
        (Byte::Double([0x62, 0x0A]), TokenType::Token("minY")),
        (Byte::Double([0x62, 0x0B]), TokenType::Token("maxY")),
        (Byte::Double([0x62, 0x0C]), TokenType::Token("[y-bar]")),
        (Byte::Double([0x62, 0x0D]), TokenType::Token("[Summ y]")),
        (Byte::Double([0x62, 0x0E]), TokenType::Token("[Summ y^2]")),
        (Byte::Double([0x62, 0x0F]), TokenType::Token("Sy")),
        (Byte::Double([0x62, 0x10]), TokenType::Token("[sigma]y")),
        (Byte::Double([0x62, 0x11]), TokenType::Token("[Summ xy]")),
        (Byte::Double([0x62, 0x13]), TokenType::Token("Med")),
        (Byte::Double([0x62, 0x14]), TokenType::Token("Q1")),
        (Byte::Double([0x62, 0x15]), TokenType::Token("Q3")),
        (Byte::Double([0x62, 0x1B]), TokenType::Token("x1")),
        (Byte::Double([0x62, 0x1C]), TokenType::Token("x2")),
        (Byte::Double([0x62, 0x1D]), TokenType::Token("x3")),
        (Byte::Double([0x62, 0x1E]), TokenType::Token("y1")),
        (Byte::Double([0x62, 0x1F]), TokenType::Token("y2")),
        (Byte::Double([0x62, 0x20]), TokenType::Token("y3")),
        (Byte::Double([0x62, 0x25]), TokenType::Token("[chi]2")),
        (Byte::Double([0x62, 0x26]), TokenType::Token("[fin]")),
        (Byte::Double([0x62, 0x27]), TokenType::Token("df")),
        (Byte::Double([0x62, 0x28]), TokenType::Token("[p-hat]")),
        (Byte::Double([0x62, 0x29]), TokenType::Token("[p-hat]1")),
        (Byte::Double([0x62, 0x2A]), TokenType::Token("[p-hat]2")),
        (Byte::Double([0x62, 0x2B]), TokenType::Token("[x-bar]1")),
        (Byte::Double([0x62, 0x2C]), TokenType::Token("Sx1")),
        (Byte::Double([0x62, 0x2D]), TokenType::Token("n_1")),
        (Byte::Double([0x62, 0x2E]), TokenType::Token("[x-bar]2")),
        (Byte::Double([0x62, 0x2F]), TokenType::Token("Sx2")),
        (Byte::Double([0x62, 0x30]), TokenType::Token("n_2")),
        (Byte::Double([0x62, 0x31]), TokenType::Token("Sxp")),
        (Byte::Double([0x62, 0x32]), TokenType::Token("lower")),
        (Byte::Double([0x62, 0x33]), TokenType::Token("upper")),
        (Byte::Double([0x62, 0x35]), TokenType::Token("r2")),
        (Byte::Double([0x62, 0x36]), TokenType::Token("R2")),
        (Byte::Double([0x62, 0x38]), TokenType::Token("SS")),
        (Byte::Double([0x62, 0x39]), TokenType::Token("MS")),
        (Byte::Double([0x63, 0x00]), TokenType::Token("ZXscl")),
        (Byte::Double([0x63, 0x01]), TokenType::Token("ZYscl")),
        (Byte::Double([0x63, 0x02]), TokenType::Token("Xscl")),
        (Byte::Double([0x63, 0x03]), TokenType::Token("Yscl")),
        (Byte::Double([0x63, 0x04]), TokenType::Token("U_nStart")),
        (Byte::Double([0x63, 0x05]), TokenType::Token("V_nStart")),
        (Byte::Double([0x63, 0x06]), TokenType::Token("U_(n-1)")),
        (Byte::Double([0x63, 0x07]), TokenType::Token("V_(n-1)")),
        // System variables (TODO)
        (Byte::Double([0xAA, 0x00]), TokenType::Token("Str1")),
        (Byte::Double([0xAA, 0x01]), TokenType::Token("Str2")),
        (Byte::Double([0xAA, 0x02]), TokenType::Token("Str3")),
        (Byte::Double([0xAA, 0x03]), TokenType::Token("Str4")),
        (Byte::Double([0xAA, 0x04]), TokenType::Token("Str5")),
        (Byte::Double([0xAA, 0x05]), TokenType::Token("Str6")),
        (Byte::Double([0xAA, 0x06]), TokenType::Token("Str7")),
        (Byte::Double([0xAA, 0x07]), TokenType::Token("Str8")),
        (Byte::Double([0xAA, 0x08]), TokenType::Token("Str9")),
        (Byte::Double([0xAA, 0x09]), TokenType::Token("Str0")),
        // BB tokens (two-byte), Incomplete TODO
        (Byte::Double([0xBB, 0x00]), TokenType::RHSFunction("npv(")),
        (Byte::Double([0xBB, 0x01]), TokenType::RHSFunction("irr(")),
        (Byte::Double([0xBB, 0x02]), TokenType::RHSFunction("bal(")),
        (Byte::Double([0xBB, 0x03]), TokenType::RHSFunction("SummPrn(")),
        (Byte::Double([0xBB, 0x04]), TokenType::RHSFunction("SummInt(")),
        (Byte::Double([0xBB, 0x05]), TokenType::RHSFunction(">Nom(")),
        (Byte::Double([0xBB, 0x06]), TokenType::RHSFunction(">Eff(")),
        (Byte::Double([0xBB, 0x07]), TokenType::RHSFunction("dbd(")),
        (Byte::Double([0xBB, 0x08]), TokenType::RHSFunction("Icm(")),
        (Byte::Double([0xBB, 0x09]), TokenType::RHSFunction("gcd(")),
        (Byte::Double([0xBB, 0x0A]), TokenType::RHSFunction("randInt(")),
        (Byte::Double([0xBB, 0x0B]), TokenType::RHSFunction("randBin(")),
        (Byte::Double([0xBB, 0x0C]), TokenType::RHSFunction("sub(")),
        (Byte::Double([0xBB, 0x0D]), TokenType::RHSFunction("stdDev(")),
        (Byte::Double([0xBB, 0x0E]), TokenType::RHSFunction("variance(")),
        (Byte::Double([0xBB, 0x0F]), TokenType::RHSFunction("inString(")),
        (Byte::Double([0xBB, 0xB0]), TokenType::Token("a")),
        (Byte::Double([0xBB, 0xB1]), TokenType::Token("b")),
        (Byte::Double([0xBB, 0xB2]), TokenType::Token("c")),
        (Byte::Double([0xBB, 0xB3]), TokenType::Token("d")),
        (Byte::Double([0xBB, 0xB4]), TokenType::Token("e")),
        (Byte::Double([0xBB, 0xB5]), TokenType::Token("f")),
        (Byte::Double([0xBB, 0xB6]), TokenType::Token("g")),
        (Byte::Double([0xBB, 0xB7]), TokenType::Token("h")),
        (Byte::Double([0xBB, 0xB8]), TokenType::Token("i")),
        (Byte::Double([0xBB, 0xB9]), TokenType::Token("j")),
        (Byte::Double([0xBB, 0xBA]), TokenType::Token("k")),
        (Byte::Double([0xBB, 0xBC]), TokenType::Token("l")),
        (Byte::Double([0xBB, 0xBD]), TokenType::Token("m")),
        (Byte::Double([0xBB, 0xBE]), TokenType::Token("n")),
        (Byte::Double([0xBB, 0xBF]), TokenType::Token("o")),
        (Byte::Double([0xBB, 0xC0]), TokenType::Token("p")),
        (Byte::Double([0xBB, 0xC1]), TokenType::Token("q")),
        (Byte::Double([0xBB, 0xC2]), TokenType::Token("r")),
        (Byte::Double([0xBB, 0xC3]), TokenType::Token("s")),
        (Byte::Double([0xBB, 0xC4]), TokenType::Token("t")),
        (Byte::Double([0xBB, 0xC5]), TokenType::Token("u")),
        (Byte::Double([0xBB, 0xC6]), TokenType::Token("v")),
        (Byte::Double([0xBB, 0xC7]), TokenType::Token("w")),
        (Byte::Double([0xBB, 0xC8]), TokenType::Token("x")),
        (Byte::Double([0xBB, 0xC9]), TokenType::Token("y")),
        (Byte::Double([0xBB, 0xCA]), TokenType::Token("z")),
        (Byte::Double([0xBB, 0x52]), TokenType::NoArgsFunction("ClrAllLists")),
        (Byte::Double([0xBB, 0x4F]), TokenType::Token("a+bi")),
        (Byte::Double([0xBB, 0x4D]), TokenType::NoArgsFunction("Real")),
        (Byte::Double([0xBB, 0x67]), TokenType::NoArgsFunction("DiagnosticOff")),
        (Byte::Double([0xBB, 0x51]), TokenType::NoArgsFunction("ExprOff")),
        (Byte::Double([0xBB, 0x66]), TokenType::NoArgsFunction("DiagnosticOn")),
        (Byte::Double([0xBB, 0x50]), TokenType::NoArgsFunction("ExprOn")),
        (Byte::Double([0xBB, 0x4A]), TokenType::RHSFunction("SetUpEditor ")),
        (Byte::Double([0xBB, 0x68]), TokenType::RHSFunction("Archive ")),
        (Byte::Double([0xBB, 0x54]), TokenType::RHSFunction("DelVar ")),
        // end of xBB
        (Byte::Double([0xEF, 0x67]), TokenType::RHSFunction("TextColor(")),
        (Byte::Double([0xEF, 0x5B]), TokenType::RHSFunction("BackgroundOn ")),
        (Byte::Double([0xEF, 0x41]), TokenType::Token("BLUE")),
        (Byte::Double([0xEF, 0x42]), TokenType::Token("RED")),
        (Byte::Double([0xEF, 0x43]), TokenType::Token("BLACK")),
        (Byte::Double([0xEF, 0x44]), TokenType::Token("MAGENTA")),
        (Byte::Double([0xEF, 0x45]), TokenType::Token("GREEN")),
        (Byte::Double([0xEF, 0x46]), TokenType::Token("ORANGE")),
        (Byte::Double([0xEF, 0x47]), TokenType::Token("BROWN")),
        (Byte::Double([0xEF, 0x48]), TokenType::Token("NAVY")),
        (Byte::Double([0xEF, 0x49]), TokenType::Token("LTBLUE")),
        (Byte::Double([0xEF, 0x4A]), TokenType::Token("YELLOW")),
        (Byte::Double([0xEF, 0x4B]), TokenType::Token("WHITE")),
        (Byte::Double([0xEF, 0x4C]), TokenType::Token("LTGRAY")),
        (Byte::Double([0xEF, 0x4D]), TokenType::Token("MEDGRAY")),
        (Byte::Double([0xEF, 0x4E]), TokenType::Token("GRAY")),
        (Byte::Double([0xEF, 0x4F]), TokenType::Token("DRAKGRAY")),
        (Byte::Double([0xEF, 0x09]), TokenType::NoArgsFunction("getDate")),
        (Byte::Double([0xEF, 0x0A]), TokenType::NoArgsFunction("getTime")),
        (Byte::Double([0xEF, 0x06]), TokenType::RHSFunction("dayOfWk(")),
        (Byte::Double([0xEF, 0x64]), TokenType::NoArgsFunction("BackgroundOff")),
        (Byte::Double([0xEF, 0x32]), TokenType::RHSFunction("remainder(")),
        (Byte::Double([0xEF, 0x6C]), TokenType::RHSFunction("BorderColor ")),
        (Byte::Double([0xEF, 0x37]), TokenType::NoArgsFunction("MATHPRINT")),
        (Byte::Double([0xEF, 0x3B]), TokenType::NoArgsFunction("AUTO")),
        (Byte::Double([0xEF, 0x39]), TokenType::NoArgsFunction("[n/d]")),
        (Byte::Double([0xEF, 0x10]), TokenType::NoArgsFunction("ClockOn")),
        (Byte::Double([0xEF, 0x0F]), TokenType::NoArgsFunction("ClockOff")),
        (Byte::Double([0xEF, 0x96]), TokenType::RHSFunction("Wait ")),
        (Byte::Double([0xEF, 0x38]), TokenType::NoArgsFunction("CLASSIC")),
        (Byte::Double([0xEF, 0x75]), TokenType::NoArgsFunction("Dot-Thin")),
        (Byte::Double([0xEF, 0x3A]), TokenType::NoArgsFunction("[Un/d]")),
        (Byte::Double([0xEF, 0x40]), TokenType::NoArgsFunction("STARTWIZARD OFF")),
        (Byte::Double([0xEF, 0x6B]), TokenType::NoArgsFunction("DetectAsymOff")),
        (Byte::Double([0xEF, 0x6A]), TokenType::NoArgsFunction("DetectAsymOn")),
        (Byte::Double([0xEF, 0x3F]), TokenType::NoArgsFunction("STARTWIZARD ON")),
        // 73 ** (Graph Options)
        (Byte::Double([0x7E, 0x06]), TokenType::NoArgsFunction("Thick")),
        (Byte::Double([0x7E, 0x00]), TokenType::NoArgsFunction("Sequential")),
        (Byte::Double([0x7E, 0x01]), TokenType::NoArgsFunction("Simul")),
        (Byte::Double([0x7E, 0x02]), TokenType::NoArgsFunction("PolarGC")),
        (Byte::Double([0x7E, 0x03]), TokenType::NoArgsFunction("RectGC")),
        (Byte::Double([0x7E, 0x04]), TokenType::NoArgsFunction("CoordOn")),
        (Byte::Double([0x7E, 0x05]), TokenType::NoArgsFunction("CoordOff")),
        (Byte::Double([0x7E, 0x06]), TokenType::NoArgsFunction("Connected")),
        (Byte::Double([0x7E, 0x07]), TokenType::NoArgsFunction("Dot")),
        (Byte::Double([0x7E, 0x08]), TokenType::NoArgsFunction("AxesOn")),
        (Byte::Double([0x7E, 0x09]), TokenType::NoArgsFunction("AxesOff")),
        (Byte::Double([0x7E, 0x0A]), TokenType::NoArgsFunction("GridOn")),
        (Byte::Double([0x7E, 0x0B]), TokenType::NoArgsFunction("GridOff")),
        (Byte::Double([0x7E, 0x0C]), TokenType::NoArgsFunction("LabelOn")),
        (Byte::Double([0x7E, 0x0D]), TokenType::NoArgsFunction("LabelOff")),
        (Byte::Double([0x7E, 0x0E]), TokenType::NoArgsFunction("Web")),
        (Byte::Double([0x7E, 0x0F]), TokenType::NoArgsFunction("Time")),
        (Byte::Double([0x7E, 0x10]), TokenType::NoArgsFunction("uvAxes")),
        (Byte::Double([0x7E, 0x11]), TokenType::NoArgsFunction("vwAxes")),
        (Byte::Double([0x7E, 0x12]), TokenType::NoArgsFunction("uwAxes")),
    ].iter().copied().collect();
}
