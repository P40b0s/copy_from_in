import React from "react";
import { IPacket } from "../types/types";
import { spreadLine } from "../utils/utils";

type PropsType = {
  result: IPacket[];
};

export const Packets: React.FC<PropsType> = ({ result }) => {
  return React.Children.toArray(
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
      .reverse()
  );
};
