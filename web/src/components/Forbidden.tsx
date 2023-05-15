/**
 * A page that is displayed in case of a 403 return code
 */
export const Forbidden: React.FC = () => {
  return (
    <div className="home_page main_page">
      <h2 className="main_page__main_heading">
        You don't have access to the page you are trying to access.
      </h2>
      <h4 className="main_page__sub_heading">
        Try creating an account from your local network
      </h4>
    </div>
  );
};
