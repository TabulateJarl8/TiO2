use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    /// Provides a `HashMap` of single-byte tokens where the key is a `u8` and the value is a `&'static str`.
    ///
    /// This hash map contains mappings for single-byte tokens used in some TI-8XP files.
    ///
    /// # Example
    ///
    /// ```
    /// use tio2::translation::tokens::SINGLE_BYTE_TOKENS;
    ///
    /// if let Some(token) = SINGLE_BYTE_TOKENS.get(&0x01) {
    ///     println!("Token for 0x01: {}", token);
    /// }
    /// ```
    pub static ref SINGLE_BYTE_TOKENS: HashMap<u8, &'static str> = [
        (0x01, ">DMS"),
        (0x02, ">Dec"),
        (0x03, ">Frac"),
        (0x04, "→"),
        (0x05, "BoxPlot"),
        (0x06, "["),
        (0x07, "]"),
        (0x08, "{"),
        (0x09, "}"),
        (0x0B, "°"),
        (0x0C, "^-1"),
        (0x0D, "^2"),
        (0x0E, "^T"),
        (0x0F, "^3"),
        (0x10, "("),
        (0x11, ")"),
        (0x12, "round("),
        (0x13, "pxl-Test("),
        (0x14, "augment("),
        (0x15, "rowSwap("),
        (0x16, "row+("),
        (0x17, "*row("),
        (0x18, "*row+("),
        (0x19, "max("),
        (0x1A, "min("),
        (0x1B, "R>Pr("),
        (0x1C, "R>Pθ("),
        (0x1D, "P>Rx("),
        (0x1E, "P>Ry("),
        (0x1F, "median("),
        (0x20, "randM("),
        (0x21, "mean("),
        (0x22, "solve("),
        (0x23, "seq("),
        (0x24, "fnInt("),
        (0x25, "nDeriv("),
        (0x27, "fMin("),
        (0x28, "fMax("),
        (0x29, " "),
        (0x2A, "\""),
        (0x2B, ","),
        (0x2C, "imaginary"),
        (0x2D, "!"),
        (0x2E, "CubicReg "),
        (0x2F, "QuartReg "),
        (0x3A, "."),
        // Alternate notation of E
        (0x3B, "10^"),
        (0x3C, " or "),
        (0x3D, " xor "),
        (0x3E, ":"),
        (0x3F, "\n"),
        (0x40, " and "),
        (0x41, "A"),
        (0x42, "B"),
        (0x43, "C"),
        (0x44, "D"),
        (0x45, "E"),
        (0x46, "F"),
        (0x47, "G"),
        (0x48, "H"),
        (0x49, "I"),
        (0x4A, "J"),
        (0x4B, "K"),
        (0x4C, "L"),
        (0x4D, "M"),
        (0x4E, "N"),
        (0x4F, "O"),
        (0x50, "P"),
        (0x51, "Q"),
        (0x52, "R"),
        (0x53, "S"),
        (0x54, "T"),
        (0x55, "U"),
        (0x56, "V"),
        (0x57, "W"),
        (0x58, "X"),
        (0x59, "Y"),
        (0x5A, "Z"),
        (0x30, "0"),
        (0x31, "1"),
        (0x32, "2"),
        (0x33, "3"),
        (0x34, "4"),
        (0x35, "5"),
        (0x36, "6"),
        (0x37, "7"),
        (0x38, "8"),
        (0x39, "9"),
        (0x5B, "θ"),
        (0x5F, "prgm"),
        (0x64, "Radian"),
        (0x65, "Degree"),
        (0x66, "Normal"),
        (0x67, "Sci"),
        (0x68, "Eng"),
        (0x69, "Float"),
        (0x6A, "="),
        (0x6B, "<"),
        (0x6C, ">"),
        (0x6D, "<="),
        (0x6E, ">="),
        (0x6F, "!="),
        (0x70, "+"),
        (0x71, "-"),
        (0x72, "Ans"),
        (0x73, "Fix "),
        (0x74, "Horiz"),
        (0x75, "Full"),
        (0x76, "Func"),
        (0x77, "Param"),
        (0x78, "Polar"),
        (0x79, "Seq"),
        (0x7A, "IndpntAuto"),
        (0x7B, "IndpntAsk"),
        (0x7C, "DependAuto"),
        (0x7D, "DependAsk"),
        (0x7F, "[box]"),
        (0x80, "[cross]"),
        (0x81, "[dot]"),
        (0x82, "*"),
        (0x83, "/"),
        (0x84, "Trace"),
        (0x85, "ClrDraw"),
        (0x86, "ZStandard"),
        (0x87, "ZTrig"),
        (0x88, "ZBox"),
        (0x89, "Zoom_In"),
        (0x8A, "Zoom_Out"),
        (0x8B, "ZSquare"),
        (0x8C, "ZInteger"),
        (0x8D, "ZPrevious"),
        (0x8E, "ZDecimal"),
        (0x8F, "ZoomStat"),
        (0x90, "ZoomRcl"),
        (0x91, "PrintScreen"),
        (0x92, "ZoomSto"),
        (0x93, "Text("),
        (0x94, " nPr "),
        (0x95, " nCr "),
        (0x96, "FnOn "),
        (0x97, "FnOff "),
        (0x98, "StorePic "),
        (0x99, "RecallPic "),
        (0x9A, "StoreGDB "),
        (0x9B, "RecallGDB "),
        (0x9C, "Line("),
        (0x9D, "Vertical "),
        (0x9E, "Pt-On("),
        (0x9F, "Pt-Off("),
        (0xA0, "Pt-Change( "),
        (0xA1, "Pxl-On( "),
        (0xA2, "Pxl-Off( "),
        (0xA3, "Pxl-Change( "),
        (0xA4, "Shade("),
        (0xA5, "Circle("),
        (0xA6, "Horizontal "),
        (0xA7, "Tangent("),
        (0xA8, "DrawInv "),
        (0xA9, "DrawF "),
        (0xAB, "rand"),
        (0xAC, "π"),
        (0xAD, "getKey"),
        (0xAE, "\""),
        (0xAF, "?"),
        (0xB0, "-"),
        (0xB1, "int("),
        (0xB2, "abs("),
        (0xB3, "det("),
        (0xB4, "identity("),
        (0xB5, "dim("),
        (0xB6, "sum("),
        (0xB7, "prod("),
        (0xB8, "not("),
        (0xB9, "iPart("),
        (0xBA, "fPart("),
        (0xBC, "sqrt("),
        (0xBD, "cubrt("),
        (0xBE, "ln ("),
        (0xBF, "e^("),
        (0xC0, "log("),
        (0xC1, "10^("),
        (0xC2, "sin("),
        (0xC3, "sin^-1("),
        (0xC4, "cos("),
        (0xC5, "cos^-1("),
        (0xC6, "tan("),
        (0xC7, "tan^-1("),
        (0xC8, "sinh("),
        (0xC9, "sinh^-1("),
        (0xCA, "cosh("),
        (0xCB, "cosh^-1("),
        (0xCC, "tanh("),
        (0xCD, "tanh^-1("),
        (0xCE, "If "),
        (0xCF, "Then"),
        (0xD0, "Else"),
        (0xD1, "While "),
        (0xD2, "Repeat "),
        (0xD3, "For "),
        (0xD4, "End"),
        (0xD5, "Return"),
        (0xD6, "Lbl "),
        (0xD7, "Goto "),
        (0xD8, "Pause "),
        (0xD9, "Stop"),
        (0xDA, "IS>("),
        (0xDB, "DS>("),
        (0xDC, "Input "),
        (0xDD, "Prompt "),
        (0xDE, "Disp "),
        (0xDF, "DispGraph"),
        (0xE0, "Output("),
        (0xE1, "ClrHome"),
        (0xE2, "Fill("),
        (0xE3, "SortA("),
        (0xE4, "SortD("),
        (0xE5, "DispTable"),
        (0xE6, "Menu("),
        (0xE7, "Send("),
        (0xE8, "Get("),
        (0xE9, "PlotsOn "),
        (0xEA, "PlotsOff "),
        (0xEC, "Plot1("),
        (0xED, "Plot2("),
        (0xEE, "Plot3("),
        (0xF0, "^"),
        (0xF1, "[xth root]"),
        (0xF2, "1-Var Stats "),
        (0xF3, "2-Var Stats "),
        (0xF4, "LinReg(a+bx) "),
        (0xF5, "ExpReg "),
        (0xF6, "LnReg "),
        (0xF7, "PwrReg "),
        (0xF8, "Med-Med "),
        (0xF9, "QuadReg "),
        (0xFA, "ClrList "),
        (0xFB, "ClrTable"),
        (0xFC, "Histogram"),
        (0xFD, "xyLine"),
        (0xFE, "Scatter"),
        (0xFF, "LinReg(ax+b) "),
        (0xEB, "l"),
    ].iter().copied().collect();
}

