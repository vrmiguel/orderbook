import React from "react";
import { Order } from "./api";
import "./OrderCard.css";

const OrderCard: React.FC<{ order: Order }> = ({ order }) => {
    return (
        <div className="order-card">
            <h3>Order ID: {order.id}</h3>
            <p>Quantity: {order.quantity}</p>
            <p>Price: {order.price}</p>
        </div>
    );
};

export default OrderCard;