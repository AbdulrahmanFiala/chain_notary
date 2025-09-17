import App from "@/App";
import CreateDocument from "@/pages/CreateDocument";
import Dashboard from "@/pages/Dashboard";
import InstitutionsTable from "@/pages/Dashboard/InstitutionsTable";
import DocumentAnalytics from "@/pages/DocumentAnalytics";
import DocumentDetails from "@/pages/DocumentDetails";
import Home from "@/pages/Home";
import PageNotFound from "@/pages/PageNotFound";
import QueryDocument from "@/pages/QueryDocument";
import UserAccount from "@/pages/UserAccount";
import DocumentHistory from "@/pages/UserAccount/DocumentHistory";
import Profile from "@/pages/UserAccount/Profile";
import Settings from "@/pages/UserAccount/Settings";
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
      {
        path: "account",
        element: (
          <ProtectedRoute>
            <UserAccount />
          </ProtectedRoute>
        ),
        children: [
          {
            index: true,
            path: "profile",
            Component: Profile
          },
          {
            path: "settings",
            Component: Settings
          },
          {
            path: "document-history",
            Component: DocumentHistory,
          }
        ]
      }
    ]
  },
  {
    path: "dashboard",
    element: (
        <Dashboard />
    ),
    children: [
      {
        path: "institutions",
        Component: InstitutionsTable,
      }
    ],
  },
  {
    path: "*",
    Component: PageNotFound,
  }
]);

export default router;