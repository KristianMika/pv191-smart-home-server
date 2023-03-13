import { useEffect, useState } from "react";

export interface IRefreshing {
  interval: number; // in ms
  children: React.ReactNode;
}

export const Refreshing: React.FC<IRefreshing> = (props) => {
  const [time, setTime] = useState(Date.now());

  useEffect(() => {
    const interval = setInterval(() => setTime(Date.now()), props.interval);
    return () => {
      clearInterval(interval);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return <div key={time}>{props.children}</div>;
};
