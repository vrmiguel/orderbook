import { useEffect, useState } from 'react';
import { getOrders, Order } from './api';

const OrderList: React.FC = () => {
    const [orders, setOrders] = useState<Order[]>([]);

    useEffect(() => {
        const fetchOrders = async () => {
            try {
                const fetchedOrders = await getOrders();
                setOrders(fetchedOrders);
            } catch (error) {
                console.error(error);
            }
        };

        fetchOrders();
    }, []);

    return (
        <div>
            <h2>Orders</h2>
            {orders.length === 0 ? (
                <p>No orders available.</p>
            ) : (
                <ul>
                    {orders.map((order) => (
                        <li key={order.id}>
                            Order ID: {order.id}, Quantity: {order.quantity}, Price: {order.price}, Side: {order.side}
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
};

export default OrderList;
