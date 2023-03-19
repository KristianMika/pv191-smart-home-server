import React from "react";
import {
  XAxis,
  YAxis,
  CartesianGrid,
  ResponsiveContainer,
  ScatterChart,
  Scatter,
} from "recharts";
import {
  getMeasurementType,
  MeasurementAttribute,
} from "../../../models/measurement";
import { IMeasurementResponse } from "../../../models/measurementResponse";

export interface IMeasurementChart {
  measurements: IMeasurementResponse[];
  measurementKey: MeasurementAttribute;
}
export const MeasurementChart: React.FC<IMeasurementChart> = (props) => {
  const yAxisPadding = 0.5;
  const strokeColor = "#82ca9d";
  const strokeWidth = 3;
  return (
    <div className="dashboard_table__element">
      <h2 className="dashboard_table__element_header">
        {getMeasurementType(props.measurementKey)}
      </h2>
      <ResponsiveContainer width="100%" aspect={1.5}>
        <ScatterChart
          data={props.measurements.map((measurement) => {
            if (!measurement.measurement_time.toISOString()) {
            }
            return {
              ...measurement,
              measurement: measurement.measurement_time,
            };
          })}
        >
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis
            dy={5}
            tickCount={24}
            dataKey={MeasurementAttribute.MeasurementTime}
            scale="time"
            tickFormatter={(label: Date) => label.getHours().toString()}
          />
          <YAxis
            tickCount={5}
            domain={[
              (dataMin: number) => dataMin - yAxisPadding,
              (dataMax: number) => dataMax + yAxisPadding,
            ]}
          />
          <Scatter
            dataKey={props.measurementKey}
            stroke={strokeColor}
            strokeWidth={strokeWidth}
          />
        </ScatterChart>
      </ResponsiveContainer>
    </div>
  );
};
