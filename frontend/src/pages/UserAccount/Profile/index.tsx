import { useAppSelector } from "@/store/hooks";
import { Button, Tag } from "antd";
import { startCase } from "lodash";
import { type FC } from "react";

const Profile: FC = () => {
  // Get user profile data from API
  const { userProfile } = useAppSelector((state) => state.auth);

  if (!userProfile) {
    return <div>Loading...</div>;
  }

  return (
    <div className="space-y-8" id="profile">
      <div className="bg-white p-6 rounded-lg shadow-sm">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-semibold text-gray-900">
            Personal Information
          </h2>
          <Button
            variant="link"
            color="primary"
            className="text-sm font-medium"
          >
            Edit
          </Button>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label
              className="block text-sm font-medium text-gray-500"
              htmlFor="fullName"
            >
              Full Name
            </label>
            <p className="mt-1 text-gray-900">{userProfile.name}</p>
          </div>
          <div>
            <label
              className="block text-sm font-medium text-gray-500"
              htmlFor="email"
            >
              Email Address
            </label>
            <p className="mt-1 text-gray-900">{userProfile.email}</p>
          </div>
          <div>
            <label
              className="block text-sm font-medium text-gray-500"
              htmlFor="phone"
            >
              Principal ID
            </label>
            <p className="mt-1 text-gray-900">
              {userProfile.internet_identity.toString()}
            </p>
          </div>
          <div>
            <label
              className="block text-sm font-medium text-gray-500"
              htmlFor="address"
            >
              Role
            </label>

            <Tag
              className="mt-1!"
              color={
                Object.keys(userProfile.role)[0] === "SuperAdmin"
                  ? "red"
                  : Object.keys(userProfile.role)[0] === "RegularUser"
                    ? "blue"
                    : "gold"
              }
            >
              {startCase(Object.keys(userProfile.role)[0])}
            </Tag>
          </div>
          {userProfile.assigned_institution_id && (
            <div>
              <label
                className="block text-sm font-medium text-gray-500"
                htmlFor="city"
              >
                Assigned Institution
              </label>
              <p className="mt-1 text-gray-900">
                {userProfile.assigned_institution_id}
              </p>
            </div>
          )}
          <div>
            <label
              className="block text-sm font-medium text-gray-500"
              htmlFor="city"
            >
              Last login
            </label>
            <p className="mt-1 text-gray-900">
              {new Date(
                Number(userProfile.last_login) / 1_000_000,
              ).toLocaleDateString("en-GB", {
                day: "2-digit",
                month: "short",
                year: "numeric",
                hour: "2-digit",
                minute: "2-digit",
                second: "2-digit",
                hour12: true,
              })}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Profile;
