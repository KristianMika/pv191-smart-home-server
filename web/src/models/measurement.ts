import thermometer from "../assets/thermometer.svg";
import humidity from "../assets/humidity.svg";
import lungs from "../assets/lungs.svg";

export interface Measurement {
  value: number;
  type: MeasurementType;
}

export enum MeasurementUnit {
  Celsius = "°C",
  Percentage = "%",
  VocIndex = "(VOC Index)",
}

export enum MeasurementType {
  Temperature = "Temperature",
  Humidity = "Humidity",
  Voc = "VOC Index",
}

export const getMeasurementUnit = (
  measurementType: MeasurementType
): MeasurementUnit => {
  switch (measurementType) {
    case MeasurementType.Temperature:
      return MeasurementUnit.Celsius;
    case MeasurementType.Humidity:
      return MeasurementUnit.Percentage;
    case MeasurementType.Voc:
      return MeasurementUnit.VocIndex;
  }
};

export const getMeasurementImage = (measurementType: MeasurementType) => {
  switch (measurementType) {
    case MeasurementType.Temperature:
      return thermometer;
    case MeasurementType.Humidity:
      return humidity;
    case MeasurementType.Voc:
      return lungs;
  }
};
