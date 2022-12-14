use std::io;
use std::io::Write;

#[derive(Debug)]
pub enum MetaCommandResult {
    METACOMMANDSUCCESS,
    METACOMMANDUNRECOGNIZEDCOMMAND,
}
#[derive(Debug)]
#[derive(PartialEq)]

pub enum StatementType {
    STATEMENTINSERT,
    STATEMENTSELECT,
    STATEMENUNINITIALIZED,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PrepareResult {
    PREPARESUCCESS,
    PREPAREUNRECOGNIZEDCOMMAND,
}
#[derive(Debug)]
pub struct Statement {
    pub statement_type: StatementType,
}

pub fn print_prompt() {
    print!("db> ");
    io::stdout().flush().unwrap();
}
pub fn get_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let trim_line = line.trim();

    trim_line.to_owned()
}

pub fn do_meta_command(command: String) -> MetaCommandResult {
    if command == ".exit" {
        return MetaCommandResult::METACOMMANDSUCCESS;
    } else {
        return MetaCommandResult::METACOMMANDUNRECOGNIZEDCOMMAND;
    }
}

pub fn prepare_statement(command: String) -> (Statement, PrepareResult) {
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

pub fn execute_statement(Statement { statement_type }: Statement) {
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