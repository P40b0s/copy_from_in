import React, { useEffect, useState } from "react";
import { IPacket, Task } from "../types/types";
import { spreadLine } from "../utils/utils";
import { TaskCard } from "./task-card";
import { settings } from "../services/tauri-service";
import { TaskModal } from "../components/new-task-modal";

import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/api/notification";

export const Tasks: React.FC = () => {
  const [tasks, setTasks] = useState<Task[]>([
    {
      copy_modifier: "copy_all",
      delete_after_copy: false,
      filters: { document_types: [], document_uids: [] },
      is_active: false,
      name: "dummyTask",
      source_dir: "/source",
      target_dir: "/target",
      timer: 0,
    },
  ]);
  const [isChanged, setChanged] = useState<boolean>(false);
  const [isModalOpen, setModalOpen] = useState<boolean>(false);

  useEffect(() => {
    const getEvents = async () => {
      const res = await settings.load_settings();
      console.log(res);
      if (typeof res !== "string" && res) setTasks(res);
    };

    getEvents();
  }, []);

  const saveHandler = async () => {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }
    const text = await settings.save_settings(tasks);
    if (text) {
      if (permissionGranted) sendNotification({ title: "Ошибка!", body: text });
    }
  };

  const onDelete = async (name: string) => {
    setChanged(true);
    const updetedTasks = tasks.filter((task) => task.name != name);
    setTasks(updetedTasks);
  };

  return (
    <>
      {isModalOpen && (
        <TaskModal
          onSaveTasks={setTasks}
          tasks={tasks}
          onClose={setModalOpen}
          toggleSave={setChanged}
        />
      )}
      {React.Children.toArray(
        tasks.map((task) => (
          <TaskCard
            copy_modifier={task.copy_modifier}
            delete_after_copy={task.delete_after_copy}
            filters={task.filters}
            is_active={task.is_active}
            name={task.name}
            onDelete={onDelete}
            source_dir={task.source_dir}
            target_dir={task.target_dir}
            timer={task.timer}
          />
        ))
      )}
      <div className="button" onClick={async () => await setModalOpen(true)}>
        Добавить задачу
      </div>
      {isChanged && (
        <button className="button" onClick={saveHandler}>
          Сохранить
        </button>
      )}
    </>
  );
};
