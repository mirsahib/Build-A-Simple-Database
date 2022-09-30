use rustdb::{
    MetaCommandResult,
    StatementType,
    PrepareResult,
    do_meta_command,
    execute_statement,
    get_line,
    prepare_statement,
    print_prompt};

fn main() {
    let mut buff: String;
    loop {
        print_prompt();
        buff = get_line();
        let first = buff.chars().next().unwrap();

        // handle meta command
        if first == '.' {
            let result:MetaCommandResult = do_meta_command(buff.to_owned());
            match result {
                MetaCommandResult::METACOMMANDSUCCESS => {
                    continue;
                }
                MetaCommandResult::METACOMMANDUNRECOGNIZEDCOMMAND => {
                    println!("Unknown meta command '{}'", buff);
                    continue;
                }
            }
        }

        let (statement, result) = prepare_statement(buff.to_owned());

        if statement.statement_type == StatementType::STATEMENUNINITIALIZED {
            match result {
                PrepareResult::PREPARESUCCESS => {
                    break;
                }
                PrepareResult::PREPAREUNRECOGNIZEDCOMMAND => {
                    println!("Unknown statement '{}'", buff);
                    continue;
                }
            }
        };
        execute_statement(statement);
        println!("executed");
    }
}
