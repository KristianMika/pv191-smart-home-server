import { render, screen } from "@testing-library/react";
import App from "./App";
import { MeasurementElement } from "./components/Home/MeasurementElement";
import { getMeasurementImage, MeasurementType } from "./models/measurement";

test("renders name of temperature measurement element", () => {
  render(
    <MeasurementElement
      imageUrl={getMeasurementImage(MeasurementType.Temperature)}
      measurement={{ value: 17.45, type: MeasurementType.Temperature }}
    />
  );
  const temperatureHeading = screen.getByText(/Temperature/i);
  expect(temperatureHeading).toBeInTheDocument();
});

test("renders value of temperature measurement element", () => {
  render(
    <MeasurementElement
      imageUrl={getMeasurementImage(MeasurementType.Temperature)}
      measurement={{ value: 17.45, type: MeasurementType.Temperature }}
    />
  );
  const temperatureValue = screen.getByText(/17.45/);
  expect(temperatureValue).toBeInTheDocument();
});
