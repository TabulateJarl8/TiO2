#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum OpCode {
/// >DMS
ToDms = 0x01,
/// >Dec
ToDec = 0x02,
/// >Frac
ToFrac = 0x03,
/// ->
Store = 0x04,
/// BoxPlot
Boxplot = 0x05,
/// [
LBracket = 0x06,
/// ]
RBracket = 0x07,
/// {
LBrace = 0x08,
/// }
RBrace = 0x09,
/// °
Degree = 0x0B,
/// ^-1
Invert = 0x0C,
/// ^2
Square = 0x0D,
/// ^T
Transpose = 0x0E,
/// ^3
Cube = 0x0F,
/// (
LParen = 0x10,
/// )
RParen = 0x11,
/// round(
Round = 0x12,
/// pxl-Test(
PxlTest = 0x13,
/// augment(
Augment = 0x14,
/// rowSwap(
RowSwap = 0x15,
/// row+(
RowAdd = 0x16,
/// *row(
RowMul = 0x17,
/// *row+(
RowMulAdd = 0x18,
/// max(
Max = 0x19,
/// min(
Min = 0x1A,
/// R>Pr(
RectToPolarR = 0x1B,
/// R>Pθ(
RectToPolarTheta = 0x1C,
/// P>Rx(
PolarToRectX = 0x1D,
/// P>Ry(
PolarToRectY = 0x1E,
/// median(
Median = 0x1F,
/// randM(
RandMatrix = 0x20,
/// mean(
Mean = 0x21,
/// solve(
Solve = 0x22,
/// seq(
Seq = 0x23,
/// fnInt(
FnInt = 0x24,
/// nDeriv(
NDeriv = 0x25,
/// fMin(
FMin = 0x27,
/// fMax(
FMax = 0x28,
/// " "
Space = 0x29,
/// \"
Quote = 0x2A,
/// ,
Comma = 0x2B,
/// imaginary
Imaginary = 0x2C,
/// !
Factorial = 0x2D,
/// CubicReg
CubicReg = 0x2E,
/// QuartReg
QuartReg = 0x2F,
/// .
Dot = 0x3A,
/// E (10^)
Exp = 0x3B,
///  or
Or = 0x3C,
///  xor
Xor = 0x3D,
/// :
Colon = 0x3E,
/// \n
Newline = 0x3F,
///  and
And = 0x40,
/// A
A = 0x41,
/// B
B = 0x42,
/// C
C = 0x43,
/// D
D = 0x44,
/// E
E = 0x45,
/// F
F = 0x46,
/// G
G = 0x47,
/// H
H = 0x48,
/// I
I = 0x49,
/// J
J = 0x4A,
/// K
K = 0x4B,
/// L
L = 0x4C,
/// M
M = 0x4D,
/// N
N = 0x4E,
/// O
O = 0x4F,
/// P
P = 0x50,
/// Q
Q = 0x51,
/// R
R = 0x52,
/// S
S = 0x53,
/// T
T = 0x54,
/// U
U = 0x55,
/// V
V = 0x56,
/// W
W = 0x57,
/// X
X = 0x58,
/// Y
Y = 0x59,
/// Z
Z = 0x5A,
/// 0
Num0 = 0x30,
/// 1
Num1 = 0x31,
/// 2
Num2 = 0x32,
/// 3
Num3 = 0x33,
/// 4
Num4 = 0x34,
/// 5
Num5 = 0x35,
/// 6
Num6 = 0x36,
/// 7
Num7 = 0x37,
/// 8
Num8 = 0x38,
/// 9
Num9 = 0x39,
/// θ
Theta = 0x5B,
/// prgm
Prgm = 0x5F,
/// Radian
ModeRadian = 0x64,
/// Degree
ModeDegree = 0x65,
/// Normal
ModeNormal = 0x66,
/// Sci
ModeSci = 0x67,
/// Eng
ModeEng = 0x68,
/// Float
ModeFloat = 0x69,
/// =
Eq = 0x6A,
/// <
Lt = 0x6B,
/// >
Gt = 0x6C,
/// <=
Lte = 0x6D,
/// >=
Gte = 0x6E,
/// !=
Neq = 0x6F,
/// +
Plus = 0x70,
/// –
Minus = 0x71,
/// Ans
Ans = 0x72,
/// Fix
ModeFix = 0x73,
/// Horiz
ModeHoriz = 0x74,
/// Full
ModeFull = 0x75,
/// Func
ModeFunc = 0x76,
/// Param
ModeParam = 0x77,
/// Polar
ModePolar = 0x78,
/// Seq
ModeSeq = 0x79,
/// IndpntAuto
ModeIndpntAuto = 0x7A,
/// IndpntAsk
ModeIndpntAsk = 0x7B,
/// DependAuto
ModeDependAuto = 0x7C,
/// DependAsk
ModeDependAsk = 0x7D,
/// ▫︎
SolidSquare = 0x7F,
/// ˖
Circumflex = 0x80,
/// ·
Bullet = 0x81,
/// *
Mul = 0x82,
/// /
Div = 0x83,
/// Trace
Trace = 0x84,
/// ClrDraw
ClrDraw = 0x85,
/// ZStandard
Zstandard = 0x86,
/// ZTrig
Ztrig = 0x87,
/// ZBox
Zbox = 0x88,
/// Zoom_In
ZoomIn = 0x89,
/// Zoom_Out
ZoomOut = 0x8A,
/// ZSquare
Zsquare = 0x8B,
/// ZInteger
Zinteger = 0x8C,
/// ZPrevious
Zprevious = 0x8D,
/// ZDecimal
Zdecimal = 0x8E,
/// ZoomStat
ZoomStat = 0x8F,
/// ZoomRcl
ZoomRcl = 0x90,
/// PrintScreen
PrintScreen = 0x91,
/// ZoomSto
ZoomSto = 0x92,
/// Text(
Text = 0x93,
///  nPr
Npr = 0x94,
///  nCr
Ncr = 0x95,
/// FnOn
FnOn = 0x96,
/// FnOff
FnOff = 0x97,
/// StorePic
StorePic = 0x98,
/// RecallPic
RecallPic = 0x99,
/// StoreGDB
StoreGdb = 0x9A,
/// RecallGDB
RecallGdb = 0x9B,
/// Line(
Line = 0x9C,
/// Vertical
Vertical = 0x9D,
/// Pt-On(
PtOn = 0x9E,
/// Pt-Off(
PtOff = 0x9F,
/// Pt-Change(
PtChange = 0xA0,
/// Pxl-On(
PxlOn = 0xA1,
/// Pxl-Off(
PxlOff = 0xA2,
/// Pxl-Change(
PxlChange = 0xA3,
/// Shade(
Shade = 0xA4,
/// Circle(
Circle = 0xA5,
/// Horizontal
Horizontal = 0xA6,
/// Tangent(
Tangent = 0xA7,
/// DrawInv
DrawInv = 0xA8,
/// DrawF
DrawF = 0xA9,
/// rand
Rand = 0xAB,
/// π
Pi = 0xAC,
/// getKey
Getkey = 0xAD,
/// \'
Apostrophe = 0xAE,
/// ?
QuestionMark = 0xAF,
/// -
Negative = 0xB0,
/// int(
Int = 0xB1,
/// abs(
Abs = 0xB2,
/// det(
Det = 0xB3,
/// identity(
Identity = 0xB4,
/// dim(
Dim = 0xB5,
/// sum(
Sum = 0xB6,
/// prod(
Prod = 0xB7,
/// not(
Not = 0xB8,
/// iPart(
Ipart = 0xB9,
/// fPart(
Fpart = 0xBA,
/// sqrt(
Sqrt = 0xBC,
/// cubrt(
Cubrt = 0xBD,
/// ln(
Ln = 0xBE,
/// e^(
PowE = 0xBF,
/// log(
Log = 0xC0,
/// 10^(
Pow10 = 0xC1,
/// sin(
Sin = 0xC2,
/// sin^-1(
Asin = 0xC3,
/// cos(
Cos = 0xC4,
/// cos^-1(
Acos = 0xC5,
/// tan(
Tan = 0xC6,
/// tan^-1(
Atan = 0xC7,
/// sinh(
Sinh = 0xC8,
/// sinh^-1(
Asinh = 0xC9,
/// cosh(
Cosh = 0xCA,
/// cosh^-1(
Acosh = 0xCB,
/// tanh(
Tanh = 0xCC,
/// tanh^-1(
Atanh = 0xCD,
/// If
If = 0xCE,
/// Then
Then = 0xCF,
/// Else
Else = 0xD0,
/// While
While = 0xD1,
/// Repeat
Repeat = 0xD2,
/// For
For = 0xD3,
/// End
End = 0xD4,
/// Return
Return = 0xD5,
/// Lbl
Lbl = 0xD6,
/// Goto
Goto = 0xD7,
/// Pause
Pause = 0xD8,
/// Stop
Stop = 0xD9,
/// IS>(
IncSkipGt = 0xDA,
/// DS>(
DecSkipGt = 0xDB,
/// Input
Input = 0xDC,
/// Prompt
Prompt = 0xDD,
/// Disp
Disp = 0xDE,
/// DispGraph
DispGraph = 0xDF,
/// Output(
Output = 0xE0,
/// ClrHome
ClrHome = 0xE1,
/// Fill(
Fill = 0xE2,
/// SortA(
SortA = 0xE3,
/// SortD(
SortD = 0xE4,
/// DispTable
DispTable = 0xE5,
/// Menu(
Menu = 0xE6,
/// Send(
Send = 0xE7,
/// Get(
Get = 0xE8,
/// PlotsOn
PlotsOn = 0xE9,
/// PlotsOff
PlotsOff = 0xEA,
/// l
List = 0xEB,
/// Plot1(
Plot1 = 0xEC,
/// Plot2(
Plot2 =0xED,
/// Plot3(
Plot3 = 0xEE,
/// ^
Pow = 0xF0,
/// [xth root]
XthRoot = 0xF1,
/// 1-Var Stats
1varStats = 0xF2,
/// 2-Var Stats
2varStats = 0xF3,
/// LinReg(a+bx)
LinRegABx = 0xF4,
/// ExpReg
ExpReg = 0xF5,
/// LnReg
LnReg = 0xF6,
/// PwrReg
PwrReg = 0xF7,
/// Med-Med
MedMed = 0xF8,
/// QuadReg
QuadReg = 0xF9,
/// ClrList
ClrList = 0xFA,
/// ClrTable
ClrTable = 0xFB,
/// Histogram
Histogram = 0xFC,
/// xyLine
XyLine = 0xFD,
/// Scatter
Scatter = 0xFE,
/// LinReg(ax+b)
LinRegAxB = 0xFF,

