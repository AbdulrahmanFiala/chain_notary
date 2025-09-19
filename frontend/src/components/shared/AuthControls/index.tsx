import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { login, logout } from "@/store/slices/authSlice";
import { LogoutOutlined, UserOutlined } from "@ant-design/icons";
import { Avatar, Button, Dropdown, type MenuProps } from "antd";
import { NavLink } from "react-router";
import "./style.css";

// Reusable button component
const AuthControls = () => {
  const dispatch = useAppDispatch();
  const { isAuthenticated, loading } = useAppSelector((state) => state.auth);

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
          className="text-red-500 font-medium inline-block w-24"
          onClick={() => dispatch(logout())}
          itemType="button"
        >
          Logout
        </span>
      ),
      icon: <LogoutOutlined />,
    },
  ];

  return (
    <>
      {!isAuthenticated ? (
        <span className="flex gap-2">
          <Button
            disabled={loading}
            variant="text"
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
