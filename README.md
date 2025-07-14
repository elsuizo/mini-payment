# mini-payment

for running the server:

```bash
cargo r --release
```

 - default `host`: `127.0.0.1`
 - default `port`: `8000`

available endpoints:

 - `POST` `/new_client`
   - imput:
    ```json
    {"client_name":"String","birth_date":"String","document_number":"String","country":"String"}
    ```
 - `POST` `/new_credit_transaction`
   - imput:
    ```json
    {"client_id":"uuid","credit_amount":"decimal"}
    ```
 - `POST` `/new_debit_transaction`
   - imput:
    ```json
    {"client_id":"uuid","debit_amount":"decimal"}
    ```
 - `POST` `/store_balances`
   - input: no input
 - `GET`  `/client_balance`
   - imput:
    ```bash
    path/client_balance?user_id=uuid
    ```
