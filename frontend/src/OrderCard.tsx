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
            <h3>Order ID: {order.id}</h3>
            <p>Quantity: {order.quantity} units</p>
            <p>Price: {formatPrice(order.price)}</p>
        </div>
    );
};

export default OrderCard;