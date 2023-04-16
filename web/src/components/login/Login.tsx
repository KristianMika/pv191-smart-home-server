import axios from "axios";
import { useState } from "react";
import { toast } from "react-toastify";
import { emptyFormData, ILoginFormData } from "../../models/loginFormData";

export const Login: React.FC = () => {
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
      <div className="login_form__wrapper">
        <form className="login_page__form" onSubmit={handleSubmit}>
          <input
            placeholder="Login"
            type="text"
            name="login"
            value={formData?.login}
            onChange={handleChange}
            required
          />

          <input
            placeholder="Password"
            type="password"
            name="password"
            value={formData?.password}
            onChange={handleChange}
            required
          />

          <input type="submit" value="Submit" />
        </form>
      </div>
    </div>
  );
};
