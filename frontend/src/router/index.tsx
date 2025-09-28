import App from "@/App";
import CreateDocument from "@/pages/CreateDocument";
import Dashboard from "@/pages/Dashboard";
import InstitutionsTable from "@/pages/Dashboard/InstitutionsTable";
import UsersTable from "@/pages/Dashboard/UsersTable";
import DocumentAnalytics from "@/pages/DocumentAnalytics";
import DocumentDetails from "@/pages/DocumentDetails";
import Home from "@/pages/Home";
import PageNotFound from "@/pages/PageNotFound";
import QueryDocument from "@/pages/QueryDocument";
import UserAccount from "@/pages/UserAccount";
import DocumentHistory from "@/pages/UserAccount/DocumentHistory";
import Profile from "@/pages/UserAccount/Profile";
import Settings from "@/pages/UserAccount/Settings";
import UserRegistration from "@/pages/UserRegistration";
import XBRLViewer from "@/pages/XBRLViewer";
import ProtectedRoute from "@/router/ProtectedRoute";
import { createBrowserRouter } from "react-router";

const router = createBrowserRouter([
  {
    path: "/",
    Component: App,
    children: [
      {
        index: true,
        Component: Home,
      },
      {
        path: "register",
        Component: UserRegistration,
      },
      {
        path: "xbrl-viewer",
        Component: XBRLViewer,
      },
      {
        path: "xbrl-viewer/:id",
        Component: XBRLViewer,
      },
      {
        path: "document",
        children: [
          {
            index: true,
            path: "create",
            element: (
              <ProtectedRoute>
                <CreateDocument />
              </ProtectedRoute>
            ),
          },
          {
            path: "query",
            element: (
              <ProtectedRoute>
                <QueryDocument />
              </ProtectedRoute>
            ),
          },
          {
            path: ":id",
            children: [
              {
                path: "view",
                element: (
                  <ProtectedRoute>
                    <DocumentDetails />
                  </ProtectedRoute>
                ),
              },
              {
                path: "analytics",
                element: (
                  <ProtectedRoute>
                    <DocumentAnalytics />
                  </ProtectedRoute>
                ),
              },
              {
                path: "spreadsheet",
                element: (
                  <ProtectedRoute>
                    <XBRLViewer />
                  </ProtectedRoute>
                ),
              },
            ],
          },
        ],
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
            Component: Profile,
          },
          {
            path: "settings",
            Component: Settings,
          },
          {
            path: "document-history",
            Component: DocumentHistory,
          },
        ],
      },
    ],
  },
  {
    path: "dashboard",
    element: (
      <ProtectedRoute admin>
        <Dashboard />
      </ProtectedRoute>
    ),
    children: [
      {
        index: true,
        path: "users",
        Component: UsersTable,
      },
      {
        path: "institutions",
        Component: InstitutionsTable,
      },
    ],
  },
  {
    path: "*",
    Component: PageNotFound,
  },
]);

export default router;
