use std::{ops::Deref, string};

use strum::EnumIter;
use tracing::debug;
use tracing_subscriber::field::debug;

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

// This struct exists for the sole reason that tan tan is able to be prompted by multiple or one matching string in a msg.
#[derive(Clone)]
pub struct Prompt {
    pub matching_strings: Vec<String>,
    pub multiple_strings: Vec<Vec<String>>,
}

impl Prompt {
    // Personally constructing prompts like this for readability (plus i like structs)
    pub fn new() -> Prompt {
        Prompt {
            matching_strings: Vec::new(),
            multiple_strings: Vec::new(),
        }
    }
    pub fn add_prompt(&self, prompt: &str) -> Prompt {
        let mut prompt_self = self.clone();
        prompt_self.matching_strings.push(prompt.to_string());
        prompt_self.clone()
    }
    pub fn add_prompts(&self, prompts: Vec<&str>) -> Prompt {
        let mut prompt_self = self.clone();
        for prompt in prompts {
            prompt_self.matching_strings.push(prompt.to_string());
        }
        prompt_self.clone()
    }
    /// A string must match to these strings in order to equal to.
    pub fn add_multi_prompt(&self, prompts: Vec<&str>) -> Prompt {
        let mut prompt_self = self.clone();
        let mut prompts_but_string: Vec<String> = prompts.iter().map(|x| x.to_string()).collect();
        prompt_self.multiple_strings.push(prompts_but_string);
        prompt_self.clone()
    }

    pub fn r#match(&self, string: String) -> bool {
        // check if there is any multi prompts, go through those first.
        // then check and filter through all the other standard prompts
        if !self.multiple_strings.is_empty() {
            for x in &self.multiple_strings {
                let mut matched: i32 = i32::default();
                for y in x {
                    if string.contains(y) {
                        matched += 1
                    }
                }
                if matched == x.len() as i32 {
                    return true;
                }
            }
        }
        // ...if code reaches here, multiple string either doesnt exist or doesn't match
        for x in &self.matching_strings {
            if string.contains(x) {
                return true;
            }
        }
        /* `bool` value */
        false
    }
}

impl TanTanPrompts {
    pub fn get_prompts(&self) -> Prompt {
        match self {
            TanTanPrompts::Hello => Prompt::new().add_prompts(vec!["hai", "hi", "hey"]),
            TanTanPrompts::GoodMorning => {
                Prompt::new().add_prompts(vec!["good morning", "morning"])
            }
            TanTanPrompts::Question => Prompt::new().add_multi_prompt(vec!["?", "tan tan"]),
            TanTanPrompts::TanTan => Prompt::new().add_prompts(vec!["tan tan", "tantan"]),
            TanTanPrompts::WelcomeNewMember => {
                Prompt::new().add_prompts(vec!["welcome", "hello new member"])
            }
            TanTanPrompts::Blehhh => Prompt::new().add_prompts(vec!["bleh", "blah"]),
            TanTanPrompts::ColonThree => Prompt::new().add_prompts(vec![":3", "colon three"]),
            TanTanPrompts::ILoveYou => {
                Prompt::new().add_prompts(vec!["tan tan i love you", "i love you", "ily"])
            }
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
            TanTanPrompts::ILoveYou => vec!["i love you more {}", "i love you too {}"],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_hello_not_eq() {
        // should be false
        let hello = TanTanPrompts::Hello.get_prompts().r#match("h".to_string());
        assert!(!hello);
    }
    #[test]
    fn prompt_hello_eq() {
        // should be true
        let hello = TanTanPrompts::Hello
            .get_prompts()
            .r#match("hey".to_string());
        assert!(hello);
    }
    #[test]
    fn multiprompt_eq() {
        // true
        let hello = TanTanPrompts::Question
            .get_prompts()
            .r#match("tan tan ?".to_string());
        assert!(hello);
    }
    #[test]
    fn multiprompt_not_eq() {
        // false
        let hello = TanTanPrompts::Question
            .get_prompts()
            .r#match("tan tan".to_string());
        assert!(!hello);
    }
}

struct Prompts;
