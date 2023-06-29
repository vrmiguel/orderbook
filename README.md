# orderbook

Example of an application with a backend in Rust (w/ `actix-web`) and a simplistic front-end in TypeScript with React.

## Front-end

See the corresponding [README](frontend/README.md);

## Back-end

### Available endpoints

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