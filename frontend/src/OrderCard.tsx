import React from "react";
import { Order } from "./api";
import "./OrderCard.css";

const OrderCard: React.FC<{ order: Order }> = ({ order }) => {
    const formatPrice = (price: number) => {
        const formattedPrice = (price / 100).toFixed(2);
        return `USD ${formattedPrice}`;
    };

    return (
        <div className="order-card">
            <div className="order-form-box">
                <h3>Order ID: {order.id}</h3>
            </div>
            <div className="order-form-box">
                <p>Quantity: {order.quantity} units</p>
                <p>Price: {formatPrice(order.price)}</p>
            </div>
            <div className="order-form-box">
                <p>Total value: {formatPrice(order.price * order.quantity)}</p>
            </div>
        </div>
    );
};


export default OrderCard;