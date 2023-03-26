import "./App.css";
import { Home } from "./components/Home/Home";

function App() {
  Notification.requestPermission();
  return (
    <div className="App">
      <Home />
    </div>
  );
}

export default App;
