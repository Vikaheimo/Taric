use std::{str::FromStr, fmt::Display};

#[derive(Debug)]
pub enum Servers {
    BR,     // BR1
    EUNE,   // EUN1
    EUW,    // EUW1
    JP,     // JP1
    KR,     // KR
    LAN,    // LA1
    LAS,    // LA2
    NA,     // NA1
    OCE,    // OC1
    PH,     // PH2
    RU,     // RU
    SG,     // SG2
    TH,     // TH2
    TR,     // TR1
    TW,     // TW2
    VN,     // VN2
}

impl Servers {
    pub fn to_lolapi_format(self) -> &'static str {
        match self{
            Servers::BR => "br1",
            Servers::EUNE => "eun1",
            Servers::EUW => "euw1",
            Servers::JP => "jp1",
            Servers::KR => "kr",
            Servers::LAN => "la1",
            Servers::LAS => "la2",
            Servers::NA => "na1",
            Servers::OCE => "oc1",
            Servers::PH => "ph2",
            Servers::RU => "ru",
            Servers::SG => "sg2",
            Servers::TH => "th2",
            Servers::TR => "tr1",
            Servers::TW => "tw2",
            Servers::VN => "vn2",
        }
    }
}

impl Display for Servers{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Servers {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Servers::*;
        match input.to_uppercase().as_str() {
            "BR"    => Ok(BR),
            "EUNE"  => Ok(EUNE),
            "EUW"   => Ok(EUW),
            "JP"    => Ok(JP),
            "KR"    => Ok(KR),
            "LAN"   => Ok(LAN),
            "LAS"   => Ok(LAS),
            "NA"    => Ok(NA),
            "OCE"   => Ok(OCE),
            "PH"    => Ok(PH),
            "RU"    => Ok(RU),
            "SG"    => Ok(SG),
            "TH"    => Ok(TH),
            "TR"    => Ok(TR),
            "TW"    => Ok(TW),
            "VN"    => Ok(VN),
            _       => Err(())
        }
    }
}

