import React from 'react';
import OrderForm from './OrderForm';
import { createOrder } from './api';
import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div>
      <OrderForm createOrder={createOrder} />
    </div>
  );
}

export default App;
