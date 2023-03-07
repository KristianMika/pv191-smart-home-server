import { useEffect, useState } from "react";
import {
  getMeasurementImage,
  getMeasurementType,
  Measurement,
} from "../../models/measurement";
import {
  IMeasurementData,
  IMeasurementResponse,
} from "../../models/measurementResponse";
import { MeasurementElement } from "./MeasurementElement";
import axios from "axios";

const tranform_response = (response: IMeasurementResponse) => {
  const measurements: Measurement[] = [];
  for (var attribute in response.measurement) {
    if (response.measurement.hasOwnProperty(attribute)) {
      const measurement: Measurement = {
        value: response.measurement[attribute as keyof IMeasurementData],
        type: getMeasurementType(attribute),
      };
      measurements.push(measurement);
    }
  }

  console.log(measurements);
  return measurements;
};
// TODO: time
export const CurrentMeasurementTable: React.FC = () => {
  const [multiMeasurement, setMultiMeasurement] = useState<Measurement[]>([]);
  useEffect(() => {
    axios
      // TODO: change to /api/measurement when done with development
      .get("http://localhost:8080/api/measurement")
      .then((response) => response.data)
      .then(tranform_response)
      .then(setMultiMeasurement);
  }, []);

  return (
    <div className="home__current_measurement_table">
      {multiMeasurement.map((measurement) => (
        <MeasurementElement
          key={measurement.type}
          imageUrl={getMeasurementImage(measurement.type)}
          measurement={measurement}
        />
      ))}
    </div>
  );
};