lazy_static! {
    /// Provides a `HashMap` of double-byte tokens where the key is a `&'static [u8; 2]` and the value is a `&'static str`.
    ///
    /// This hash map contains mappings for double-byte tokens used in some TI-8XP files.
    ///
    /// # Example
    ///
    /// ```
    /// use tio2::translation::tokens::DOUBLE_BYTE_TOKENS;
    ///
    /// if let Some(token) = DOUBLE_BYTE_TOKENS.get(&[0x5C, 0x00]) {
    ///     println!("Token for 0x5C, 0x00: {}", token);
    /// }
    /// ```
    pub static ref DOUBLE_BYTE_TOKENS: HashMap<&'static [u8; 2], &'static str> = [
        // System variables (incomplete: TODO)
        (&[0x5C, 0x00], "[A]"),
        (&[0x5C, 0x01], "[B]"),
        (&[0x5C, 0x02], "[C]"),
        (&[0x5C, 0x03], "[D]"),
        (&[0x5C, 0x04], "[E]"),
        (&[0x5C, 0x05], "[F]"),
        (&[0x5C, 0x06], "[G]"),
        (&[0x5C, 0x07], "[H]"),
        (&[0x5C, 0x08], "[I]"),
        (&[0x5C, 0x09], "[J]"),
        (&[0x5D, 0x00], "l1"),
        (&[0x5D, 0x01], "l2"),
        (&[0x5D, 0x02], "l3"),
        (&[0x5D, 0x03], "l4"),
        (&[0x5D, 0x04], "l5"),
        (&[0x5D, 0x05], "l6"),
        (&[0x5D, 0x06], "l7"),
        (&[0x5D, 0x07], "l8"),
        (&[0x5D, 0x08], "l0"),
        (&[0x5E, 0x10], "y1"),
        (&[0x5E, 0x11], "y2"),
        (&[0x5E, 0x12], "y3"),
        (&[0x5E, 0x13], "y4"),
        (&[0x5E, 0x14], "y5"),
        (&[0x5E, 0x15], "y6"),
        (&[0x5E, 0x16], "y7"),
        (&[0x5E, 0x17], "y8"),
        (&[0x5E, 0x18], "y9"),
        (&[0x5E, 0x19], "y0"),
        (&[0x5E, 0x40], "r_1"),
        (&[0x5E, 0x41], "r_2"),
        (&[0x5E, 0x42], "r_3"),
        (&[0x5E, 0x43], "r_4"),
        (&[0x5E, 0x44], "r_5"),
        (&[0x5E, 0x45], "r_6"),
        (&[0x5E, 0x20], "x1t"),
        (&[0x5E, 0x21], "y1t"),
        (&[0x5E, 0x22], "x2t"),
        (&[0x5E, 0x23], "y2t"),
        (&[0x5E, 0x24], "x3t"),
        (&[0x5E, 0x25], "y3t"),
        (&[0x5E, 0x26], "x4t"),
        (&[0x5E, 0x27], "y4t"),
        (&[0x5E, 0x28], "x5t"),
        (&[0x5E, 0x29], "y5t"),
        (&[0x5E, 0x2A], "x6t"),
        (&[0x5E, 0x2B], "y6t"),
        (&[0x60, 0x00], "Pic1"),
        (&[0x60, 0x01], "Pic2"),
        (&[0x60, 0x02], "Pic3"),
        (&[0x60, 0x03], "Pic4"),
        (&[0x60, 0x04], "Pic5"),
        (&[0x60, 0x05], "Pic6"),
        (&[0x60, 0x06], "Pic7"),
        (&[0x60, 0x07], "Pic8"),
        (&[0x60, 0x08], "Pic9"),
        (&[0x60, 0x09], "Pic0"),
        (&[0x61, 0x00], "GDB1"),
        (&[0x61, 0x01], "GDB2"),
        (&[0x61, 0x02], "GDB3"),
        (&[0x61, 0x03], "GDB4"),
        (&[0x61, 0x04], "GDB5"),
        (&[0x61, 0x05], "GDB6"),
        (&[0x61, 0x06], "GDB7"),
        (&[0x61, 0x07], "GDB8"),
        (&[0x61, 0x08], "GDB9"),
        (&[0x61, 0x09], "GDB0"),
        (&[0x62, 0x01], "RegEq"),
        (&[0x62, 0x03], "[x-bar]"),
        (&[0x62, 0x04], "[Summ x]"),
        (&[0x62, 0x05], "[Summ x^2]"),
        (&[0x62, 0x06], "Sx"),
        (&[0x62, 0x07], "[sigma]x"),
        (&[0x62, 0x08], "minX"),
        (&[0x62, 0x09], "maxX"),
        (&[0x62, 0x0A], "minY"),
        (&[0x62, 0x0B], "maxY"),
        (&[0x62, 0x0C], "[y-bar]"),
        (&[0x62, 0x0D], "[Summ y]"),
        (&[0x62, 0x0E], "[Summ y^2]"),
        (&[0x62, 0x0F], "Sy"),
        (&[0x62, 0x10], "[sigma]y"),
        (&[0x62, 0x11], "[Summ xy]"),
        (&[0x62, 0x13], "Med"),
        (&[0x62, 0x14], "Q1"),
        (&[0x62, 0x15], "Q3"),
        (&[0x62, 0x1B], "x1"),
        (&[0x62, 0x1C], "x2"),
        (&[0x62, 0x1D], "x3"),
        (&[0x62, 0x1E], "y1"),
        (&[0x62, 0x1F], "y2"),
        (&[0x62, 0x20], "y3"),
        (&[0x62, 0x25], "[chi]2"),
        (&[0x62, 0x26], "[fin]"),
        (&[0x62, 0x27], "df"),
        (&[0x62, 0x28], "[p-hat]"),
        (&[0x62, 0x29], "[p-hat]1"),
        (&[0x62, 0x2A], "[p-hat]2"),
        (&[0x62, 0x2B], "[x-bar]1"),
        (&[0x62, 0x2C], "Sx1"),
        (&[0x62, 0x2D], "n_1"),
        (&[0x62, 0x2E], "[x-bar]2"),
        (&[0x62, 0x2F], "Sx2"),
        (&[0x62, 0x30], "n_2"),
        (&[0x62, 0x31], "Sxp"),
        (&[0x62, 0x32], "lower"),
        (&[0x62, 0x33], "upper"),
        (&[0x62, 0x35], "r2"),
        (&[0x62, 0x36], "R2"),
        (&[0x62, 0x38], "SS"),
        (&[0x62, 0x39], "MS"),
        (&[0x63, 0x00], "ZXscl"),
        (&[0x63, 0x01], "ZYscl"),
        (&[0x63, 0x02], "Xscl"),
        (&[0x63, 0x03], "Yscl"),
        (&[0x63, 0x04], "U_nStart"),
        (&[0x63, 0x05], "V_nStart"),
        (&[0x63, 0x06], "U_(n-1)"),
        (&[0x63, 0x07], "V_(n-1)"),
        // System variables (TODO)
        (&[0xAA, 0x00], "Str1"),
        (&[0xAA, 0x01], "Str2"),
        (&[0xAA, 0x02], "Str3"),
        (&[0xAA, 0x03], "Str4"),
        (&[0xAA, 0x04], "Str5"),
        (&[0xAA, 0x05], "Str6"),
        (&[0xAA, 0x06], "Str7"),
        (&[0xAA, 0x07], "Str8"),
        (&[0xAA, 0x08], "Str9"),
        (&[0xAA, 0x09], "Str0"),
        // BB tokens (two-byte), Incomplete TODO
        (&[0xBB, 0x00], "npv("),
        (&[0xBB, 0x01], "irr("),
        (&[0xBB, 0x02], "bal("),
        (&[0xBB, 0x03], "SummPrn("),
        (&[0xBB, 0x04], "SummInt("),
        (&[0xBB, 0x05], ">Nom("),
        (&[0xBB, 0x06], ">Eff("),
        (&[0xBB, 0x07], "dbd("),
        (&[0xBB, 0x08], "Icm("),
        (&[0xBB, 0x09], "gcd("),
        (&[0xBB, 0x0A], "randInt("),
        (&[0xBB, 0x0B], "randBin("),
        (&[0xBB, 0x0C], "sub("),
        (&[0xBB, 0x0D], "stdDev("),
        (&[0xBB, 0x0E], "variance("),
        (&[0xBB, 0x0F], "inString("),
        (&[0xBB, 0xB0], "a"),
        (&[0xBB, 0xB1], "b"),
        (&[0xBB, 0xB2], "c"),
        (&[0xBB, 0xB3], "d"),
        (&[0xBB, 0xB4], "e"),
        (&[0xBB, 0xB5], "f"),
        (&[0xBB, 0xB6], "g"),
        (&[0xBB, 0xB7], "h"),
        (&[0xBB, 0xB8], "i"),
        (&[0xBB, 0xB9], "j"),
        (&[0xBB, 0xBA], "k"),
        (&[0xBB, 0xBC], "l"),
        (&[0xBB, 0xBD], "m"),
        (&[0xBB, 0xBE], "n"),
        (&[0xBB, 0xBF], "o"),
        (&[0xBB, 0xC0], "p"),
        (&[0xBB, 0xC1], "q"),
        (&[0xBB, 0xC2], "r"),
        (&[0xBB, 0xC3], "s"),
        (&[0xBB, 0xC4], "t"),
        (&[0xBB, 0xC5], "u"),
        (&[0xBB, 0xC6], "v"),
        (&[0xBB, 0xC7], "w"),
        (&[0xBB, 0xC8], "x"),
        (&[0xBB, 0xC9], "y"),
        (&[0xBB, 0xCA], "z"),
        // end of xBB
        (&[0xEF, 0x67], "TextColor("),
        (&[0xEF, 0x5B], "BackgroundOn "),
        (&[0xEF, 0x41], "BLUE"),
        (&[0xEF, 0x42], "RED"),
        (&[0xEF, 0x43], "BLACK"),
        (&[0xEF, 0x44], "MAGENTA"),
        (&[0xEF, 0x45], "GREEN"),
        (&[0xEF, 0x46], "ORANGE"),
        (&[0xEF, 0x47], "BROWN"),
        (&[0xEF, 0x48], "NAVY"),
        (&[0xEF, 0x49], "LTBLUE"),
        (&[0xEF, 0x4A], "YELLOW"),
        (&[0xEF, 0x4B], "WHITE"),
        (&[0xEF, 0x4C], "LTGRAY"),
        (&[0xEF, 0x4D], "MEDGRAY"),
        (&[0xEF, 0x4E], "GRAY"),
        (&[0xEF, 0x4F], "DRAKGRAY"),
        (&[0xEF, 0x09], "getDate"),
        (&[0xEF, 0x0A], "getTime"),
        (&[0xEF, 0x06], "dayOfWk("),
        (&[0xBB, 0x52], "ClrAllLists"),
        (&[0xEF, 0x64], "BackgroundOff"),
        (&[0xEF, 0x32], "remainder("),
        (&[0xEF, 0x6C], "BorderColor "),
        (&[0xEF, 0x37], "MATHPRINT"),
        (&[0xEF, 0x3B], "AUTO"),
        (&[0xEF, 0x39], "[n/d]"),
        (&[0xBB, 0x4F], "a+bi"),
        (&[0x7E, 0x06], "Thick"),
        (&[0xEF, 0x10], "ClockOn"),
        (&[0xEF, 0x0F], "ClockOff"),
        (&[0xEF, 0x96], "Wait "),
        (&[0xEF, 0x38], "CLASSIC"),
        (&[0xEF, 0x75], "Dot-Thin"),
        (&[0xBB, 0x4D], "Real"),
        (&[0xEF, 0x3A], "[Un/d]"),
        (&[0xBB, 0x67], "DiagnosticOff"),
        (&[0xEF, 0x40], "STARTWIZARD OFF"),
        (&[0xBB, 0x51], "ExprOff"),
        (&[0xEF, 0x6B], "DetectAsymOff"),
        (&[0xEF, 0x6A], "DetectAsymOn"),
        (&[0xBB, 0x66], "DiagnosticOn"),
        (&[0xEF, 0x3F], "STARTWIZARD ON"),
        (&[0xBB, 0x50], "ExprOn"),
        (&[0xBB, 0x4A], "SetUpEditor "),
        // 73 ** (Graph Options)
        (&[0x7E, 0x00], "Sequential"),
        (&[0x7E, 0x01], "Simul"),
        (&[0x7E, 0x02], "PolarGC"),
        (&[0x7E, 0x03], "RectGC"),
        (&[0x7E, 0x04], "CoordOn"),
        (&[0x7E, 0x05], "CoordOff"),
        (&[0x7E, 0x06], "Connected"),
        (&[0x7E, 0x07], "Dot"),
        (&[0x7E, 0x08], "AxesOn"),
        (&[0x7E, 0x09], "AxesOff"),
        (&[0x7E, 0x0A], "GridOn"),
        (&[0x7E, 0x0B], "GridOff"),
        (&[0x7E, 0x0C], "LabelOn"),
        (&[0x7E, 0x0D], "LabelOff"),
        (&[0x7E, 0x0E], "Web"),
        (&[0x7E, 0x0F], "Time"),
        (&[0x7E, 0x10], "uvAxes"),
        (&[0x7E, 0x11], "vwAxes"),
        (&[0x7E, 0x12], "uwAxes"),
        (&[0xBB, 0x68], "Archive "),
        (&[0xBB, 0x54], "DelVar "),
    ].iter().copied().collect();
}