/// [A]
MatrixA = 0x5C00,
/// [B]
MatrixB = 0x5C01 ,
/// [C]
MatrixC = 0x5C02,
/// [D]
MatrixD = 0x5C03,
/// [E]
MatrixE = 0x5C04,
/// [F]
MatrixF = 0x5C05,
/// [G]
MatrixG = 0x5C06,
/// [H]
MatrixH = 0x5C07,
/// [I]
MatrixI = 0x5C08,
/// [J]
MatrixJ = 0x5C09,

/// l1
L1 = 0x5D00,
/// l2
L2 =  0x5D01,
/// l3
L3 =  0x5D02,
/// l4
L4 =  0x5D03,
/// l5
L5 =  0x5D04,
/// l6
L6 =  0x5D05,
/// l7
L7 =  0x5D06,
/// l8
L8 =  0x5D07,
/// l0
L0 =  0x5D08,

/// y1
Y1 = 0x5E10,
/// y2
Y2 = 0x5E11,
/// y3
Y3 = 0x5E12,
/// y4
Y4 = 0x5E13,
/// y5
Y5 = 0x5E14,
/// y6
Y6 = 0x5E15,
/// y7
Y7 = 0x5E16,
/// y8
Y8 = 0x5E17,
/// y9
Y9 = 0x5E18,
/// y0
Y0 = 0x5E19,
/// r_1
R1 = 0x5E40,
/// r_2
R2 = 0x5E41,
/// r_3
R3 = 0x5E42,
/// r_4
R4 = 0x5E43,
/// r_5
R5 = 0x5E44,
/// r_6
R6 = 0x5E45,
/// x1t
X1t = 0x5E20,
/// y1t
Y1t = 0x5E21,
/// x2t
X2t = 0x5E22,
/// y2t
Y2t = 0x5E23,
/// x3t
X3t = 0x5E24,
/// y3t
Y3t = 0x5E25,
/// x4t
X4t = 0x5E26,
/// y4t
Y4t = 0x5E27,
/// x5t
X5t = 0x5E28,
/// y5t
Y5t = 0x5E29,
/// x6t
X6t = 0x5E2A,
/// y6t
Y6t = 0x5E2B,
/// Sequence variable: u in u(n)
SeqU = 0x5E80,
/// Sequence variable: v in v(n)
SeqV = 0x5E81,
/// Sequence variable: w in w(n)
SeqW = 0x5E82,

