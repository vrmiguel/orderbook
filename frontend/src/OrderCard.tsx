import React from "react";
import { Order } from "./api";
import "./OrderCard.css";

interface OrderCardProps {
    order: Order;
    onDeleteOrder: (orderId: string) => void;
}

const OrderCard: React.FC<OrderCardProps> = ({ order, onDeleteOrder }) => {
    const handleDeleteOrder = () => {
        onDeleteOrder(order.id);
    };

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
            <button onClick={handleDeleteOrder}>Cancel</button>
        </div>
    );
};


export default OrderCard;