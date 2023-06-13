trait Decoder {
    fn decode(self, l: impl AsRef<Vec<char>>) -> EasterBunnyDeflaterWrapper;
}

enum EasterBunnyDeflaterWrapper {
    RawCopy(EasterBunnyDeflater<RawCopy>),
    TokenDecode(EasterBunnyDeflater<TokenDecode>),
    TokenExecute(EasterBunnyDeflater<TokenExecute>),
    Done(EasterBunnyDeflater<Done>),
}

impl<'a> Decoder for EasterBunnyDeflaterWrapper {
    fn decode(self, l: impl AsRef<Vec<char>>) -> EasterBunnyDeflaterWrapper {
        match self {
            EasterBunnyDeflaterWrapper::RawCopy(rc) => rc.decode(l),
            EasterBunnyDeflaterWrapper::TokenDecode(td) => td.decode(l),
            EasterBunnyDeflaterWrapper::TokenExecute(te) => te.decode(l),
            EasterBunnyDeflaterWrapper::Done(_) => self,
        }
    }
}

pub struct EasterBunnyDeflater<S> {
    decoded_string: String,
    pos: usize,
    state: S,
}

pub struct RawCopy {}
impl EasterBunnyDeflater<RawCopy> {
    fn new() -> Self {
        EasterBunnyDeflater::<RawCopy> {
            decoded_string: String::new(),
            pos: 0,
            state: RawCopy {},
        }
    }
}
impl EasterBunnyDeflater<RawCopy> {
    pub fn decode_line(line: impl AsRef<str>) -> String {
        let mut easter_bunny_deflater =
            EasterBunnyDeflaterWrapper::RawCopy(EasterBunnyDeflater::new());

        let chars: Vec<char> = line.as_ref().chars().collect();
        loop {
            easter_bunny_deflater = easter_bunny_deflater.decode(&chars);
            match easter_bunny_deflater {
                EasterBunnyDeflaterWrapper::Done(done) => {
                    return done.decoded_string;
                }
                _ => {}
            }
        }
    }
}

impl Decoder for EasterBunnyDeflater<RawCopy> {
    fn decode(mut self, l: impl AsRef<Vec<char>>) -> EasterBunnyDeflaterWrapper {
        if self.pos >= l.as_ref().len() {
            return EasterBunnyDeflaterWrapper::Done(EasterBunnyDeflater {
                decoded_string: self.decoded_string,
                pos: self.pos,
                state: Done {},
            });
        }

        let c = l.as_ref()[self.pos as usize];
        self.pos += 1;

        if c == '(' {
            EasterBunnyDeflaterWrapper::TokenDecode(EasterBunnyDeflater {
                decoded_string: self.decoded_string,
                pos: self.pos,
                state: TokenDecode {
                    token_str: String::new(),
                },
            })
        } else {
            self.decoded_string.push(c);
            EasterBunnyDeflaterWrapper::RawCopy(self)
        }
    }
}

struct TokenDecode {
    token_str: String,
}
impl Decoder for EasterBunnyDeflater<TokenDecode> {
    fn decode(mut self, l: impl AsRef<Vec<char>>) -> EasterBunnyDeflaterWrapper {
        let c = l.as_ref()[self.pos as usize];
        self.pos += 1;
        if c != ')' {
            self.state.token_str.push(c);
            EasterBunnyDeflaterWrapper::TokenDecode(self)
        } else {
            let (characters_string, iterations_string) =
                self.state.token_str.split_once("x").unwrap();
            EasterBunnyDeflaterWrapper::TokenExecute(EasterBunnyDeflater {
                decoded_string: self.decoded_string,
                pos: self.pos,
                state: TokenExecute {
                    source_chars: String::new(),
                    characters: characters_string.parse().unwrap(),
                    iterations: iterations_string.parse().unwrap(),
                },
            })
        }
    }
}

struct Done {}

struct TokenExecute {
    source_chars: String,
    characters: usize,
    iterations: usize,
}
impl Decoder for EasterBunnyDeflater<TokenExecute> {
    fn decode(mut self, l: impl AsRef<Vec<char>>) -> EasterBunnyDeflaterWrapper {
        if self.state.source_chars.len() < self.state.characters {
            let c = l.as_ref()[self.pos as usize];
            self.pos += 1;
            self.state.source_chars.push(c);
            EasterBunnyDeflaterWrapper::TokenExecute(self)
        } else {
            if self.state.source_chars.contains('(') && self.state.source_chars.contains(')') {
                self.state.source_chars = EasterBunnyDeflater::decode_line(self.state.source_chars);
            }

            for _ in 0..self.state.iterations {
                self.decoded_string.push_str(&self.state.source_chars);
            }
            EasterBunnyDeflaterWrapper::RawCopy(EasterBunnyDeflater {
                decoded_string: self.decoded_string,
                pos: self.pos,
                state: RawCopy {},
            })
        }
    }
}
