use std::io;
use std::io::Write;

#[derive(Debug)]
enum MetaCommandResult {
    METACOMMANDSUCCESS,
    METACOMMANDUNRECOGNIZEDCOMMAND,
}
#[derive(Debug)]
#[derive(PartialEq)]

enum StatementType {
    STATEMENTINSERT,
    STATEMENTSELECT,
    STATEMENUNINITIALIZED,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum PrepareResult {
    PREPARESUCCESS,
    PREPAREUNRECOGNIZEDCOMMAND,
}
#[derive(Debug)]
struct Statement {
    statement_type: StatementType,
}

fn main() {
    let mut buff: String;
    loop {
        print_prompt();
        buff = get_line();
        let first = buff.chars().next().unwrap();

        // handle meta command
        if first == '.' {
            let result: MetaCommandResult = do_meta_command(buff.to_owned());
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
        // handle statement

        let (statement, result) = prepare_statement(buff.to_owned());
        
        if statement.statement_type == StatementType::STATEMENUNINITIALIZED{
            match result {
                PrepareResult::PREPARESUCCESS => {
                    break;
                },
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
fn print_prompt() {
    print!("db> ");
    io::stdout().flush().unwrap();
}
fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let trim_line = line.trim();

    trim_line.to_owned()
}

fn do_meta_command(command: String) -> MetaCommandResult {
    if command == ".exit" {
        return MetaCommandResult::METACOMMANDSUCCESS;
    } else {
        return MetaCommandResult::METACOMMANDUNRECOGNIZEDCOMMAND;
    }
}

fn prepare_statement(command: String) -> (Statement, PrepareResult) {
    let mut statement: Statement = Statement {
        statement_type: StatementType::STATEMENUNINITIALIZED,
    };
    if command.len() > 6 {
        if &command[..6] == "select" || &command[..6] == "SELECT" {
            statement.statement_type = StatementType::STATEMENTSELECT;
            return (statement, PrepareResult::PREPARESUCCESS);
        }
        if &command[..6] == "insert" || &command[..6] == "INSERT" {
            statement.statement_type = StatementType::STATEMENTINSERT;
            return (statement, PrepareResult::PREPARESUCCESS);
        }
    }

    return (statement, PrepareResult::PREPAREUNRECOGNIZEDCOMMAND);
}

fn execute_statement(Statement { statement_type }: Statement) {
    match statement_type {
        StatementType::STATEMENTSELECT => {
            println!("This is where we would select a statement")
        }
        StatementType::STATEMENTINSERT => {
            println!("This is where we would insert a statement")
        }
        StatementType::STATEMENUNINITIALIZED => {
            println!("This is where we would initialize a state machine")
        }
    }
}
