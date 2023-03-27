import axios from "axios";
import { useEffect, useState } from "react";
import { MeasurementAttribute } from "../../../models/measurement";
import { IMeasurementResponse } from "../../../models/measurementResponse";
import { MeasurementChart } from "./MeasurementChart";

const TEMPERATURE_STROKE_COLOR = "#FFA300";
const HUMIDITY_STROKE_COLOR = "#42bdff";
const VOC_INDEX_STROKE_COLOR = "#34a12e";
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
        strokeColor={TEMPERATURE_STROKE_COLOR}
      />
      <MeasurementChart
        measurements={pastMeasurements}
        measurementKey={MeasurementAttribute.Humidity}
        strokeColor={HUMIDITY_STROKE_COLOR}
      />
      <MeasurementChart
        measurements={pastMeasurements}
        measurementKey={MeasurementAttribute.Voc}
        strokeColor={VOC_INDEX_STROKE_COLOR}
      />
    </div>
  );
};
