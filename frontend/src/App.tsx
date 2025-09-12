import Footer from '@/components/Footer';
import Header from '@/components/Header';
import { useEffect, type FC } from 'react';
import { Outlet } from 'react-router';
import { useAppDispatch } from './store/hooks';
import { initializeAuth } from './store/slices/authSlice';

const App: FC = () => {

  const dispatch = useAppDispatch();

  useEffect(() => {
    dispatch(initializeAuth());
  }, [dispatch]);

  return <>
    <Header />
    <Outlet />
    <Footer />
  </>;
}

export default App;