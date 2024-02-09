import React from "react";
import { Task } from "../types/types";
import "../App.scss";

interface IProps {
  onClose: (value: boolean) => void;
}

export const TaskModal: React.FC<IProps> = ({ onClose }) => {
  return (
    <div className="modal-wrapper">
      <div className="modal-view">
        <div className="modal-row">
          <label>Название </label>
          <input type="text" />
        </div>
        <div className="modal-row">
          <label>Путь к дирректории источника </label>
          <input type="text" />
        </div>
        <div className="modal-row">
          <label>Путь к дирректории назначения </label>
          <input type="text" />
        </div>
        <div className="modal-row">
          <label>Таймер </label>
          <input type="text" />
        </div>
        <div className="modal-row">
          <label>Удалить после копирования </label>
          <input type="checkbox" />
        </div>
        <div className="modal-row">
          <label>Модификатор </label>
          <select>
            <option value="copy_all">copy_all</option>
            <option value="copy_only">copy_only</option>
            <option value="copy_except">copy_except</option>
          </select>
        </div>
        <div className="modal-row">
          <label>Активна </label>
          <input type="checkbox" />
        </div>
        <div className="modal-row">
          <label>Фильтры(хз) </label>
          <input type="checkbox" />
        </div>
        <div className="modal-row">
          <button onClick={() => onClose(false)} className="button">
            Добавить
          </button>

          <button onClick={() => onClose(false)} className="button">
            Закрыть
          </button>
        </div>
      </div>
    </div>
  );
};
