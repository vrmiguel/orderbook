# Orderbook's backend service

## Building

Run `nix-shell` to install the required dependencies (other than Cargo itself).

Then compile the server with `cargo build --release`.

## Running

By default, the server will be bound to `http://127.0.0.1:8080`.
This behavior is configurable through the `ADDRESS` and `PORT` environment variables, such as in:

```bash
ADDRESS='0.0.0.0' PORT=8000 cargo run --release
```

## Available endpoints

#### `POST /orders/bids`: create a new bid order

```bash
$ echo '{"quantity": 2, "price": 3500}' | xh POST http://127.0.0.1:8080/orders/bids 
HTTP/1.1 200 OK
Content-Length: 47
Content-Type: application/json

{
    "uuid": "c2e9ef83-46bf-4bf9-8cba-3fac6aeb04c7"
}
```

#### `POST /orders/asks`: create a new ask order

```bash
echo '{"quantity": 5, "price": 35}' | xh POST http://127.0.0.1:8080/orders/asks
HTTP/1.1 200 OK
Content-Length: 47
Content-Type: application/json

{
    "uuid": "e26bde4c-ca03-4328-82e4-d0c99c14bd1a"
}
```

#### `GET /orders`: list all inserted orders

```bash
$ xh GET http://127.0.0.1:8080/orders
HTTP/1.1 200 OK
Content-Length: 180
Content-Type: application/json

{
    "orders": [
        {
            "id": "e26bde4c-ca03-4328-82e4-d0c99c14bd1a",
            "quantity": 5,
            "price": 35,
            "side": "ask"
        },
        {
            "id": "c2e9ef83-46bf-4bf9-8cba-3fac6aeb04c7",
            "quantity": 2,
            "price": 3500,
            "side": "bid"
        }
    ]
}
```

#### `DELETE /orders`: delete a given order

```bash
 echo '{ "uuid": "6eed020a-1b64-4991-ad60-444015b15dfa" }' | xh DELETE http://127.0.0.1:8080/orders
HTTP/1.1 200 OK
Access-Control-Allow-Credentials: true
Content-Length: 0
Vary: Origin, Access-Control-Request-Method, Access-Control-Request-Headers
```