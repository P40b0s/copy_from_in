import "./App.scss";
import React, { useCallback, useEffect, useMemo, useState } from "react";
import { IDocument, IPacket, Task } from "./types/types";
import { TauriEvents, settings } from "./services/tauri-service";
import { mockPackets } from "./services/api";
import { TaskCard } from "./components/task-card";
import { Packets } from "./components/packets";
import { Errors } from "./components/errors";
import { Tasks } from "./components/tasks";

function App() {
  const [selectedId, setSelectedId] = useState<number>(1);
  const [errors, setErrors] = useState<number>(0);
  const [documents, setDocuments] = useState<number>(0);

  const [result, setResult] = useState<IPacket[]>([]);
  const [isChanged, setChanged] = useState<boolean>(false);

  console.log("!!!AppRender");
  useMemo(() => {
    TauriEvents.new_document_event((event) => {
      // console.log(`before ${result} | ${JSON.stringify(result.values())}`);
      console.log("!!!event");

      // if (event.payload.document) setDocuments(documents + 1);
      // if (event.payload.error) setErrors(errors + 1);
      if (event.payload && result.values())
        setResult((oldArray) => [event.payload, ...oldArray]);
    });
  }, []);

  return (
    <>
      <div className="tabs">
        <div className="tabs__row">
          <div
            className={`tab ${selectedId == 1 ? "selected" : ""}`}
            onClick={() => setSelectedId(1)}
          >
            Пакеты{`${documents > 0 ? ` (${documents})` : ""}`}
          </div>
          <div
            className={`tab ${selectedId == 2 ? "selected" : ""}`}
            onClick={() => setSelectedId(2)}
          >
            Ошибки{`${errors > 0 ? ` (${errors})` : ""}`}
          </div>
          <div
            className={`tab ${selectedId == 3 ? "selected" : ""}`}
            onClick={() => setSelectedId(3)}
          >
            Сервис
          </div>
          <div
            className={`tab ${selectedId == 4 ? "selected" : ""}`}
            onClick={() => setSelectedId(4)}
          >
            Настройки
          </div>
        </div>
      </div>
      {selectedId < 3 && (
        <div className="textarea">
          {selectedId == 2 && <Errors result={result} />}
          {selectedId == 1 && <Packets result={result} />}
        </div>
      )}
      {selectedId == 3 && (
        <div className="service-page">
          <div className="button">Очистка папок</div>
          <div className="button">Очистка задач</div>
        </div>
      )}
      {selectedId == 4 && (
        <div className="settings-page">
          <Tasks />
        </div>
      )}
    </>
  );
}

export default App;
