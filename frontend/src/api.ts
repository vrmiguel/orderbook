import axios, { AxiosResponse } from "axios";

export type OrderCreationResponse = {
    uuid: string;
}

export type Order = {
    id: string;
    quantity: number;
    price: number;
    side: OrderSide;
}

// The base address of our backend service
const baseUrl = process.env.BASE_URL || 'http://127.0.0.1:8080/';

export type OrderSide = 'bid' | 'ask';

const api = axios.create({
    baseURL: baseUrl,
});

// Request interceptor for extra-logging
api.interceptors.request.use(
    (config) => {
        console.log('Request:', config.method?.toUpperCase(), config.url);
        console.log('Request Data:', config.data);
        return config;
    },
    (error) => {
        console.error('Request Error:', error);
        return Promise.reject(error);
    }
);

export async function createOrder(quantity: number, price: number, orderSide: OrderSide): Promise<string> {
    const endpoint = orderSide === 'bid' ? '/orders/bids' : '/orders/asks';
    const requestBody = { quantity, price };

    try {
        const response = await api.post<OrderCreationResponse>(endpoint, requestBody);
        return response.data.uuid;
    } catch (error) {
        throw new Error(`Failed to create ${orderSide.toLowerCase()} order: ${error}.`);
    }
}

export async function getOrders(): Promise<Order[]> {
    try {
        const response: AxiosResponse<{ orders: Order[] }> = await api.get('/orders');
        return response.data.orders;
    } catch (error) {
        throw new Error(`Failed to fetch orders: ${error}.`);
    }
}
