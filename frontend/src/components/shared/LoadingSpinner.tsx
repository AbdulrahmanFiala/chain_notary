import React from 'react';
import './style.css';

const LoadingSpinner: React.FC = () => {
  return (
    <div className='loader-container'>
      <svg viewBox="25 25 50 50" className='loader'>
        <circle r="20" cy="50" cx="50"></circle>
      </svg>
    </div>
  )
}

export default LoadingSpinner;