import React from "react";
import { IPacket } from "../types/types";

type PropsType = {
  result: IPacket[];
};

export const Errors: React.FC<PropsType> = ({ result }) => {
  return React.Children.toArray(
    result
      .filter((el) => el.error)
      .map((item, index) => (
        <p>
          {`[ ${index} ]`}
          {item.error ? item.error : ""}
        </p>
      ))
      .reverse()
  );
};
