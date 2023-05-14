import axios from "axios";
import { useEffect, useState } from "react";
import { toast } from "react-toastify";
import { emptyFormData, ILoginFormData } from "../../models/loginFormData";
import { useNavigate } from "react-router-dom";
import { isJwtSet } from "../../auth";

export const Login: React.FC = () => {
  const navigate = useNavigate();

  useEffect(() => {
    if (isJwtSet()) {
      navigate("/");
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const [formData, setFormData] = useState<ILoginFormData>(emptyFormData);
  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    const submitRegistration = new Promise<void>((resolve, reject) =>
      axios
        .post("/login", {
          login: formData.login,
          password: formData.password,
        })
        .then(() => {
          setFormData(emptyFormData);
          resolve();
          navigate("/");
        })
        .catch((error) => {
          reject(error.response?.data?.message);
        })
    );
    toast.promise(submitRegistration, {
      pending: "Loging in...",
      success: "Login was successful",
      error: {
        render({ data }) {
          return `${data || "Login failed"}`;
        },
      },
    });
  };

  const handleChange = (event: React.FormEvent) => {
    const name = (event.target as HTMLTextAreaElement).name;
    const value = (event.target as HTMLTextAreaElement).value;
    setFormData((prev) => {
      return { ...prev, [name]: value };
    });
  };

  return (
    <div className="login_page main_page">
      <div className="login_form__wrapper form__wrapper">
        <form className="login_page__form page__form" onSubmit={handleSubmit}>
          <input
            className="form__input"
            placeholder="Login"
            type="text"
            name="login"
            value={formData?.login}
            onChange={handleChange}
            required
          />

          <input
            className="form__input"
            placeholder="Password"
            type="password"
            name="password"
            value={formData?.password}
            onChange={handleChange}
            required
          />

          <input className="submit_button" type="submit" value="Submit" />
        </form>
      </div>
    </div>
  );
};
