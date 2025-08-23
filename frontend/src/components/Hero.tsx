import { NavLink } from "react-router";

const Hero = () => {
  return (
    <section className="bg-gray-50 py-20">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <h1 className="text-5xl font-bold text-gray-900 mb-6">
          Trust in Every Document
        </h1>
        <p className="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
          Blockchain notarization for governments and institutions. Instantly
          issue tamper-proof digital certificates.
        </p>
        <div className="flex justify-center items-center space-x-4">
          <NavLink
            to="/create-document"
            className="bg-blue-600 text-white px-6 py-3 rounded-lg shadow hover:bg-blue-700 transition duration-300"
          >
            Publish Earning release
          </NavLink>
          <NavLink
            to="/query-document"
            className="bg-gray-200 text-gray-800 px-6 py-3 rounded-lg shadow hover:bg-gray-300 transition duration-300"
          >
            Query Earning release
          </NavLink>
        </div>
      </div>
    </section>
  );
};

export default Hero;