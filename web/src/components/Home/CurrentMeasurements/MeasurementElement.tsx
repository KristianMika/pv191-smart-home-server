import { getMeasurementUnit, Measurement } from "../../../models/measurement";

export interface IMeasurementElement {
  imageUrl: string;
  measurement: Measurement;
}

// Returns the value is is not undefined and not null, returns a dash otherwise
// note: if the value is 0, it is not considered undefined
const valueOrDefault = (value?: number) => {
  if (value === undefined || value === null) {
    return "-";
  }
  return value;
};

export const MeasurementElement: React.FC<IMeasurementElement> = (
  props: IMeasurementElement
) => {
  return (
    <div className="current_measurement_table__measurement_element">
      <img
        className="measurement_element__image"
        src={props.imageUrl}
        alt={props.measurement.type}
      />
      <div className="measurement_element__label">
        <span className="measurement_element__value">
          {valueOrDefault(props.measurement.value)}
        </span>{" "}
        <span className="measurement_element__unit">
          {getMeasurementUnit(props.measurement.type)}
        </span>
        <br />
        <span className="measurement_element__name">
          {props.measurement.type}
        </span>
      </div>
    </div>
  );
};
