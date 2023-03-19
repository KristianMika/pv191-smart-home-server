export interface IMeasurementTime {
  measurementTime?: Date;
}

const getDisplayableTime = (dateTime?: Date): string =>
  dateTime?.toLocaleString() || "-";

export const MeasurementTime: React.FC<IMeasurementTime> = (props) => {
  return (
    <div className="home__current_measurement_time">
      <p className="home__current_measurement_time__content">
        Measurement time: {getDisplayableTime(props.measurementTime)}
      </p>
    </div>
  );
};
