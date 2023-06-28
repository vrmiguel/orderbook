#!/usr/bin/env bash

# Run 2000 concurrent workers each doing 500.000 requests
hey \
    -n 500000 \
    -c 2000   \
    -m POST \
    -H 'Content-Type: application/json' \
    -H 'Accept: application/json, */*;q=0.5' \
    -d '{"quantity": 5, "price": 35}' \
    http://127.0.0.1:8080/orders/asks