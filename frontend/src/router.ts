import App from "@/App";
import CreateDocument from "@/pages/CreateDocument";
import DocumentDetails from "@/pages/DocumentDetails";
import QueryDocument from "@/pages/QueryDocument";
import {
  createBrowserRouter
} from "react-router";

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
    path: "/create-document",
    Component: CreateDocument,
    loader,
  },
  {
    path: "/document-details",
    Component: DocumentDetails,
    loader,
  }, {
    path: "/query-document",
    Component: QueryDocument,
    loader,
  },
]);

export default router;