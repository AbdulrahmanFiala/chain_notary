import { Button } from "antd"
import type { FC } from "react"

const Profile: FC = () => {
  return (
    <div className="space-y-8" id="profile">
      <div className="bg-white p-6 rounded-lg shadow-sm">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-xl font-semibold text-gray-900">Personal Information</h2>
          <Button variant="link" color="primary" className="text-sm font-medium">Edit</Button>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label className="block text-sm font-medium text-gray-500" htmlFor="fullName">Full Name</label>
            <p className="mt-1 text-gray-900">John Doe</p>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-500" htmlFor="email">Email Address</label>
            <p className="mt-1 text-gray-900">john.doe@email.com</p>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-500" htmlFor="phone">Phone Number</label>
            <p className="mt-1 text-gray-900">+1 (555) 123-4567</p>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-500" htmlFor="wallet">Wallet Address</label>
            <p className="mt-1 text-gray-900 truncate">0x1234...abcd</p>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Profile