use lazynote_common::commands::{
    body::{JsonBody, RawBody},
    reg_command, reg_command_async,
    result::{CommandResValue, CommandResult},
};

pub fn hello_command_json(JsonBody(args): JsonBody<Vec<String>>) -> CommandResult {
    println!("[hello_command_json]");
    CommandResValue::json(args)
}

pub fn hello_command_raw<'a>(RawBody(args): RawBody) -> CommandResult {
    println!("[hello_command_raw]");
    CommandResValue::raw(args.clone())
}

pub fn hello_command_void<'a>() -> CommandResult {
    println!("[hello_command_void]");
    CommandResValue::ok()
}

pub async fn hello_command_json_async(JsonBody(args): JsonBody<Vec<String>>) -> CommandResult {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("[hello_command_json_async]");
    CommandResValue::json(args)
}

pub async fn hello_command_raw_async(RawBody(args): RawBody<'_>) -> CommandResult {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("[hello_command_raw_async]");
    CommandResValue::raw(args.clone())
}

pub async fn hello_command_void_async() -> CommandResult {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    println!("[hello_command_void_async]");
    CommandResValue::ok()
}

pub fn reg_commands() {
    reg_command("hello_command_json", hello_command_json);
    reg_command("hello_command_raw", hello_command_raw);
    reg_command("hello_command_void", hello_command_void);
    reg_command_async("hello_command_json_async", hello_command_json_async);
    reg_command_async("hello_command_raw_async", hello_command_raw_async);
    reg_command_async("hello_command_void_async", hello_command_void_async);
}
