import App from "@/App";
import CreateDocument from "@/pages/CreateDocument";
import Dashboard from "@/pages/Dashboard";
import InstitutionsTable from "@/pages/Dashboard/InstitutionsTable";
import Statistics from "@/pages/Dashboard/Statistics";
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
        path: "document",
        children: [
          {
            index: true,
            path: "create",
            element: (
              <ProtectedRoute requiredRoles={["InstitutionMember"]}>
                <CreateDocument />
              </ProtectedRoute>
            ),
          },
          {
            path: "query",
            element: (
              <ProtectedRoute
                requiredRoles={[
                  "SuperAdmin",
                  "RegularUser",
                  "InstitutionMember",
                ]}
              >
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
                  <ProtectedRoute
                    requiredRoles={[
                      "SuperAdmin",
                      "RegularUser",
                      "InstitutionMember",
                    ]}
                  >
                    <DocumentDetails />
                  </ProtectedRoute>
                ),
              },
              {
                path: "analytics",
                element: (
                  <ProtectedRoute
                    requiredRoles={[
                      "SuperAdmin",
                      "RegularUser",
                      "InstitutionMember",
                    ]}
                  >
                    <DocumentAnalytics />
                  </ProtectedRoute>
                ),
              },
              {
                path: "spreadsheet",
                element: (
                  <ProtectedRoute
                    requiredRoles={[
                      "SuperAdmin",
                      "RegularUser",
                      "InstitutionMember",
                    ]}
                  >
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
      <ProtectedRoute requiredRoles={["SuperAdmin"]}>
        <Dashboard />
      </ProtectedRoute>
    ),
    children: [
      {
        index: true,
        path: "",
        Component: Statistics,
      },
      {
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
