# p2p
# Soroban Messaging Contract

## Project Vision

The Soroban Messaging Contract is a decentralized solution for enabling secure communication within Soroban-based applications. It facilitates the exchange of messages between users on the Soroban blockchain, offering a reliable platform for decentralized messaging.

### Key Features:

- **Message Sending:** Users can send messages to specific addresses on the Soroban blockchain.
- **Message Retrieval:** Recipients can retrieve messages sent to their address.
- **Message Deletion:** Users can delete messages from their inbox.

## Getting Started

To integrate the Soroban Messaging Contract into your Soroban-based decentralized application, follow these steps:

1. **Installation:** Incorporate the Soroban SDK into your project and include the messaging contract implementation.

   ```rust
   use soroban_sdk::{contractimpl, Address, Env, BytesN, Vec, panic_with};
   
   pub const MESSAGE_LIMIT: u32 = 1024;  
   
   #[contractimpl]
   impl MessagingContract {
       
       // Implementation details...
   }
