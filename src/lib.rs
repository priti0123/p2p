use soroban_sdk::{contractimpl, Address, Env, BytesN, Vec, panic_with};

pub const MESSAGE_LIMIT: u32 = 1024;  

#[contractimpl]
impl MessagingContract {
    
    pub fn send_message(env: Env, recipient: Address, message: BytesN<MESSAGE_LIMIT>) {
        let sender = env.invoker(); 
        if message.len() == 0 {
            panic_with(env, "Empty message not allowed");
        }

        let mut inbox = env
            .storage()
            .persistent()
            .get::<Address, Vec<BytesN<MESSAGE_LIMIT>>>(&recipient)
            .unwrap_or_default();
        inbox.push(message);

        env.storage().persistent().set(&recipient, &inbox);
    }

    
    pub fn get_messages(env: Env) -> Vec<BytesN<MESSAGE_LIMIT>> {
        let recipient = env.invoker();
        let inbox = env
            .storage()
            .persistent()
            .get::<Address, Vec<BytesN<MESSAGE_LIMIT>>>(&recipient)
            .unwrap_or_default();
        inbox
    }
    pub fn delete_messages(env: Env) {
        let recipient = env.invoker();
        env.storage().persistent().remove(&recipient);
    }
}
