use dialogue_rs::{check_syntax_with_options, Script, SyntaxCheckerOptions};

pub(crate) fn check(script: &Script) -> Result<(), anyhow::Error> {
    // TODO make this configurable
    let syntax_checker_options = SyntaxCheckerOptions::default();
    check_syntax_with_options(&script, &syntax_checker_options)?;

    eprintln!("Congratulations: Your script is valid.");

    Ok(())
}
