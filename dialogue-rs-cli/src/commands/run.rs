use anyhow::bail;
use dialogue_rs::{
    script::command::Command, Script, StateMachine, CHOICE_COMMAND, GOTO_COMMAND, IF_COMMAND,
    SAY_COMMAND, SET_COMMAND, TRIGGER_COMMAND,
};

pub(crate) fn run(script: &Script) -> Result<(), anyhow::Error> {
    let mut state_machine = StateMachine::builder().script(script).build()?;

    // Commands are OK to clone since they're just a references to short strings and we're displaying them
    // one-by-one.
    while let Some(command) = state_machine.next_command()? {
        match command.name() {
            SAY_COMMAND => say_command(&command)?,
            GOTO_COMMAND => state_machine.follow_goto()?,
            CHOICE_COMMAND => {
                // TODO For choice commands, we need to collect all the choices together before
                // displaying them and allowing the user to choose one. Then, we need to continue
                // from the block following the chosen command.
                choice_command(&mut state_machine)?;
            }
            SET_COMMAND => {
                todo!("Handle set command");
            }
            TRIGGER_COMMAND => {
                todo!("Handle trigger command");
            }
            IF_COMMAND => {
                todo!("Handle if command");
            }
            _ => println!("unhandled command: {:?}", command),
        }
    }

    Ok(())
}

fn choice_command(
    state_machine: &mut StateMachine<'_>,
) -> Result<(), anyhow::Error> {
    // TODO We need to collect all the choices together before displaying them and allowing the
    // user to choose one. Then, we need to continue from the block following the chosen command.
    // - Display all the choices
    // - Allow the user to choose one
    // - Continue from the block following the chosen command
    // - If there is no block following the chosen command, then choosing it should continue outside the current block.

    let mut choices = vec![
        state_machine.current_line_number(),
    ];

    while let Some(next) = state_machine.next_command()? {
        if next.name() == CHOICE_COMMAND {
            choices.push(state_machine.current_line_number());
        } else {
            break;
        }
    }

    let choices = choices
        .iter()
        .map(|line_number| {
            let command = state_machine
                .get_tle(*line_number)
                .unwrap()
                .as_line()
                .unwrap()
                .as_command()
                .unwrap();
            (*line_number, command.suffix().expect("all these commands are choices with suffixes"))
        })
        .collect::<Vec<_>>();

    println!("Choose:");
    for (_, (i, choice)) in choices.iter().enumerate() {
        println!("\t{i}: {choice}");
    }

    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if let Ok(i) = input.parse::<usize>() {
            if i < choices.len() {
                let (line_number, _command) = choices[i];
                state_machine.set_current_line_number(line_number);
                break;
            }
        }
        println!("Invalid choice. Try again.");
    }

    state_machine.enter_choice_block()?;

    Ok(())
}

fn say_command(command: &Command) -> Result<(), anyhow::Error> {
    match (command.prefix(), command.suffix()) {
        (Some(prefix), Some(suffix)) => println!("{prefix}:\t{suffix}"),
        (None, Some(suffix)) => println!("{suffix}"),
        _ => bail!("Invalid say command: {:?}", command),
    }
    Ok(())
}
