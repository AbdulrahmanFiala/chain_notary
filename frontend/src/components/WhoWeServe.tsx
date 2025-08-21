import { Award, Building2, Coins } from 'lucide-react';

const WhoWeServe = () => {
  const services = [
    {
      icon: Building2,
      title: 'Governments',
      description: 'Issue official documents and certificates with blockchain security'
    },
    {
      icon: Coins,
      title: 'Financial Institutions',
      description: 'Secure degree certificates and academic credentials'
    },
    {
      icon: Award,
      title: 'Licensing Authorities',
      description: 'Issue tamper-proof professional licenses and certifications'
    }
  ];

  return (
    <section className="py-20 bg-gray-50">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        <h2 className="text-4xl font-bold text-center text-gray-900 mb-16">
          Who We Serve
        </h2>

        <div className="grid md:grid-cols-3 gap-12">
          {services.map((service, index) => (
            <div key={index} className="text-center bg-white p-8 rounded-lg">
              <div className="w-16 h-16 bg-blue-600 rounded-full flex items-center justify-center mx-auto mb-6">
                <service.icon className="w-8 h-8 text-white" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-4">
                {service.title}
              </h3>
              <p className="text-gray-600">
                {service.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default WhoWeServe;