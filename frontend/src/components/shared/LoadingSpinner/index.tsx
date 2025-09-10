import { LoadingOutlined } from '@ant-design/icons';
import { Spin, type SpinProps } from 'antd';
import { type FC } from 'react';
import './style.css';

const LoadingSpinner: FC<SpinProps> = (props) => {
  return (
    <Spin rootClassName='bg-white!' fullscreen spinning indicator={<LoadingOutlined spin />} size="large" {...props} />
  )
}

export default LoadingSpinner;