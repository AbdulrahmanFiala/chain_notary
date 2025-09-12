import App from "@/App";
import CreateDocument from "@/pages/CreateDocument";
import DocumentAnalytics from "@/pages/DocumentAnalytics";
import DocumentDetails from "@/pages/DocumentDetails";
import Home from "@/pages/Home";
import PageNotFound from "@/pages/PageNotFound";
import QueryDocument from "@/pages/QueryDocument";
import ProtectedRoute from "@/router/ProtectedRoute";
import {
  createBrowserRouter
} from "react-router";


const router = createBrowserRouter([
  {
    path: "/",
    Component: App,
    children: [
      {
        index: true,
        Component: Home
      },
      {
        path: "document",
        children: [
          {
            index: true,
            path: "create",
            element: <ProtectedRoute><CreateDocument /></ProtectedRoute>,
          },
          {
            path: "query",
            element: <ProtectedRoute><QueryDocument /></ProtectedRoute>,
          },
          {
            path: ":id",
            children: [
              {
                path: "view",
                element: <ProtectedRoute><DocumentDetails /></ProtectedRoute>,
              },
              {
                path: "analytics",
                element: <ProtectedRoute><DocumentAnalytics /></ProtectedRoute>,
              }
            ]
          },

        ]
      },
    ]
  },
  {
    path: "*",
    Component: PageNotFound,
  }
]);

export default router;