import { useEffect } from "react";
import { createBrowserRouter, Outlet, RouterProvider } from "react-router-dom";
import { ToastContainer } from "react-toastify";
import "./App.css";
import { PrivateRoute } from "./components/common/PrivateRoute";
import { Refreshing } from "./components/common/Refreshing";
import { Home } from "./components/Home/Home";
import { Login } from "./components/login/Login";
import { PageNavbar } from "./components/Navbar";
import { Register } from "./components/register/Register";
const REFRESH_INTERVAL_MS = 30 * 1000;

function App() {
  useEffect(() => {
    Notification.requestPermission();
  }, []);

  const NavbarWrapper: React.FC = () => {
    return (
      <>
        <PageNavbar />
        <Outlet />
      </>
    );
  };
  const router = createBrowserRouter([
    {
      path: "/",
      element: <NavbarWrapper />,
      children: [
        {
          path: "/",
          element: (
            <PrivateRoute>
              <Refreshing interval={REFRESH_INTERVAL_MS}>
                <Home />
              </Refreshing>
            </PrivateRoute>
          ),
        },
        { path: "/register", element: <Register /> },
        { path: "/login", element: <Login /> },
      ],
    },
  ]);
  return (
    <div className="App">
      <ToastContainer />
      <RouterProvider router={router}></RouterProvider>
    </div>
  );
}

export default App;
