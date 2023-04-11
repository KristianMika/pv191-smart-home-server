import { useEffect } from "react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { ToastContainer } from "react-toastify";
import "./App.css";
import { Refreshing } from "./components/common/Refreshing";
import { Home } from "./components/Home/Home";
import { PageNavbar } from "./components/Navbar";
import { Register } from "./components/register/Register";
const REFRESH_INTERVAL_MS = 60 * 1000;

function App() {
  useEffect(() => {
    Notification.requestPermission();
  }, []);

  const router = createBrowserRouter([
    {
      path: "/",
      element: (
        <Refreshing interval={REFRESH_INTERVAL_MS}>
          <Home />
        </Refreshing>
      ),
    },
    { path: "/register", element: <Register /> },
  ]);
  return (
    <div className="App">
      <PageNavbar />
      <ToastContainer />
      <RouterProvider router={router} />
    </div>
  );
}

export default App;
