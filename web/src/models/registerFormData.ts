export interface RegisterFormData {
  firstName: string;
  login: string;
  password: string;
  passwordAgain: string;
}
export const emptyFormData: RegisterFormData = {
  firstName: "",
  login: "",
  password: "",
  passwordAgain: "",
};
