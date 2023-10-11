use logos::Logos;

#[derive(Logos, Debug)]
#[logos(subpattern float=r"[0-9]+\.[0-9]+")]
pub enum Token<'a> {
    #[regex(r"l[1-6]")]
    List(&'a str),
    #[regex(r"\[[A-J]\]")]
    Matrix(&'a str),

    #[regex(r"[A-Za-z-]+\(")]
    Function(&'a str),
    #[regex(r"[A-Z]{1}[a-z]+\s?", priority=3)]
    Keyword(&'a str),

    // things like ClrHome, AxesOff, etc.
    #[regex(r"[A-Z]{1}[A-Za-z]+")]
    SingleLineFunction(&'a str),

    #[regex(r"→|->")]
    Store(&'a str),
    #[token("=")]
    Comparison(&'a str),

    #[regex("(?&float)+", priority = 2)]
    Float(&'a str),
    #[regex(r"[0-9]+", priority = 1)]
    Int(&'a str),
    #[regex(r"[A-Z]{1}|θ")]
    Variable(&'a str)
}