import LoadingSpinner from "@/components/shared/LoadingSpinner";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { initializeAuth } from "@/store/slices/authSlice";
import { useEffect, useState, type FC, type ReactNode } from "react";
import { Navigate, useLocation, useNavigation } from "react-router";

const ProtectedRoute: FC<{
  children: ReactNode;
  requiredRoles?: ("RegularUser" | "SuperAdmin" | "InstitutionMember")[];
}> = ({ children, requiredRoles }) => {
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

  // Check for specific required role
  if (requiredRoles && requiredRoles.length > 0) {
    const userRoleKey = Object.keys(userProfile.role)[0] as
      | "RegularUser"
      | "SuperAdmin"
      | "InstitutionMember";
    if (!requiredRoles.includes(userRoleKey)) {
      const roleMessage = `Access restricted to access this page only. Contact your administrator for more information`;
      sessionStorage.setItem("messageApi", roleMessage);
      return <Navigate to="/" state={{ from: location }} replace />;
    }
  }

  return <>{children}</>;
};

export default ProtectedRoute;
