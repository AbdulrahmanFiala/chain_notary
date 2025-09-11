import AuthControls from '@/components/shared/AuthControls';
import type { FC } from 'react';
import { NavLink } from 'react-router';

const Header: FC = () => {
  return (
    <header className="bg-white shadow-sm border-b border-gray-100">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          <div className="flex-shrink-0 flex justify-between w-full">
            <NavLink to="/" className="text-xl font-semibold! text-gray-900">ChainNotary</NavLink>
            <nav>
              <ul className="flex space-x-4">
                <li>
                  <AuthControls />
                </li>
              </ul>
            </nav>
          </div>
        </div>
      </div>
    </header>
  );
};

export default Header;