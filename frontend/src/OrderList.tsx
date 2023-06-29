import { useEffect, useState } from 'react';
import { getOrders, Order } from './api';
import OrderCard from './OrderCard';
import './OrderList.css';

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

    const bidOrders = orders.filter((order) => order.side.toLowerCase() === 'bid');
    const askOrders = orders.filter((order) => order.side.toLowerCase() === 'ask');

    return (
        <div>
            <h2>Orders</h2>
            {orders.length === 0 ? (
                <p>No orders available.</p>
            ) : (
                <div className="order-list-container">
                    <div className="order-list-column">
                        <h3>Bids</h3>
                        {bidOrders.length === 0 ? (
                            <p>No bid orders available.</p>
                        ) : (
                            bidOrders.map((order) => (
                                <OrderCard key={order.id} order={order} />
                            ))
                        )}
                    </div>
                    <div className="order-list-column">
                        <h3>Asks</h3>
                        {askOrders.length === 0 ? (
                            <p>No ask orders available.</p>
                        ) : (
                            askOrders.map((order) => (
                                <OrderCard key={order.id} order={order} />
                            ))
                        )}
                    </div>
                </div>
            )}
        </div>
    );
};


export default OrderList;

