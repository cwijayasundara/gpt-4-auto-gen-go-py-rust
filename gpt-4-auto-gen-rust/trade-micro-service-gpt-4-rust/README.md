cargo new trade_microservice
cd trade_microservice

Create a table in Postgres

CREATE TABLE trades (
id SERIAL PRIMARY KEY,
value NUMERIC NOT NULL,
date TIMESTAMP NOT NULL,
trader VARCHAR(255) NOT NULL);


To recap, this updated `src/handlers.rs` file contains implementations of the following CRUD handlers:

1. `create_trade`: Inserts a new trade into the `trades` table.
2. `get_trades`: Retrieves all trades from the `trades` table.
3. `get_trade_by_id`: Retrieves a specific trade by its ID from the `trades` table.
4. `update_trade`: Updates a specific trade in the `trades` table.
5. `delete_trade`: Deletes a specific trade from the `trades` table.

