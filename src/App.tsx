import "./App.scss";
import React, { useEffect, useState } from "react";
import { IDocument, IPacket } from "./types/types";
import { TauriEvents } from "./services/tauri-service";
import { mockPackets } from "./services/api";
import { TaskModal } from "./components/new-task-modal";

function App() {
  const [selectedId, setSelectedId] = useState<number>(1);
  const [isModalOpen, setModalOpen] = useState<boolean>(false);
  const [errors, setErrors] = useState<number>(0);
  const [documents, setDocuments] = useState<number>(0);

  const [result, setResult] = useState<IPacket[]>([]);

  const spreadLine = (doc: IDocument) => {
    let line = "";
    if (doc) {
      if (doc.parseTime) line += " " + doc.parseTime;
      if (doc.name) line += " " + doc.name;
      if (doc.organization) line += " " + doc.organization;
      if (doc.docType) line += " " + doc.docType;
      if (doc.number) line += " " + doc.number;
      if (doc.signDate) line += " " + doc.signDate;
    }
    return line;
  };

  useEffect(() => {
    let errorsSum = 0;
    let documentsSum = 0;
    const resultArray: Array<IPacket> = [];
    const getEvents = async () => {
      const res = await mockPackets();
      setDocuments(documentsSum);
      setErrors(errorsSum);
      // setResult(resultArray);
      setResult(res);
    };

    // TauriEvents.new_document_event((event) => {
    //   if (event.payload.document) documentsSum += 1;
    //   if (event.payload.error) errorsSum += 1;
    //   resultArray.push(event.payload);
    // });
    getEvents();
  }, []);

  return (
    <>
      {isModalOpen && <TaskModal onClose={setModalOpen} />}
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
          {selectedId == 2 &&
            React.Children.toArray(
              result
                .filter((el) => el.error)
                .map((item, index) => (
                  <p>
                    {`[ ${index} ]`}
                    {item.error ? item.error : ""}
                  </p>
                ))
            )}
          {selectedId == 1 &&
            React.Children.toArray(
              result
                .filter((el) => el.document)
                .map((item, index) => (
                  <p>
                    {`[ ${index} ]`}
                    {item.document
                      ? spreadLine(item.document)
                      : " Сломанный пакет. См. раздел Ошибки"}
                  </p>
                ))
            )}
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
          <div className="button" onClick={() => setModalOpen(true)}>
            Добавить задачу
          </div>
        </div>
      )}
    </>
  );
}

export default App;
