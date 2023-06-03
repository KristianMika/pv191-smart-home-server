import axios, { AxiosResponse } from "axios";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { toast } from "react-toastify";
import { isJwtSet } from "../../auth";
import { IUser } from "../../models/IUser";
import { Measurement, MeasurementType } from "../../models/measurement";
import { IMeasurementResponse } from "../../models/measurementResponse";
import { CurrentMeasurementTable } from "./CurrentMeasurements/CurrentMeasurementTable";
import { MeasurementTime } from "./CurrentMeasurements/MeasurementTime";
import { MeasurementHistory } from "./HistoryMeasurements/MeasurementHistory";

const VOC_INDEX_NOTIFICATION_THRESHOLD = 60;
const TEN_MINUTES = 1000 * 60 * 10;

const generateHIghVocNotification = (vocIndex: number) => {
  new Notification("High VOC alert", {
    body: `We have detected VOC of ${vocIndex}!`,
    icon: "voc-notification-icon.png",
  });
};

export const Home: React.FC = () => {
  const navigate = useNavigate();
  useEffect(() => {
    if (!isJwtSet()) {
      toast.warn("Your session has expired. Please, log in.");
      navigate("/login");
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const checkVoc = (vocIndex?: number) => {
    const lastNotificationTime = parseInt(
      localStorage.getItem("lastNotificationTime") || ""
    );
    if (
      vocIndex &&
      vocIndex > VOC_INDEX_NOTIFICATION_THRESHOLD &&
      (!lastNotificationTime || lastNotificationTime < Date.now() - TEN_MINUTES)
    ) {
      localStorage.setItem("lastNotificationTime", Date.now().toString());
      generateHIghVocNotification(vocIndex);
    }
  };
  const [user, setUser] = useState<IUser>({ first_name: "" });
  const [multiMeasurement, setMultiMeasurement] = useState<Measurement[]>([]);
  const [measurementTime, setMeasurementTime] = useState<Date>();

  const getMeasurements = async () => {
    let response;
    try {
      response = await axios.get("/api/measurement");
    } catch (e) {
      throw new Error("Couldn't fetch measurements from the server");
    }

    const data: IMeasurementResponse = response.data as IMeasurementResponse;

    const measurements: Measurement[] = [
      { type: MeasurementType.Temperature, value: data.temperature },
      { type: MeasurementType.Humidity, value: data.humidity },
      { type: MeasurementType.Voc, value: data.voc_index },
    ];
    checkVoc(data.voc_index);
    setMeasurementTime(new Date(data.measurement_time));
    setMultiMeasurement(measurements);
  };

  // utilize the toaster for popping error messages if the promise fails
  useEffect(() => {
    toast.promise(getMeasurements, {
      error: {
        render({ data }) {
          return `${data || "An error occurred"}`;
        },
      },
    });

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const getUser = async () => {
    let response;
    try {
      response = await axios.get("/api/user");
    } catch (e) {
      console.log("Couldn't fetch user");
      return;
    }
    const data: IUser = (response as AxiosResponse).data as IUser;
    setUser(data);
  };
  useEffect(() => {
    getUser();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="home_page main_page">
      <h2 className="main_page__main_heading--left">
        Welcome, {user.first_name}!
      </h2>
      <CurrentMeasurementTable measurements={multiMeasurement} />
      <MeasurementHistory />
      <MeasurementTime measurementTime={measurementTime} />
    </div>
  );
};
