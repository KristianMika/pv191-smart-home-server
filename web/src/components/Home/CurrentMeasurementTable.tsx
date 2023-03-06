import { useEffect, useState } from "react";
import {
  getMeasurementImage,
  Measurement,
  MeasurementType,
} from "../../models/measurement";
import { MeasurementElement } from "./MeasurementElement";

export const CurrentMeasurementTable: React.FC = () => {
  const [multiMeasurement, setMultiMeasurement] = useState<Measurement[]>([]);
  useEffect(() => {
    const mockedMultiMeasurement = [
      { type: MeasurementType.Humidity, value: 55 },
      { type: MeasurementType.Temperature, value: 21.5 },
      { type: MeasurementType.Voc, value: 127 },
    ];
    setMultiMeasurement(mockedMultiMeasurement);
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
