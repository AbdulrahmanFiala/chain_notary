import CertificateSuccess from "#pages/CertificateSuccess.tsx";
import CreateCertificate from "#pages/CreateCertificate.tsx";
import {
  createBrowserRouter
} from "react-router";
import App from "./App";

const simulateApiFetch = async () => {
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve({ message: "Data fetched from API" });
    }, 5000); // simulate 5 second delay
  });
};

export const loader = async () => {
  return;
};

export const router = createBrowserRouter([
  {
    path: "/",
    Component: App,
    loader,
  },
  {
    path: "/create-certificate",
    Component: CreateCertificate,
    loader,
  },
  {
    path: "/certificate-success",
    Component: CertificateSuccess,
    loader,
  }
]);
