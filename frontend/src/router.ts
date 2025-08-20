import {
  createBrowserRouter
} from "react-router";
import App from "@/App";
import DocumentDetails from "@/pages/DocumentDetails";
import QueryDocument from "@/pages/QueryDocument";
import CreateDocument from "@/pages/CreateDocument";

export const loader = async () => {
  return;
};

const router = createBrowserRouter([
  {
    path: "/",
    Component: App,
    loader,
  },
  {
    path: "/create-certificate",
    Component: CreateDocument,
    loader,
  },
  {
    path: "/certificate-success",
    Component: DocumentDetails,
    loader,
  }, {
    path: "/query-document",
    Component: QueryDocument,
    loader,
  },
]);

export default router;