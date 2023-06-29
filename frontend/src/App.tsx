import React from 'react';
import OrderForm from './OrderForm';
import { createOrder } from './api';
import './App.css';
import OrderList from './OrderList';

function App() {
  return (
    <div>
      <OrderForm createOrder={createOrder} />
      <OrderList />
    </div>
  );
}

export default App;
