import { CurrentMeasurementTable } from "./CurrentMeasurementTable";

export const Home: React.FC = () => {
  const username = "Andy"; //TODO: load from JWT
  return (
    <div className="home main_page">
      <h2>Hi, {username}!</h2>

      <CurrentMeasurementTable />
    </div>
  );
};
