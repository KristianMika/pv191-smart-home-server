import { createBrowserRouter, RouterProvider } from "react-router-dom";
import { ToastContainer } from "react-toastify";
import "./App.css";
import { Home } from "./components/Home/Home";
import { PageNavbar } from "./components/Navbar";
import { Register } from "./components/register/Register";

function App() {
  Notification.requestPermission(); //TODO: useEffect
  const router = createBrowserRouter([
    {
      path: "/",
      element: <Home />,
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
