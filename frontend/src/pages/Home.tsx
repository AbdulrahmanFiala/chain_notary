import Hero from "@/components/Hero";
import HowItWorks from "@/components/HowItWorks";
import WhoWeServe from "@/components/WhoWeServe";
import type { FC } from "react";

const Home: FC = () => {
  return (
    <div className="min-h-screen bg-white">
      <Hero />
      <HowItWorks />
      <WhoWeServe />
    </div>
  );
}

export default Home;