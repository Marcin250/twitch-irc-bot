use std::env;
use std::io;
use redis::Connection;
use redis::RedisError;

extern crate redis;

pub struct ConsoleInput
{
}

impl ConsoleInput {
    pub fn arguments() -> Vec<String>
    {
        return env::args().collect();
    }

    pub fn ask_for_input(valueOf: String) -> String
    {
        let mut input = String::new();

        println!("Enter value of {}: ", valueOf);
    
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    
        return input.trim().to_string();
    }
}

pub struct Environment
{
}

impl Environment {
    pub fn get(key: String) -> String
    {
        return env::var(key.clone()).unwrap_or_else(|_| {
            panic!("Failed to read {:?} key from environment variables.", key);
        });
    }
}

pub struct RedisConnection
{
}

impl RedisConnection
{
    pub fn initialize() -> Connection
    {
        initializeRedis().unwrap()
    }
}

pub fn key(suffix: String) -> String
{
    Environment::get("APP_NAME".to_owned()) + ":" + &suffix
}

fn initializeRedis() -> Result<Connection, RedisError>
{
    let client = redis::Client::open(Environment::get("REDIS_CONNECTION_URI".to_owned()))?;
    let connection = client.get_connection()?;

    Ok(connection)
}