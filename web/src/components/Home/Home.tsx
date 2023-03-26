import axios from "axios";
import { useEffect, useState } from "react";
import { Measurement, MeasurementType } from "../../models/measurement";
import { IMeasurementResponse } from "../../models/measurementResponse";
import { Refreshing } from "../common/Refreshing";
import { CurrentMeasurementTable } from "./CurrentMeasurements/CurrentMeasurementTable";
import { MeasurementTime } from "./CurrentMeasurements/MeasurementTime";
import { MeasurementHistory } from "./HistoryMeasurements/MeasurementHistory";

const REFRESH_INTERVAL_MS = 10 * 1000;
const VOC_INDEX_NOTIFICATION_THRESHOLD = 60;
const TEN_MINUTES = 1000 * 60 * 10;

export const Home: React.FC = () => {
  const checkVoc = (vocIndex?: number) => {
    if (
      vocIndex &&
      vocIndex > VOC_INDEX_NOTIFICATION_THRESHOLD &&
      lastNotificationTime > Date.now() - TEN_MINUTES
    ) {
      setLastNotificationTime(Date.now());
      new Notification("High VOC alert", {
        body: `We have detected VOC of ${vocIndex}!`,
        icon: "logo192.png",
      });
    }
  };
  const username = "Andy"; //TODO: load from JWT
  const [multiMeasurement, setMultiMeasurement] = useState<Measurement[]>([]);
  const [measurementTime, setMeasurementTime] = useState<Date>();
  const [lastNotificationTime, setLastNotificationTime] = useState<number>(
    Date.now()
  );
  useEffect(() => {
    axios.get("/api/measurement").then((response) => {
      const data: IMeasurementResponse = response.data as IMeasurementResponse;

      const measurements: Measurement[] = [
        { type: MeasurementType.Temperature, value: data.temperature },
        { type: MeasurementType.Humidity, value: data.humidity },
        { type: MeasurementType.Voc, value: data.voc_index },
      ];
      checkVoc(data.voc_index);
      setMeasurementTime(new Date(data.measurement_time));
      setMultiMeasurement(measurements);
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return (
    <div className="home_page main_page">
      <h2>Hi, {username}!</h2>
      <Refreshing interval={REFRESH_INTERVAL_MS}>
        <CurrentMeasurementTable measurements={multiMeasurement} />
        <MeasurementHistory />
        <MeasurementTime measurementTime={measurementTime} />
      </Refreshing>
    </div>
  );
};
