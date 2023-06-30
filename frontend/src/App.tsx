import React from 'react';
import OrderForm from './OrderForm';
import { createOrder } from './api';
import './App.css';
import OrderList from './OrderList';

function App() {
  return (
    <div className="App">
      <OrderForm createOrder={createOrder} />
      <OrderList />
    </div>
  );
}

export default App;
