import React, { useState, ChangeEvent, FormEvent } from 'react';
import { OrderSide } from './api';
import './OrderForm.css';

interface OrderFormProps {
    createOrder: (quantity: number, price: number, orderSide: OrderSide) => Promise<string>;
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
        <form onSubmit={handleSubmit} className="form-container">
            <div className="form-row">
                <label>
                    Quantity:
                    <input type="number" value={quantity} onChange={handleQuantityChange} />
                </label>
            </div>
            <div className="form-row">
                <label>
                    Price:
                    <input type="number" value={price} onChange={handlePriceChange} />
                </label>
            </div>
            <div className="form-row">
                <label>
                    Order side:
                    <select value={side} onChange={handleSideChange}>
                        <option value="bid">Bid</option>
                        <option value="ask">Ask</option>
                    </select>
                </label>
            </div>
            <button type="submit">Create Order</button>
        </form>
    );
};

export default OrderForm;
