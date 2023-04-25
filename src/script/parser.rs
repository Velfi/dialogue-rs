use std::fmt;

pub const RULE_LINE: &str = "Line";

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "script.pest"]
pub struct ScriptParser;

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Rule {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rule::Script => "Script",
            Rule::StartMarker => "Start Marker",
            Rule::EndMarker => "End Marker",
            Rule::Marker => "Marker",
            Rule::Subject => "subject",
            Rule::Text => "text",
            Rule::EOI => "End of Input",
            Rule::MarkerName => "marker name",
            Rule::Char => "single character",
            Rule::SubjectChar => "subject character",
            Rule::Space => "space",
            Rule::GotoCommand => "GOTO command",
            Rule::ChoiceCommand => "CHOICE command",
            Rule::InfixSayCommand => "SAY command (infix)",
            Rule::PostfixSayCommand => "SAY command (postfix)",
            Rule::SayCommand => "SAY command",
            Rule::StartOfScript => "Start of script",
            Rule::EndOfScript => "End of script",
            Rule::Line => "Line",
            Rule::IndentedBlock => "Indent",
        }
    }
}

#[cfg(test)]
mod test {
    use super::Rule;
    use super::ScriptParser;
    use pest::Parser;

    #[test]
    fn test_start_marker_parsing() {
        let _start_marker =
            ScriptParser::parse(Rule::StartMarker, "%START%\n").expect("parse succeeds");
    }

    #[test]
    fn test_end_marker_parsing() {
        let _end_marker = ScriptParser::parse(Rule::EndMarker, "%END%").expect("parse succeeds");
    }

    #[test]
    fn test_marker_parsing() {
        let markers = ["%12345%", "%HYPHENS-ARE-OK%", "%------%"];

        for marker in markers {
            let _marker = ScriptParser::parse(Rule::Marker, marker).expect("parse succeeds");
        }
    }

    #[test]
    #[should_panic = "ParsingError"]
    fn test_char_doesnt_parse_newlines() {
        let _char = ScriptParser::parse(Rule::Char, "\n").expect("parse succeeds");
    }

    #[test]
    fn test_text_parses_all_allowed_characters() {
        let everything = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890-=!@#$%^&*()_[]\\;',./{}|:\"<>?";
        let _text = ScriptParser::parse(Rule::Text, everything).expect("parse succeeds");
    }

    #[test]
    fn test_goto_command() {
        let _goto_command =
            ScriptParser::parse(Rule::GotoCommand, "GOTO %START%").expect("parse succeeds");
    }

    #[test]
    fn test_choice_command() {
        let _choice_command = ScriptParser::parse(Rule::ChoiceCommand, "CHOICE Succeed at parsing")
            .expect("parse succeeds");
    }

    #[test]
    fn test_say_command() {
        let lines = [
            "SAY \"Did it work?\" she asked herself",
            "ZELDA SAY \"Did it work?\" she asked herself",
        ];

        for line in lines {
            let _say_command = ScriptParser::parse(Rule::SayCommand, line).expect("parse succeeds");
        }
    }

    #[test]
    fn test_line_parse() {
        let lines = [
            "%START%\n",
            "SAY blah blah blah\n",
            "CHOICE Do something\n",
            "GOTO %THE-PLACE%\n",
        ];

        for line in lines {
            let _line = ScriptParser::parse(Rule::Line, line).expect("parse succeeds");
        }
    }

    #[test]
    fn test_two_line_script() {
        let script =
            std::fs::read_to_string("example_scripts/two-line.script").expect("script exists");
        let _parsed_script =
            ScriptParser::parse(Rule::Script, &script).expect("parse should succeed");
    }

    #[test]
    fn test_three_line_script() {
        let script =
            std::fs::read_to_string("example_scripts/three-line.script").expect("script exists");
        let _parsed_script =
            ScriptParser::parse(Rule::Script, &script).expect("parse should succeed");
    }

    #[test]
    fn test_jimi() {
        let script = std::fs::read_to_string("example_scripts/jimi.script").expect("script exists");
        let _parsed_script =
            ScriptParser::parse(Rule::Script, &script).expect("parse should succeed");
    }

    #[test]
    fn test_daisy_and_luigi() {
        let script = std::fs::read_to_string("example_scripts/daisy-and-luigi.script")
            .expect("script exists");
        let _parsed_script =
            ScriptParser::parse(Rule::Script, &script).expect("parse should succeed");
    }

    #[test]
    fn test_capital_of_spain() {
        let script = std::fs::read_to_string("example_scripts/capital-of-spain.script")
            .expect("script exists");
        let _parsed_script =
            ScriptParser::parse(Rule::Script, &script).expect("parse should succeed");
    }
}
