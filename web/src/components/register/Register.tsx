import axios from "axios";
import React, { useState } from "react";
import { toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import { emptyFormData, RegisterFormData } from "../../models/registerFormData";

const MIN_FIRST_NAME_LENGTH = 1;
const MAX_FIRST_NAME_LENGTH = 20;
const MIN_LOGIN_LENGTH = 4;
const MAX_LOGIN_LENGTH = 20;
const MIN_PASSWORD_LENGTH = 8;
const MAX_PASSWORD_LENGTH = 100;
export const Register: React.FC = () => {
  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    if (formData.password !== formData.passwordAgain) {
      toast.warn("Passwords don't match");
      return;
    }
    const submitRegistration = new Promise<void>((resolve, reject) =>
      axios
        .post("/register", {
          first_name: formData.firstName,
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
      pending: "Submitting registration request",
      success: "Registration successful",
      error: {
        render({ data }) {
          return `${data || "Registration failed"}`;
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
  const [formData, setFormData] = useState<RegisterFormData>(emptyFormData);
  return (
    <div className="register_page main_page">
      <div className="register_form__wrapper">
        <form className="register_page__form" onSubmit={handleSubmit}>
          <input
            placeholder="First Name"
            type="text"
            name="firstName"
            value={formData?.firstName}
            onChange={handleChange}
            className="form__input"
            minLength={MIN_FIRST_NAME_LENGTH}
            maxLength={MAX_FIRST_NAME_LENGTH}
            required
          />

          <input
            placeholder="Login"
            type="text"
            name="login"
            value={formData?.login}
            onChange={handleChange}
            className="form__input"
            minLength={MIN_LOGIN_LENGTH}
            maxLength={MAX_LOGIN_LENGTH}
            required
          />

          <input
            placeholder="Password"
            type="password"
            name="password"
            value={formData?.password}
            onChange={handleChange}
            className="form__input"
            minLength={MIN_PASSWORD_LENGTH}
            maxLength={MAX_PASSWORD_LENGTH}
            required
          />
          <input
            placeholder="Password again"
            type="password"
            name="passwordAgain"
            value={formData?.passwordAgain}
            onChange={handleChange}
            className="form__input"
            minLength={MIN_PASSWORD_LENGTH}
            maxLength={MAX_PASSWORD_LENGTH}
            required
          />
          <input className="submit_button" type="submit" value="Submit" />
        </form>
      </div>
    </div>
  );
};
