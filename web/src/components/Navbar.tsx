import axios from "axios";
import Container from "react-bootstrap/Container";
import Nav from "react-bootstrap/Nav";
import Navbar from "react-bootstrap/Navbar";
import { useNavigate } from "react-router-dom";
import { toast } from "react-toastify";
import Button from "react-bootstrap/Button";
import { isJwtSet } from "../auth";

export const PageNavbar: React.FC = () => {
  const navigate = useNavigate();

  const logout = () => {
    const submitLogout = new Promise<void>((resolve, reject) =>
      axios
        .post("/logout")
        .then(() => {
          resolve();
          navigate("/login");
        })
        .catch((error) => {
          reject(error.response?.data?.message);
        })
    );
    toast.promise(submitLogout, {
      pending: "Logging out...",
      success: "Successfully logged out",
      error: {
        render({ data }) {
          return `${data || "Logout failed"}`;
        },
      },
    });
  };

  const getAuthButtons = () => {
    if (isJwtSet()) {
      return (
        <Button
          variant="link"
          className="navbar__button"
          onClick={() => {
            logout();
          }}
        >
          Log out
        </Button>
      );
    } else {
      return (
        <>
          <Nav.Link href="/register">Register</Nav.Link>
          <Nav.Link href="/login">Login</Nav.Link>
        </>
      );
    }
  };

  return (
    <Navbar bg="light" expand="lg">
      <Container>
        <Navbar.Brand href="/">Home Server</Navbar.Brand>
        <Navbar.Toggle aria-controls="basic-navbar-nav" />
        <Navbar.Collapse id="basic-navbar-nav">
          <Nav className="me-auto">{getAuthButtons()}</Nav>
        </Navbar.Collapse>
      </Container>
    </Navbar>
  );
};
