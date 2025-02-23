use strum::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum TanTanPrompts {
    Hello,
    GoodMorning,
    Question,
    TanTan,
    WelcomeNewMember,
    Blehhh,
    ColonThree,
    ILoveYou,
}

impl TanTanPrompts {
    pub fn get_prompts(&self) -> Vec<&'static str> {
        match self {
            TanTanPrompts::Hello => vec!["hai", "hey", "hi"],
            TanTanPrompts::GoodMorning => vec!["good morning", "morning"],
            TanTanPrompts::Question => vec!["?"],
            TanTanPrompts::TanTan => vec!["tan tan", "tantan"],
            TanTanPrompts::WelcomeNewMember => vec!["welcome", "hello new member"],
            TanTanPrompts::Blehhh => vec!["bleh", "blah"],
            TanTanPrompts::ColonThree => vec![":3", "colon three"],
            TanTanPrompts::ILoveYou => vec!["tan tan i love you", "i love you", "ily"],
        }
    }

    pub fn get_replies(&self) -> Vec<&'static str> {
        match self {
            TanTanPrompts::Hello => vec![
                "great to see you here {} :3",
                "yoyoyoyoyoyoyoyo {} :3",
                "{}! its about time you showed up! :3",
                "whats up {} im tan tan, yo",
                "hey {}! hru?",
                "whats good {}?",
                "welcome back {}! how are you?",
            ],
            TanTanPrompts::GoodMorning => vec![
                "gm {}, remember to drink some water :)",
                "rise and shine {}!",
            ],
            TanTanPrompts::Question => vec!["yes", "no", "mayyybe", "idk :P"],
            TanTanPrompts::TanTan => vec![
                "yo wassupa",
                "is it me you're looking for?",
                "tan tan reporting for duty",
            ],
            TanTanPrompts::WelcomeNewMember => vec!["welcome new member"],
            TanTanPrompts::Blehhh => vec!["blehhh :P"],
            TanTanPrompts::ColonThree => vec![":3"],
            TanTanPrompts::ILoveYou => {
                vec!["i love you more {}", "i love you too {}"]
            }
        }
    }
}

pub struct Tantan {
    text: TanTanPrompts,
}

struct Replies {
    Hello: Vec<String>,
    GoodMorning: Vec<String>,
    Question: Vec<String>,
    TanTan: Vec<String>,
    WelcomeNewMember: Vec<String>,
    Blehhh: Vec<String>,
    ColonThree: Vec<String>,
    ILoveYou: Vec<String>,
}

struct Prompts;
