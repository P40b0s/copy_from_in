import React, { useState } from "react";
import { Task } from "../types/types";
import "../App.scss";

interface IProps {
  onClose: (value: boolean) => void;
  onSaveTasks: (value: Task[]) => void;
  tasks: Task[];
  toggleSave: (value: boolean) => void;
}

export const TaskModal: React.FC<IProps> = ({
  onClose,
  onSaveTasks,
  tasks,
  toggleSave,
}) => {
  const [addFilter, setFilter] = useState<boolean>(false);
  const [filters, setFilters] = useState<
    Array<{ type: "document_types" | "document_uids"; text: string }>
  >([]);
  const [inputText, setInputText] = useState<string>("");
  const [filterType, setFilterType] = useState<
    "document_types" | "document_uids"
  >("document_types");
  const [inputName, setInputName] = useState<string>("");
  const [pathToSource, setPathToSource] = useState<string>("");
  const [pathToDist, setPathToDist] = useState<string>("");
  const [timer, setTimer] = useState<number>(3000);
  const [shouldDelete, setShouldDelete] = useState<boolean>(false);
  const [shouldActive, setShouldActive] = useState<boolean>(false);

  const [modifier, setModifier] = useState<
    "copy_all" | "copy_only" | "copy_except"
  >("copy_all");

  return (
    <div className="modal-wrapper">
      <div className="modal-view">
        <div className="modal-row">
          <label>Название </label>
          <input
            type="text"
            value={inputName}
            onChange={(e) => setInputName(e.target.value)}
          />
        </div>
        <div className="modal-row">
          <label>Путь к дирректории источника </label>
          <input
            type="text"
            value={pathToSource}
            onChange={(e) => setPathToSource(e.target.value)}
          />
        </div>
        <div className="modal-row">
          <label>Путь к дирректории назначения </label>
          <input
            type="text"
            value={pathToDist}
            onChange={(e) => setPathToDist(e.target.value)}
          />
        </div>
        <div className="modal-row">
          <label>Таймер </label>
          <input
            type="text"
            value={timer}
            onChange={(e) => setTimer(Number(e.target.value))}
          />
        </div>
        <div className="modal-row">
          <label>Удалить после копирования </label>
          <input
            type="checkbox"
            checked={shouldDelete}
            onChange={() => setShouldDelete((prev) => !prev)}
          />
        </div>
        <div className="modal-row">
          <label>Модификатор </label>
          <select value={modifier}>
            <option value="copy_all" onSelect={() => setModifier("copy_all")}>
              copy_all
            </option>
            <option value="copy_only" onSelect={() => setModifier("copy_only")}>
              copy_only
            </option>
            <option
              value="copy_except"
              onSelect={() => setModifier("copy_except")}
            >
              copy_except
            </option>
          </select>
        </div>
        <div className="modal-row">
          <label>Активна </label>
          <input
            type="checkbox"
            checked={shouldActive}
            onChange={() => setShouldActive((prev) => !prev)}
          />
        </div>
        <div className="modal-row"></div>
        <div
          style={{
            borderWidth: 2,
            borderStyle: "solid",
            borderRadius: 4,
            padding: 4,
          }}
        >
          {!addFilter && (
            <div className="modal-row">
              <div className="modal-column">
                <label>Фильтры:</label>
                {React.Children.toArray(
                  filters.map((el) => (
                    <label>
                      {el.type} {el.text}
                    </label>
                  ))
                )}
              </div>
              <div className="modal-column">
                <button className="button" onClick={() => setFilter(true)}>
                  Добавить фильтр
                </button>
              </div>
            </div>
          )}
          {addFilter && (
            <div className="modal-column">
              <div className="modal-row" style={{ gap: 5 }}>
                <select defaultValue={filterType}>
                  <option
                    value="document_types"
                    onSelect={() => setFilterType("document_types")}
                  >
                    Тип пакетов
                  </option>
                  <option
                    value="document_uids"
                    onSelect={() => setFilterType("document_uids")}
                  >
                    Идентификатор отправителя
                  </option>
                </select>
                <input
                  type="text"
                  value={inputText}
                  onChange={(e) => setInputText(e.target.value)}
                />
              </div>
              <div className="modal-row">
                <button
                  className="button"
                  onClick={() => {
                    setFilters((prev) => [
                      ...prev,
                      { text: inputText, type: filterType },
                    ]);
                    setFilter(false);
                    setInputText("");
                  }}
                >
                  Добавить
                </button>
                <button className="button" onClick={() => setFilter(false)}>
                  Отмена
                </button>
              </div>
            </div>
          )}
        </div>
        <br />
        <div className="modal-row">
          <button
            onClick={() => {
              onSaveTasks([
                ...tasks,
                {
                  copy_modifier: modifier,
                  delete_after_copy: shouldDelete,
                  is_active: shouldActive,
                  name: inputName,
                  source_dir: pathToSource,
                  target_dir: pathToDist,
                  timer: timer,
                  filters: {
                    document_types: filters
                      .filter((item) => item.type === "document_types")
                      .map((it) => it.text),
                    document_uids: filters
                      .filter((item) => item.type === "document_uids")
                      .map((it) => it.text),
                  },
                },
              ]);
              toggleSave(true);
              onClose(false);
            }}
            className="button"
          >
            Добавить задачу
          </button>

          <button onClick={() => onClose(false)} className="button">
            Закрыть
          </button>
        </div>
      </div>
    </div>
  );
};
