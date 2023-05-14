import Cookies from "js-cookie";

/**
 * Checks if the dummy non-HttpOnly cookie that indicates the presence
 * of a real httpOnly access token is present
 *
 * The reason behind this is that we can't access httpOnly cookies so the backend
 * sets this dummy non-httpOnly cookie with the same expiration
 * on the user's authentication
 *
 * @returns true if the dummy authentication cookie is set, false otherwise
 */
export const isJwtSet = (): boolean => {
  return Cookies.get("jwt_set") !== undefined;
};
