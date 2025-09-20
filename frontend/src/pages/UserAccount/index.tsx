import { useAppSelector } from "@/store/hooks";
import { UserOutlined } from "@ant-design/icons";
import { Avatar } from "antd";
import type { FC } from "react";
import { NavLink, Outlet } from "react-router";

const UserAccount: FC = () => {
  const { userProfile } = useAppSelector((state) => state.auth);

  return (
    <main className="container mx-auto px-4 py-8">
      <div className="flex items-center mb-8">
        <Avatar
          className="cursor-pointer"
          size={96}
          alt="User avatar large"
          icon={<UserOutlined />}
        />
        <div className="ml-6">
          <h1 className="text-3xl font-bold text-gray-900">
            {userProfile?.name}
          </h1>
          <p className="text-gray-500">{userProfile?.email}</p>
        </div>
      </div>
      <div className="mb-8">
        <div className="border-b border-gray-200">
          <nav aria-label="Tabs" className="-mb-px flex space-x-8">
            <NavLink
              className={({ isActive }: { isActive: boolean }) =>
                (isActive
                  ? "border-blue-600 text-blue-600"
                  : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300") +
                " whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm"
              }
              to="/account/profile"
            >
              Profile
            </NavLink>
            <NavLink
              className={({ isActive }: { isActive: boolean }) =>
                (isActive
                  ? "border-blue-600 text-blue-600"
                  : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300") +
                " whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm"
              }
              to="/account/document-history"
            >
              Document History
            </NavLink>
            <NavLink
              className={({ isActive }: { isActive: boolean }) =>
                (isActive
                  ? "border-blue-600 text-blue-600"
                  : "border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300") +
                " whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm"
              }
              to="/account/settings"
            >
              Settings
            </NavLink>
          </nav>
        </div>
      </div>
      <div className="space-y-8" id="profile">
        <Outlet />
      </div>
    </main>
  );
};

export default UserAccount;
