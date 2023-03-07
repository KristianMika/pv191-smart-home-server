export interface IMeasurementResponse {
  measurement: IMeasurementData;
  measurement_time: Date;
}

export interface IMeasurementData {
  temperature: number;
  humidity: number;
  voc_index: number;
}
