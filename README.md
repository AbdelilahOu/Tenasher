# Tenasher

## sea-orm migrations example cli

### how to use

I have defined my custome commands in the src/main.rs you can edit as you wish

- Apply all pending migrations
  ```sh
  cargo run up
  ```
- Rollback all applied migrations
  ```sh
  cargo run down
  ```
- Drop all tables from the database, then reapply all migrations
  ```sh
  cargo run fresh
  ```
