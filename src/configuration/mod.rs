use crate::utils::Environment;

pub struct Configuration
{
    accounts: Vec<Account>,
}

impl Configuration {
    pub fn make() -> Configuration
    {
        let value = Environment::get("TWITCH_IRC_ACCOUNTS".to_string());
        let accounts = value.split(",");
        let mut collection = Vec::new();

        for account in accounts {
            if let Some((user, token)) = account.split_once(':') {
                collection.push(Account::new(user.to_string(), token.to_string()));
            }
        }

        return Configuration { accounts: collection };
    }

    pub fn accounts(self) -> Vec<Account>
    {
        return self.accounts;
    }
}

#[derive(Clone)]
pub struct Account
{
    user: String,
    token: String,
}

impl Account {
    pub fn new(user: String, token: String) -> Account
    {
        return Account { user, token };
    }

    pub fn data(&self) -> (String, String)
    {
        return (self.user.clone().to_string(), self.token.clone().to_string());
    }
}

pub fn loadConfiguration() -> Configuration {
    return Configuration::make();
}