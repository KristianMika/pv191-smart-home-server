import axios from "axios";
import { useEffect, useState } from "react";
import { MeasurementAttribute } from "../../../models/measurement";
import { IMeasurementResponse } from "../../../models/measurementResponse";
import { MeasurementChart } from "./MeasurementChart";

export const MeasurementHistory: React.FC = () => {
  const [pastMeasurements, setPastMeasurements] = useState<
    IMeasurementResponse[]
  >([]);

  useEffect(() => {
    axios.get("/api/past_measurements").then((response) => {
      const data: IMeasurementResponse[] =
        response.data as IMeasurementResponse[];
      const measurements = data.map((measurement) => {
        return {
          ...measurement,
          measurement_time: new Date(measurement.measurement_time),
        };
      });
      console.log(measurements);
      setPastMeasurements(measurements);
    });
  }, []);
  return (
    <div className="home__dashboard_table">
      <MeasurementChart
        measurements={pastMeasurements}
        measurementKey={MeasurementAttribute.Temperature}
      />
      <MeasurementChart
        measurements={pastMeasurements}
        measurementKey={MeasurementAttribute.Humidity}
      />
      <MeasurementChart
        measurements={pastMeasurements}
        measurementKey={MeasurementAttribute.Voc}
      />
    </div>
  );
};
