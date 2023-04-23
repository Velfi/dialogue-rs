use crate::{
    error::Error,
    script::parser::{Rule, ScriptParser},
};
use pest::Parser;

const INDENT: &str = "    ";

pub fn pairs_to_string(script_str: &str) -> Result<String, Error> {
    let mut builder_string = String::new();
    let pairs = ScriptParser::parse(Rule::Script, script_str).map_err(Error::parse_error)?;

    for pair in pairs {
        builder_string.push_str(&format!("{}\n", pair.as_rule()));
        builder_string = pairs_to_string_with_indent(pair.into_inner(), builder_string, 1)?;
    }

    Ok(builder_string)
}

fn pairs_to_string_with_indent(
    pairs: pest::iterators::Pairs<Rule>,
    mut builder_string: String,
    with_indent: usize,
) -> Result<String, Error> {
    for pair in pairs {
        builder_string.push_str(&format!(
            "{}{}\n",
            INDENT.repeat(with_indent),
            pair.as_rule()
        ));
        builder_string =
            pairs_to_string_with_indent(pair.into_inner(), builder_string, with_indent + 1)?;
    }

    Ok(builder_string)
}

#[cfg(test)]
mod tests {
    use super::pairs_to_string;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_script_structure_to_string() {
        let script = std::fs::read_to_string("example_scripts/daisy-and-luigi.script")
            .expect("example script exists");
        let script_structure = pairs_to_string(&script).expect("script structure is parseable");

        let expected = "Script
    Start of script
        Start Marker
    Line
        SAY command
            SAY command (infix)
                subject
                text
    Line
        SAY command
            SAY command (infix)
                subject
                text
        Indent
            Line
                CHOICE command
                    text
                Indent
                    Line
                        SAY command
                            SAY command (infix)
                                subject
                                text
                    Line
                        GOTO command
                            Marker
                                marker name
            Line
                CHOICE command
                    text
                Indent
                    Line
                        SAY command
                            SAY command (infix)
                                subject
                                text
    Line
        SAY command
            SAY command (infix)
                subject
                text
    End of script
        End Marker
        End of Input
";

        assert_eq!(script_structure, expected);
    }

    //     #[test]
    //     fn test_script_structure_to_string_v2() {
    //         let script = std::fs::read_to_string("example_scripts/daisy-and-luigi.script")
    //             .expect("example script exists");
    //         let script_structure = pairs_to_string(&script).expect("script structure is parseable");

    //         let expected = "Script
    //     Start of script
    //         Start Marker
    //     Line
    //         SAY command
    //             SAY command (infix)
    //                 subject
    //                 text
    //     Line
    //         SAY command
    //             SAY command (infix)
    //                 subject
    //                 text
    //     Indent
    //         Line
    //             CHOICE command
    //                 text
    //                 text
    //         Indent
    //             Line
    //                 SAY command
    //                     SAY command (infix)
    //                         subject
    //             Line
    //                 GOTO command
    //                     Marker
    //                         marker name
    //         Line
    //             CHOICE command
    //                 text
    //         Indent
    //             Line
    //                 SAY command
    //                     SAY command (infix)
    //                         subject
    //                         text
    //     Line
    //         SAY command
    //             SAY command (infix)
    //                 subject
    //                 text
    //     End of script
    //         End Marker
    //         End of Input
    // ";

    //         assert_eq!(script_structure, expected);
    //     }
}
