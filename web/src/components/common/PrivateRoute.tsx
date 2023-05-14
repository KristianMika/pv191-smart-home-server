import { Navigate } from "react-router-dom";
import { isJwtSet } from "../../auth";

interface IPrivateRouteProps {
  children: React.ReactElement;
}
/**
 * Protects the route by redirecting to the login page in case
 * the user has not been authenticated
 * Note: Doesn't serve as a security measure
 */
export const PrivateRoute: React.FC<IPrivateRouteProps> = (props) => {
  return isJwtSet() ? <>{props.children}</> : <Navigate to="/login" replace />;
};