/// Pic1
Pic1 = 0x6000,
/// Pic2
Pic2 = 0x6001,
/// Pic3
Pic3 = 0x6002,
/// Pic4
Pic4 = 0x6003,
/// Pic5
Pic5 = 0x6004,
/// Pic6
Pic6 = 0x6005,
/// Pic7
Pic7 = 0x6006,
/// Pic8
Pic8 = 0x6007,
/// Pic9
Pic9 = 0x6008,
/// Pic0
Pic0 = 0x6009,

/// GDB1
Gdb1 = 0x6100,
/// GDB2
Gdb2 = 0x6101,
/// GDB3
Gdb3 = 0x6102,
/// GDB4
Gdb4 = 0x6103,
/// GDB5
Gdb5 = 0x6104,
/// GDB6
Gdb6 = 0x6105,
/// GDB7
Gdb7 = 0x6106,
/// GDB8
Gdb8 = 0x6107,
/// GDB9
Gdb9 = 0x6108,
/// GDB0
Gdb0 = 0x6109,

/// RegEq
RegEq =  0x6201,
/// [x-bar]
XBar =  0x6203,
/// [Summ x]
SumX =  0x6204,
/// [Summ x^2]
SumXSq =  0x6205,
/// Sx
StddevX =  0x6206,
/// [sigma]x
SigmaX =  0x6207,
/// minX
MinX =  0x6208,
/// maxX
MaxX =  0x6209,
/// minY
MinY =  0x620A,
/// maxY
MaxY =  0x620B,
/// [y-bar]
YBar =  0x620C,
/// [Summ y]
SumY =  0x620D,
/// [Summ y^2]
SumYSq =  0x620E,
/// Sy
StddevY =  0x620F,
/// [sigma]y
SigmaY =  0x6210,
/// [Summ xy]
SumXy =  0x6211,
/// Med
Median =  0x6213,
/// Q1
Q1 =  0x6214,
/// Q3
Q3 =  0x6215,
/// Regression variable a
RegA =  0x6216,
/// Regression variable b
RegB =  0x6217,
/// Regression variable c
RegC =  0x6218,
/// Regression variable d
RegD =  0x6219,
/// Regression variable e
RegE =  0x621a,
/// x1
X1 =  0x621B,
/// x2
X2 =  0x621C,
/// x3
X3 =  0x621D,
/// y1
Y1 =  0x621E,
/// y2
Y2 =  0x621F,
/// y3
Y3 =  0x6220,

