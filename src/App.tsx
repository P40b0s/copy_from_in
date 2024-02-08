import "./App.scss";
import { useState } from "react";

function App() {
  const [selectedId, setSelectedId] = useState<number>(1);
  return (
    <>
      <div className="tabs">
        <div className="tabs__row">
          <div
            className={`tab ${selectedId == 1 ? "selected" : ""}`}
            onClick={() => setSelectedId(1)}
          >
            Статус
          </div>
          <div
            className={`tab ${selectedId == 2 ? "selected" : ""}`}
            onClick={() => setSelectedId(2)}
          >
            Ошибки
          </div>
          <div
            className={`tab ${selectedId == 3 ? "selected" : ""}`}
            onClick={() => setSelectedId(3)}
          >
            Сервер
          </div>
        </div>
      </div>
      <div className="textarea">
        <p>It was a dark and stormy night...1</p>
        <p>It was a dark and stormy night...2</p>

        <p>It was a dark and stormy night...3</p>
      </div>
    </>
  );
}

export default App;
