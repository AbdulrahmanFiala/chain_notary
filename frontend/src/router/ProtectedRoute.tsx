import LoadingSpinner from "@/components/shared/LoadingSpinner";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { initializeAuth } from "@/store/slices/authSlice";
import { useEffect, useState, type FC, type ReactNode } from "react";
import { Navigate, useLocation, useNavigation } from "react-router";

const ProtectedRoute: FC<{ children: ReactNode; admin?: boolean }> = ({
  children,
  admin = false,
}) => {
  const location = useLocation();
  const navigation = useNavigation();
  const { messageApi } = useAppSelector((state) => state.message);
  const [isChecking, setIsChecking] = useState(true);
  const { isAuthenticated, userProfile } = useAppSelector(
    (state) => state.auth,
  );

  const dispatch = useAppDispatch();

  useEffect(() => {
    const checkAuth = async () => {
      await dispatch(initializeAuth());

      setIsChecking(false);
    };
    checkAuth();
  }, [dispatch, messageApi]);

  if (navigation.state === "loading" || isChecking) return <LoadingSpinner />;

  if (!isAuthenticated) {
    messageApi?.warning("Please login to access this page", 10);
    return <Navigate to="/" state={{ from: location }} replace />;
  }

  if (!(userProfile?.name && userProfile.email)) {
    messageApi?.info("Please complete your profile to access all features", 10);
    return <Navigate to="/register" state={{ from: location }} replace />;
  }

  if (
    admin &&
    userProfile &&
    Object.keys(userProfile.role)[0] !== "SuperAdmin"
  ) {
    sessionStorage.setItem(
      "messageApi",
      "You are not authorized to access this page. Contact your administrator for more information",
    );

    return <Navigate to="/" state={{ from: location }} replace />;
  }

  return <>{children}</>;
};

export default ProtectedRoute;
