import { useAppDispatch } from "@/store/hooks";
import { setMessageApi } from "@/store/slices/messageSlice";
import {
  BankOutlined,
  HomeOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  UserOutlined,
} from "@ant-design/icons";
import type { MenuProps } from "antd";
import { Menu, message } from "antd";
import React, { useEffect, useState } from "react";
import { NavLink, Outlet } from "react-router";

type MenuItem = Required<MenuProps>["items"][number];

const Dashboard: React.FC = () => {
  const [collapsed, setCollapsed] = useState(false);
  const [messageApi, contextHolder] = message.useMessage();
  
  const dispatch = useAppDispatch();

  useEffect(() => {
    dispatch(setMessageApi(messageApi));
  }, [dispatch, messageApi]);

  const toggleCollapsed = () => {
    setCollapsed(!collapsed);
  };

  const items: MenuItem[] = [
    {
      key: "0",
      onClick: toggleCollapsed,
      label: collapsed ? "Expand" : "Collapse",

      icon: collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />,
    },
    { key: "1", icon: <HomeOutlined />, label: <NavLink to="/">Home</NavLink> },
    {
      key: "2",
      icon: <BankOutlined />,
      label: <NavLink to="/dashboard/institutions">Institutions</NavLink>,
    },
    {
      key: "3",
      icon: <UserOutlined />,
      label: <NavLink to="/dashboard/users">Users</NavLink>,
    },
  ];
  return (
    <div
      className="h-screen overflow-hidden flex w-full!"
      style={{ width: 256 }}
    >
      {contextHolder}
      <Menu
        className="max-w-2/12"
        defaultSelectedKeys={["1"]}
        defaultOpenKeys={["sub1"]}
        mode="inline"
        theme="dark"
        inlineCollapsed={collapsed}
        items={items}
      />
      <main className="w-full py-5 px-4 overflow-scroll">
        <Outlet />
      </main>
    </div>
  );
};

export default Dashboard;
