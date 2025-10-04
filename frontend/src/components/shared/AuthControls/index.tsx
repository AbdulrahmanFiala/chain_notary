import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { login, logout } from "@/store/slices/authSlice";
import {
  DashboardOutlined,
  LogoutOutlined,
  UserOutlined,
} from "@ant-design/icons";
import { Avatar, Button, Dropdown, type MenuProps } from "antd";
import { NavLink, useNavigate } from "react-router";
import "./style.css";

// Reusable button component
const AuthControls = () => {
  const dispatch = useAppDispatch();
  const { isAuthenticated, loading, userProfile } = useAppSelector(
    (state) => state.auth,
  );
  const { messageApi } = useAppSelector((state) => state.message);
  const navigate = useNavigate();

  const items: MenuProps["items"] = [
    {
      key: "1",
      label: (
        <NavLink className="inline-block w-24" to="/account/profile">
          Account
        </NavLink>
      ),
      icon: <UserOutlined />,
    },
    {
      key: "2",
      label: (
        <span
          className="font-medium inline-block w-24"
          onClick={() => {
            dispatch(logout());
            navigate("/");
            messageApi?.success("Logout successful", 2);
          }}
          itemType="button"
        >
          Logout
        </span>
      ),
      danger: true,
      icon: <LogoutOutlined />,
    },
  ];

  if (userProfile && Object.keys(userProfile.role)[0] === "SuperAdmin")
    items.unshift({
      key: "0",
      label: (
        <NavLink className="inline-block w-24" to="/dashboard">
          Dashboard
        </NavLink>
      ),
      icon: <DashboardOutlined />,
    });
  return (
    <>
      {!isAuthenticated ? (
        <span className="flex gap-2">
          <Button
            disabled={loading}
            variant="solid"
            color="primary"
            onClick={() => dispatch(login())}
          >
            Join Now
          </Button>
        </span>
      ) : (
        <Dropdown
          disabled={loading}
          menu={{ items, selectable: true }}
          placement="bottom"
          trigger={["click"]}
        >
          <Avatar className="cursor-pointer" icon={<UserOutlined />} />
        </Dropdown>
      )}
    </>
  );
};

export default AuthControls;
