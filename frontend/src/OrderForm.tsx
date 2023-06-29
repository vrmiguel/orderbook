import React, { useState, ChangeEvent, FormEvent } from 'react';
import { OrderSide } from './api';

interface OrderFormProps {
    createOrder: (quantity: number, price: number, orderSide: OrderSide) => Promise<string>;
}

interface Order {
    quantity: number;
    price: number;
    side: 'bid' | 'ask';
}

const OrderForm: React.FC<OrderFormProps> = ({ createOrder }) => {
    const [quantity, setQuantity] = useState('');
    const [price, setPrice] = useState('');
    const [side, setSide] = useState<'bid' | 'ask'>('bid');

    const handleSubmit = async (e: FormEvent) => {
        e.preventDefault();

        await createOrder(Number(quantity), Number(price), side);
        setQuantity('');
        setPrice('');
    };

    const handleQuantityChange = (e: ChangeEvent<HTMLInputElement>) => {
        setQuantity(e.target.value);
    };

    const handlePriceChange = (e: ChangeEvent<HTMLInputElement>) => {
        setPrice(e.target.value);
    };

    const handleSideChange = (e: ChangeEvent<HTMLSelectElement>) => {
        setSide(e.target.value as 'bid' | 'ask');
    };

    return (
        <form onSubmit={handleSubmit}>
            <label>
                Quantity:
                <input type="number" value={quantity} onChange={handleQuantityChange} />
            </label>
            <br />
            <label>
                Price:
                <input type="number" value={price} onChange={handlePriceChange} />
            </label>
            <br />
            <label>
                Side:
                <select value={side} onChange={handleSideChange}>
                    <option value="bid">Bid</option>
                    <option value="ask">Ask</option>
                </select>
            </label>
            <br />
            <button type="submit">Create Order</button>
        </form>
    );
};

export default OrderForm;