// TODO:
/// n
N =  0x6221,
/// p
P =  0x6222,
/// z
Z =  0x6223,
/// t
T =  0x6224,
// TODO END

/// [chi]2
ChiSq =  0x6225,
/// [fin]
Fin =  0x6226,
/// df
Df =  0x6227,
/// [p-hat]
PHat =  0x6228,
/// [p-hat]1
PHat1 =  0x6229,
/// [p-hat]2
PHat2 =  0x622A,
/// [x-bar]1
XBar1 =  0x622B,
/// Sx1
StddevX1 =  0x622C,
/// n_1
N1 =  0x622D,
/// [x-bar]2
XBar2 =  0x622E,
/// Sx2
StddevX2 =  0x622F,
/// n_2
N2 =  0x6230,
/// Sxp
StddevXp =  0x6231,
/// lower
Lower =  0x6232,
/// upper
Upper =  0x6233,
/// r2
RSq =  0x6235,
/// R2
RSqCapital =  0x6236,
/// Factor df
FactorDf =  0x6237,
/// Factor SS
FactorSs =  0x6238,
/// Factor MS
FactorMs =  0x6239,
/// Error DF
ErrorDf =  0x623A,
/// Error SS
ErrorSs =  0x623B,
/// Error MS
ErrorMs =  0x623C,

