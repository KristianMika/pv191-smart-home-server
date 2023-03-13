import { getMeasurementImage, Measurement } from "../../models/measurement";
import { MeasurementElement } from "./MeasurementElement";

export interface ICurrentMeasurementTable {
  measurements: Measurement[];
}
export const CurrentMeasurementTable: React.FC<ICurrentMeasurementTable> = (
  props
) => {
  return (
    <div className="home__current_measurement_table">
      {props.measurements.map((measurement) => (
        <MeasurementElement
          key={measurement.type}
          imageUrl={getMeasurementImage(measurement.type)}
          measurement={measurement}
        />
      ))}
    </div>
  );
};
