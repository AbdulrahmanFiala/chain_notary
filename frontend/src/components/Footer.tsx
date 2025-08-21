import { Facebook, Github } from 'lucide-react';

const Footer = () => {
  return (
    <footer className="bg-white border-t border-gray-200">

      <div className="border-gray-200 flex flex-col sm:flex-row justify-between items-center max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <p className="text-gray-600 text-sm">
          © 2025 ChainNotary. All rights reserved.
        </p>
        <div className="flex space-x-4 mt-4 sm:mt-0">
          <a href="https://www.facebook.com/profile.php?id=61578384099198" className="text-gray-400 hover:text-gray-600">
            <Facebook className="w-5 h-5" />
          </a>
          <a href="https://github.com/AbdulrahmanFiala/chain_notary" className="text-gray-400 hover:text-gray-600">
            <Github className="w-5 h-5" />
          </a>
        </div>
      </div>
    </footer>
  );
};

export default Footer;