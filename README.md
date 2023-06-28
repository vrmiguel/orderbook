# orderbook


## Back-end

### Available endpoints

##### `GET /orders`

#### `POST /orders/bids`: create a new bid order

```bash
$ echo '{"quantity": 2, "price": 3500}' | xh POST http://127.0.0.1:8080/orders/bids 
HTTP/1.1 200 OK
Content-Length: 47
Content-Type: application/json
Date: Wed, 28 Jun 2023 21:55:50 GMT

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
Date: Wed, 28 Jun 2023 21:57:47 GMT

{
    "uuid": "e26bde4c-ca03-4328-82e4-d0c99c14bd1a"
}
```