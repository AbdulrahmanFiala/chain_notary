import type { FC } from "react"

const Settings: FC = () => {
  return (
    <div className="bg-white p-6 rounded-lg shadow-sm">
      <h2 className="text-xl font-semibold text-gray-900 mb-4">Settings</h2>
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <div>
            <p className="font-medium text-gray-900">Two-Factor Authentication</p>
            <p className="text-sm text-gray-500">Enhance your account security.</p>
          </div>
          <button className="bg-blue-600 text-white font-medium py-2 px-4 rounded-lg hover:bg-blue-700 text-sm">Enable</button>
        </div>
        <div className="flex items-center justify-between pt-4 border-t border-gray-200">
          <div>
            <p className="font-medium text-gray-900">Change Password</p>
            <p className="text-sm text-gray-500">Last changed over a year ago.</p>
          </div>
          <button className="text-sm font-medium text-blue-600 hover:text-blue-700">Change</button>
        </div>
        <div className="flex items-center justify-between pt-4 border-t border-gray-200">
          <div>
            <p className="font-medium text-red-600">Delete Account</p>
            <p className="text-sm text-gray-500">Permanently delete your account and all data.</p>
          </div>
          <button className="text-sm font-medium text-red-600 hover:text-red-700">Delete</button>
        </div>
      </div>
    </div>

  )
}

export default Settings