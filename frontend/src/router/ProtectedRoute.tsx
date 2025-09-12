import LoadingSpinner from '@/components/shared/LoadingSpinner';
import { useAppSelector } from '@/store/hooks';
import { useEffect, useState, type FC, type ReactNode } from 'react';
import { Navigate, useLocation, useNavigation } from 'react-router';

const ProtectedRoute: FC<{ children: ReactNode }> = ({ children }) => {

  const location = useLocation();
  const navigation = useNavigation();
  const [isChecking, setIsChecking] = useState(true)
  const { isAuthenticated, loading } = useAppSelector(state => state.auth);

  useEffect(() => {
    const checkuth = async () => {
      let checkCount = 0;
      const internal: NodeJS.Timeout = await setInterval(() => {
        if (!loading && checkCount < 5) {
          checkCount += 1;
          if (isAuthenticated) {
            setIsChecking(false)
            clearInterval(internal);
          }
        } else {
          setIsChecking(false)
          clearInterval(internal);
        }
      }, 1000);
    };
    checkuth();
  }, [loading, isAuthenticated]);

  if (navigation.state === "loading" || isChecking) return <LoadingSpinner />

  if (!isAuthenticated) return <Navigate to="/" state={{ from: location }} replace />;

  return <>{children}</>;
};

export default ProtectedRoute;