/// ZXscl
ZxScl =  0x6300,
/// ZYscl
ZyScl =  0x6301,
/// Xscl
XScl =  0x6302,
/// Yscl
YScl =  0x6303,

    // TODO: changeme
/// u(nMin)
UMin =  0x6304,
/// v(nMin)
VMin =  0x6305,
    // TODO END

/// u(n-1)
UNMinus1 = 0x6306,
/// v(n-1)
VNMinus1 = 0x6307,

// TODO: most of these arent acctually supported:
// http://tibasicdev.wikidot.com/window-tokens
/// Str1
Str1 = 0xAA00,
/// Str2
Str2 = 0xAA01,
/// Str3
Str3 = 0xAA02,
/// Str4
Str4 = 0xAA03,
/// Str5
Str5 = 0xAA04,
/// Str6
Str6 = 0xAA05,
/// Str7
Str7 = 0xAA06,
/// Str8
Str8 = 0xAA07,
/// Str9
Str9 = 0xAA08,
/// Str0
Str0 = 0xAA09,

// TODO: finish these
/// npv(
Npv = 0xBB00,
/// irr(
Irr = 0xBB01,
/// bal(
Bal = 0xBB02,
/// SummPrn(
SumPrn = 0xBB03,
/// SummInt(
SumInt = 0xBB04,
/// >Nom(
ToNomial = 0xBB05,
/// >Eff(
ToEffective = 0xBB06,
/// dbd(
Dbd = 0xBB07,
/// Icm(
Icm = 0xBB08,
/// gcd(
Gcd = 0xBB09,
/// randInt(
Randint = 0xBB0A,
/// randBin(
Randbin = 0xBB0B,
/// sub(
Substring = 0xBB0C,
/// stdDev(
Stddev = 0xBB0D,
/// variance(
Variance = 0xBB0E,
/// inString(
InString = 0xBB0F,
/// a
LowerA = 0xBBB0,
/// b
LowerB = 0xBBB1,
/// c
LowerC = 0xBBB2,
/// d
LowerD = 0xBBB3,
/// e
LowerE = 0xBBB4,
/// f
LowerF = 0xBBB5,
/// g
LowerG = 0xBBB6,
/// h
LowerH = 0xBBB7,
/// i
LowerI = 0xBBB8,
/// j
LowerJ = 0xBBB9,
/// k
LowerK = 0xBBBA,
/// l
LowerL = 0xBBBC,
/// m
LowerM = 0xBBBD,
/// n
LowerN = 0xBBBE,
/// o
LowerO = 0xBBBF,
/// p
LowerP = 0xBBC0,
/// q
LowerQ = 0xBBC1,
/// r
LowerR = 0xBBC2,
/// s
LowerS = 0xBBC3,
/// t
LowerT = 0xBBC4,
/// u
LowerU = 0xBBC5,
/// v
LowerV = 0xBBC6,
/// w
LowerW = 0xBBC7,
/// x
LowerX = 0xBBC8,
/// y
LowerY = 0xBBC9,
/// z
LowerZ = 0xBBCA,
/// ClrAllLists
ClrAllLists = 0xBB52,
/// a+bi
ModeImaginary = 0xBB4F,
/// Real
ModeReal = 0xBB4D,
/// DiagnosticOff
ModeDiagnosticOff = 0xBB67,
/// ExprOff
ModeExprOff = 0xBB51,
/// DiagnosticOn
ModeDiagnosticOn = 0xBB66,
/// ExprOn
ExprOn = 0xBB50,
/// SetUpEditor
SetUpEditor = 0xBB4A,
/// Archive
Archive = 0xBB68,
/// DelVar
DelVar = 0xBB54,
/// TextColor(
TextColor = 0xBB67,
/// BackgroundOn
BackgroundOn = 0xBB5B,
/// BLUE
Blue = 0xBB41,
/// RED
Red = 0xBB42,
/// BLACK
Black = 0xBB43,
/// MAGENTA
Magenta = 0xBB44,
/// GREEN
Green = 0xBB45,
/// ORANGE
Orange = 0xBB46,
/// BROWN
Brown = 0xBB47,
/// NAVY
Navy = 0xBB48,
/// LTBLUE
Ltblue = 0xBB49,
/// YELLOW
Yellow = 0xBB4A,
/// WHITE
White = 0xBB4B,
/// LTGRAY
Ltgray = 0xBB4C,
/// MEDGRAY
Medgray = 0xBB4D,
/// GRAY
Gray = 0xBB4E,
/// DRAKGRAY
Darkgray = 0xBB4F,
/// getDate
GetDate = 0xBB09,
/// getTime
GetTime = 0xBB0A,
/// dayOfWk(
DayOfWk = 0xBB06,
/// BackgroundOff
BackgroundOff = 0xBB64,
/// remainder(
Remainder = 0xBB32,
/// BorderColor
BorderColor = 0xBB6C,
/// MATHPRINT
ModeMathprint = 0xBB37,
/// AUTO
ModeAuto = 0xBB3B,
/// [n/d]
NOverD = 0xBB39,
/// ClockOn
ModeClockOn = 0xBB10,
/// ClockOff
ModeClockOff = 0xBB0F,
/// Wait
Wait = 0xBB96,
/// CLASSIC
ModeClassic = 0xBB38,
/// Dot-Thin
DotThin = 0xBB75,
/// [Un/d]
UnOverD = 0xBB3A,
/// STARTWIZARD OFF
StartwizardOff = 0xBB40,
/// DetectAsymOff
DetectAsymOff = 0xBB6B,
/// DetectAsymOn
DetectAsynOn = 0xBB6A,
/// Thick
Thick = 0xBB06,
/// STARTWIZARD ON
StartwizardOn = 0xBB3F,
/// Sequential
ModeSequential = 0xBB00,
/// Simul
ModeSimul = 0xBB01,
/// PolarGC
PolarGc = 0xBB02,
/// RectGC
RectGc = 0xBB03,
/// CoordOn
CoordOn = 0xBB04,
/// CoordOff
CoordOff = 0xBB05,
/// Dot
Dot = 0xBB07,
/// AxesOn
AxesOn = 0xBB08,
/// AxesOff
AxesOff = 0xBB09,
/// GridOn
GridOn = 0xBB0A,
/// GridOff
GridOff = 0xBB0B,
/// LabelOn
LabelOn = 0xBB0C,
/// LabelOff
LabelOff = 0xBB0D,
/// Web
Web = 0xBB0E,
/// Time
Time = 0xBB0F,
/// uvAxes
UvAxes = 0xBB10,
/// vwAxes
VwAxes = 0xBB11,
/// uwAxes
UwAxes = 0xBB12,
}
