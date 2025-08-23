import { FileText, Share2, Shield } from 'lucide-react';

const HowItWorks = () => {
  const steps = [
    {
      icon: FileText,
      title: 'Upload Document',
      description: 'Upload or scan the official document'
    },
    {
      icon: Shield,
      title: 'Secure Minting',
      description: 'Mint a secure NFT with issuer, timestamp, and document type'
    },
    {
      icon: Share2,
      title: 'Share & Verify',
      description: 'Share or verify the document on-chain'
    }
  ];

  return (
    <section className="py-20 bg-white">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        <h2 className="text-4xl font-bold text-center text-gray-900 mb-16">
          How It Works
        </h2>

        <div className="grid md:grid-cols-3 gap-12">
          {steps.map((step, index) => (
            <div key={index} className="text-center bg-gray-50 p-8 rounded-lg">
              <div className="w-16 h-16 bg-blue-600 rounded-full flex items-center justify-center mx-auto mb-6">
                <step.icon className="w-8 h-8 text-white" />
              </div>
              <h3 className="text-xl font-semibold text-gray-900 mb-4">
                {step.title}
              </h3>
              <p className="text-gray-600">
                {step.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default HowItWorks;