import React from "react";
import { Task } from "../types/types";

export const TaskCard: React.FC<
  Task & { onDelete: (value: string) => void }
> = ({
  name,
  source_dir,
  target_dir,
  timer,
  delete_after_copy,
  copy_modifier,
  is_active,
  filters,
  onDelete,
}) => {
  return (
    <div className="task-card">
      <div className="list">
        <ul>
          <li>
            <span>{name}</span> <div className="tooltiptext">Название</div>
          </li>
          <li>
            <span>{source_dir}</span>
            <div className="tooltiptext">Путь к дирректории источника</div>
          </li>
          <li>
            <span>{target_dir}</span>
            <div className="tooltiptext">Путь к дирректории назначения</div>
          </li>
          <li>
            <span>{timer}</span>
            <div className="tooltiptext">Таймер</div>
          </li>
          <li>
            <span>{delete_after_copy ? "Delete" : "No-Delete"}</span>
            <div className="tooltiptext">Удалить после копирования</div>
          </li>
          <li>
            <span>{copy_modifier}</span>
            <div className="tooltiptext">Модификатор</div>
          </li>
          <li>
            <span>{is_active ? "Active" : "Inactive"}</span>
            <div className="tooltiptext">Активна</div>
          </li>
          <li>
            <span>
              фильтры: document_types:{" "}
              {filters.document_types.length > 0
                ? React.Children.toArray(filters.document_types.map((e) => e))
                : "Нет фильтров"}{" "}
              document_uids:{" "}
              {filters.document_uids.length > 0
                ? React.Children.toArray(filters.document_uids.map((e) => e))
                : "Нет фильтров"}
            </span>
            <div className="tooltiptext"></div>
          </li>
        </ul>
      </div>
      <button className="button" onClick={() => onDelete(name)}>
        Удалить
      </button>
    </div>
  );
};
