import axios from "axios";
import { useEffect, useState } from "react";
import { Measurement, MeasurementType } from "../../models/measurement";
import { IMeasurementResponse } from "../../models/measurementResponse";
import { CurrentMeasurementTable } from "./CurrentMeasurementTable";

export const Home: React.FC = () => {
  const username = "Andy"; //TODO: load from JWT
  const [multiMeasurement, setMultiMeasurement] = useState<Measurement[]>([]);
  const [measurementTime, setMeasurementTime] = useState<Date>();
  useEffect(() => {
    axios.get("/api/measurement").then((response) => {
      const data: IMeasurementResponse = response.data as IMeasurementResponse;

      const measurements: Measurement[] = [
        { type: MeasurementType.Temperature, value: data.temperature },
        { type: MeasurementType.Humidity, value: data.humidity },
        { type: MeasurementType.Voc, value: data.voc_index },
      ];
      setMeasurementTime(new Date(data.measurement_time));
      setMultiMeasurement(measurements);
    });
  }, []);
  return (
    <div className="home main_page">
      <h2>Hi, {username}!</h2>

      <CurrentMeasurementTable measurements={multiMeasurement} />
      <div className="home__current_measurement_time">
        <p className="home__current_measurement_time__content">
          Measurement time: {measurementTime?.toLocaleString() || "-"}
        </p>
      </div>
    </div>
  );
};
