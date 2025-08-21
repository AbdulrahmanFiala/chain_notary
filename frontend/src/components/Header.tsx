import { NavLink } from 'react-router';

const Header = () => {
  return (
    <header className="bg-white shadow-sm border-b border-gray-100">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          <div className="flex-shrink-0 flex justify-between w-full">
            <NavLink to="/" className="text-xl font-bold text-gray-900">ChainNotary</NavLink>
            <nav>
              <ul className="flex space-x-4">
                <li>
                  <NavLink to="/create-document" className="text-gray-50 bg-blue-600 px-4 py-2 rounded hover:text-gray-200">Publish Document</NavLink>
                </li>
                <li>
                  <NavLink to="/query-document" className="bg-gray-200 text-gray-800 px-4 py-2 rounded hover:text-blue-600">Query Doument</NavLink>
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